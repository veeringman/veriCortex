# ProofCortex Public API Server

A lightweight Rust backend exposing public APIs to interact with the
ProofCortex Ob Chain Verifier smart contract deployed on BlockDAG’s EVM-compatible network.

This API allows off-chain agents, dApps, platforms, and CLI clients to:
- Submit verification proofs
- Fetch proof metadata
- Check proof validity
- List proofs per submitter
- Retrieve global verification statistics

---

## Project structure

proofcortex-api/
```
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
```

---

## Getting started / Running locally

1. Install Rust
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. Build the project
```sh
cargo build
```

3. Create a `.env` file with the following variables:
```
RPC_URL=https://mainnet.blockdag.network
PRIVATE_KEY=your_private_key_here
CONTRACT_ADDRESS=0xYourContractAddress
PORT=8080
```

4. Start the server
```sh
cargo run
```

---

## API Documentation

Base URL: `http://localhost:<PORT>` (default PORT is 8080)

### 1) Submit Proof
- Method: POST
- Endpoint: `/proofs/submit`
- Request JSON:
```json
{
  "proofHash": "0xabc123...",
  "modelId": "model-v1",
  "valid": true
}
```
- Response JSON:
```json
{
  "proofId": "0x92fae1..."
}
```

### 2) Get Proof
- Method: GET
- Endpoint: `/proofs/{proofId}`
- Response JSON:
```json
{
  "modelId": "model-v1",
  "proofHash": "0xabc123...",
  "submitter": "0x1A2b3C...",
  "valid": true,
  "timestamp": 1732302100
}
```

### 3) Check Validity
- Method: GET
- Endpoint: `/proofs/{proofId}/valid`
- Response JSON:
```json
{
  "valid": true
}
```

### 4) List Proofs by Submitter
- Method: GET
- Endpoint: `/proofs/submitter/{address}`
- Response JSON:
```json
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
```

### 5) Global Stats
- Method: GET
- Endpoint: `/stats`
- Response JSON:
```json
{
  "totalProofs": 120,
  "validProofs": 110,
  "invalidProofs": 10,
  "uniqueSubmitters": 23
}
```

---

## Contract loader example (Rust)

```rust
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
```

---

## Testing with curl

Submit a proof:
```sh
curl -X POST http://localhost:8080/proofs/submit \
  -H "Content-Type: application/json" \
  -d '{"proofHash":"0xaaa","modelId":"test","valid":true}'
```

Fetch proof details:
```sh
curl http://localhost:8080/proofs/0x123
```

Check validity:
```sh
curl http://localhost:8080/proofs/0x123/valid
```

---

## Future extensibility
- Multi-chain support
- Event-driven proof ingestion
- JWT-authenticated endpoints
- SDK integrations (Python, Rust, JS)
- Dioxus-based web dashboard
- Container packaging
- Off-chain computation workers

---

## Requirements
- Rust (installed)
- RPC URL (BlockDAG)
- A deployer private key
- Contract address
