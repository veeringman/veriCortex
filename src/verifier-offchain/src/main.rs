use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{fs, path::Path};
use hex::encode as hex_encode;
use dotenv::dotenv;
use std::env;
use log::{info, warn};

#[derive(Debug, Serialize, Deserialize)]
struct ProofJson {
    model_id: String,
    input_hash: String,  // hex string (0x...)
    output_hash: String  // hex string (0x...)
}

/// Compute deterministic recomputed hash: SHA256(model_id || ":" || input_hash)
fn compute_recomputed_output_hash(model_id: &str, input_hash: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(model_id.as_bytes());
    hasher.update(b":");
    hasher.update(input_hash.as_bytes());
    let out = hasher.finalize();
    format!("0x{}", hex_encode(out))
}

/// Compute a proofId equivalent to solidity:
/// keccak256(abi.encodePacked(modelId, inputHash, outputHash, submitter))
fn compute_proof_id(model_id: &str, input_hash: &str, output_hash: &str, submitter: &str) -> String {
    use ethers::utils::keccak256;
    // assemble bytes similar to abi.encodePacked
    let mut concat: Vec<u8> = vec![];
    concat.extend_from_slice(model_id.as_bytes());
    // remove 0x prefix and decode hex bytes
    let in_bytes = hex::decode(input_hash.trim_start_matches("0x")).unwrap_or_default();
    concat.extend_from_slice(&in_bytes);
    let out_bytes = hex::decode(output_hash.trim_start_matches("0x")).unwrap_or_default();
    concat.extend_from_slice(&out_bytes);
    // address bytes: remove 0x and decode (20 bytes expected)
    let addr_bytes = hex::decode(submitter.trim_start_matches("0x")).unwrap_or_default();
    concat.extend_from_slice(&addr_bytes);

    let digest = keccak256(concat);
    format!("0x{}", hex_encode(digest))
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    dotenv().ok();

    let proof_path = Path::new("src/verifier-offchain/tests/sample_proof.json");
    if !proof_path.exists() {
        // also accept tests at crate-root tests/
        let alt = Path::new("src/verifier-offchain/tests/sample_proof.json");
        if alt.exists() { }
    }

    let raw = fs::read_to_string(proof_path)
        .context("Failed to read sample_proof.json at src/verifier-offchain/tests/sample_proof.json")?;
    let proof: ProofJson = serde_json::from_str(&raw).context("Invalid proof JSON")?;

    info!("Loaded proof for model: {}", proof.model_id);
    let recomputed = compute_recomputed_output_hash(&proof.model_id, &proof.input_hash);
    info!("Local recomputed output hash: {}", recomputed);
    info!("Output hash from proof:       {}", proof.output_hash);

    if recomputed == proof.output_hash {
        info!("✅ Local recompute success: proof matches");
    } else {
        warn!("⚠️ Local recompute mismatch - proof may be invalid for demo");
    }

    // If RPC_URL & SUBMITTER_PRIVATE_KEY & VERIFIER_CONTRACT_ADDRESS present -> attempt submission
    let rpc_url = env::var("RPC_URL").ok();
    let submitter_pk = env::var("SUBMITTER_PRIVATE_KEY").ok();
    let contract_addr = env::var("VERIFIER_CONTRACT_ADDRESS").ok();

    let can_submit = rpc_url.is_some() && submitter_pk.is_some() && contract_addr.is_some();

    // compute proofId using a dummy submitter address when offline:
    let offline_submitter = "0x0000000000000000000000000000000000000000";
    let submitter_for_id = if can_submit {
        // if will submit, compute with wallet address later - placeholder for now
        offline_submitter
    } else {
        offline_submitter
    };

    let proof_id = compute_proof_id(
        &proof.model_id,
        &proof.input_hash,
        &proof.output_hash,
        submitter_for_id,
    );
    info!("Computed proofId (local): {}", proof_id);

    if !can_submit {
        info!("RPC_URL or SUBMITTER_PRIVATE_KEY or VERIFIER_CONTRACT_ADDRESS not set. Running in OFFLINE mode.");
        info!("To enable on-chain submission, set values in .env (copy .env.example -> .env).");
        return Ok(());
    }

    // If we reach here, attempt to submit to chain
    info!("RPC and credentials found. Preparing to submit to contract...");

    // perform on-chain submission using ethers
    use ethers::prelude::*;
    use std::sync::Arc;

    let rpc = rpc_url.unwrap();
    let pk = submitter_pk.unwrap();
    let contract = contract_addr.unwrap();
    let chain_id: u64 = env::var("CHAIN_ID").ok().and_then(|s| s.parse().ok()).unwrap_or(1337);

    // provider + wallet
    let provider = Provider::<Http>::try_from(rpc.clone())?.interval(std::time::Duration::from_millis(200u64));
    let wallet: LocalWallet = pk.parse::<LocalWallet>()?.with_chain_id(chain_id);
    let wallet_addr = wallet.address();
    let client = SignerMiddleware::new(provider, wallet);
    let client = Arc::new(client);

    // abigen the contract inline (same interface as demo)
    abigen!(
        VeriCortexVerifierContract,
        r#"[
            function submitProof(string modelId, bytes32 inputHash, bytes32 outputHash) public returns (bytes32)
            event ProofSubmitted(bytes32 indexed proofId, address indexed submitter, string modelId)
            event ProofVerified(bytes32 indexed proofId, bool success)
        ]"#
    );

    let contract_address: Address = contract.parse()?;
    let contract = VeriCortexVerifierContract::new(contract_address, client.clone());

    // convert input/output hex to H256
    let to_h256 = |h: &str| -> Result<H256, anyhow::Error> {
        let clean = h.trim_start_matches("0x");
        let bytes = hex::decode(clean)?;
        if bytes.len() > 32 { anyhow::bail!("hash too long"); }
        let mut arr = [0u8; 32];
        arr[32 - bytes.len()..].copy_from_slice(&bytes);
        Ok(H256::from(arr))
    };

    let input32 = to_h256(&proof.input_hash)?;
    let output32 = to_h256(&proof.output_hash)?;

    info!("Submitting transaction to contract: {}", contract_address);

    let pending_tx = contract
        .submit_proof(proof.model_id.clone(), input32, output32)
        .legacy()
        .send()
        .await
        .context("submit tx failed")?;

    info!("Submitted tx hash: {:?}", pending_tx.tx_hash());

    let receipt = pending_tx.await.context("tx await failed")?;
    info!("Tx mined in block: {:?}", receipt.block_number);

    // With wallet address known, recompute proofId properly
    let final_proof_id = compute_proof_id(
        &proof.model_id,
        &proof.input_hash,
        &proof.output_hash,
        &format!("0x{:x}", wallet_addr),
    );
    info!("Final computed proofId: {}", final_proof_id);

    info!("Done. Monitor ProofSubmitted / ProofVerified events on-chain.");

    Ok(())
}
