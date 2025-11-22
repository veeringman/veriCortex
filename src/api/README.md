VeriCortex Public API Server

A lightweight Rust (Axum) backend exposing public APIs to interact with the VeriCortexVerifier smart contract deployed on BlockDAG’s EVM-compatible network.

This API allows off-chain agents, dApps, platforms, and CLI clients to:
	•	Submit verification proofs
	•	Fetch proof metadata
	•	Check proof validity
	•	List proofs per submitter
	•	Retrieve global verification statistics

⸻

Project Structure

vericortex-api/
  src/
    main.rs
    routes.rs
    handlers/
      submit.rs
      get_proof.rs
      is_valid.rs
      list_by_submitter.rs
      stats.rs
    abi/
      VeriCortexVerifier.json
    contract.rs
    config.rs
  Cargo.toml
  README.md


⸻

Running Locally

1. Install Rust

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

2. Build the project

cargo build

3. Create .env

RPC_URL=https://mainnet.blockdag.network
PRIVATE_KEY=your_private_key_here
CONTRACT_ADDRESS=0xYourContractAddress
PORT=8080

4. Start server

cargo run


⸻

API Documentation

1. Submit Proof

POST /proofs/submit

Request Body

{
  "proofHash": "0xabc123...",
  "modelId": "model-v1",
  "valid": true
}

Response

{
  "proofId": "0x92fae1..."
}


⸻

2. Get Proof

GET /proofs/{proofId}

Response

{
  "modelId": "model-v1",
  "proofHash": "0xabc123...",
  "submitter": "0x1A2b3C...",
  "valid": true,
  "timestamp": 1732302100
}


⸻

3. Check Validity

GET /proofs/{proofId}/valid

Response

{
  "valid": true
}


⸻

4. List Proofs by Submitter

GET /proofs/submitter/{address}

Response

{
  "submitter": "0x1A2b...",
  "proofs": [
    {
      "proofId": "0xaa41...",
      "modelId": "model-v1",
      "valid": true
    }
  ]
}


⸻

5. Global Stats

GET /stats

Response

{
  "totalProofs": 120,
  "validProofs": 110,
  "invalidProofs": 10,
  "uniqueSubmitters": 23
}


⸻

Contract Loader Example (Rust)

use ethers::prelude::*;
use std::sync::Arc;

pub async fn load_contract() -> Result<VeriCortexVerifier<Provider<Http>>, ContractError> {
    let provider = Provider::<Http>::try_from(std::env::var("RPC_URL").unwrap())?;
    let wallet = std::env::var("PRIVATE_KEY")
        .unwrap()
        .parse::<LocalWallet>()?;
    let client = SignerMiddleware::new(provider, wallet.with_chain_id(1993));
    let client = Arc::new(client);
    let address: Address = std::env::var("CONTRACT_ADDRESS")
        .unwrap()
        .parse()
        .unwrap();

    let abi = include_bytes!("./abi/VeriCortexVerifier.json");
    Ok(VeriCortexVerifier::new(address, client, abi)?)
}


⸻

Testing with Curl

Submit a proof

curl -X POST http://localhost:8080/proofs/submit \
  -H "Content-Type: application/json" \
  -d '{"proofHash":"0xaaa","modelId":"test","valid":true}'

Fetch proof details

curl http://localhost:8080/proofs/0x123

Check validity

curl http://localhost:8080/proofs/0x123/valid


⸻

Future Extensibility
	•	Multi-chain support
	•	Event-driven proof ingestion
	•	JWT-authenticated endpoints
	•	SDK integrations (Python, Rust, JS)
	•	Dioxus-based web dashboard
	•	Container packaging
	•	Off-chain computation workers

⸻

Requirements
	•	Rust installed
	•	RPC URL (BlockDAG)
	•	A deployer private key
	•	Contract address
