# ProofCortexVerifier – On-chain Anchor for Off-chain AI Inference Proofs

[![Solidity](https://img.shields.io/badge/Solidity-%5E0.8.20-brightgreen)](https://soliditylang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Design](https://img.shields.io/badge/Design-Lightweight%20&%20Auditable-blue)]()

**ProofCortexVerifier** is a minimal, gas-efficient smart contract that permanently anchors cryptographic proofs of off-chain AI/ML inference on any EVM-compatible blockchain (Ethereum, Polygon, Arbitrum, BlockDAG layers, etc.).

It allows a trusted off-chain verifier service to record:
- Model identifier and version
- Hashes of input, output, execution trace, and full proof bundle
- Validation result (valid/invalid)
- Submitter address and timestamp

Proof validity can be updated until explicitly locked, enabling deferred or multi-stage verification workflows.

Perfect for ZKML, decentralized AI agents, inference marketplaces, and trustless AI applications.

## Features

- Immutable on-chain proof registry
- Optional single trusted submitter (recommended) or fully permissionless mode
- Validity can be updated until `lockProof` is called
- Full event history for indexing and monitoring
- Extremely low gas usage – stores only essential hashes
- Simple, auditable code – easy to verify and fork
- MIT licensed – free for commercial and open-source use

## Smart Contract
#### *Solidity*
```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

/// @title ProofCortexVerifier
/// @notice On-chain anchor for off-chain AI inference proofs
/// @dev Designed for BlockDAG’s EVM layer – lightweight & auditable.
contract ProofCortexVerifier {

    struct ProofRecord {
        string modelId;
        string version;          // Model version or commit
        bytes32 inputHash;       // Hash of input to the model
        bytes32 outputHash;      // Hash of model output
        bytes32 traceHash;       // Trace/Merkle root of execution steps
        bytes32 proofHash;       // Combined proof bundle hash
        address submitter;       // Off-chain verifier service
        bool valid;              // Off-chain validation result
        uint256 timestamp;       // Anchor time
        bool locked;             // Prevent further updates
    }

    mapping(bytes32 => ProofRecord) public proofs;

    // Optional: restrict who may post proofs
    address public trustedSubmitter;

    event ProofSubmitted(bytes32 indexed proofId, address indexed submitter, string modelId, bool valid);
    event ProofUpdated(bytes32 indexed proofId, bool valid, uint256 timestamp);
    event ProofAnchored(bytes32 indexed proofId, bytes32 proofHash, uint256 time);

    constructor(address _trustedSubmitter) {
        trustedSubmitter = _trustedSubmitter;
    }

    modifier onlyTrusted() {
        require(msg.sender == trustedSubmitter || trustedSubmitter == address(0), "Not authorized");
        _;
    }

    function submitProof(
        string calldata modelId,
        string calldata version,
        bytes32 inputHash,
        bytes32 outputHash,
        bytes32 traceHash,
        bytes32 proofHash,
        bool valid
    ) external onlyTrusted returns (bytes32 proofId) {

        proofId = keccak256(abi.encodePacked(modelId, version, proofHash, msg.sender));

        require(proofs[proofId].submitter == address(0), "Proof already exists");

        proofs[proofId] = ProofRecord({
            modelId: modelId,
            version: version,
            inputHash: inputHash,
            outputHash: outputHash,
            traceHash: traceHash,
            proofHash: proofHash,
            submitter: msg.sender,
            valid: valid,
            timestamp: block.timestamp,
            locked: false
        });

        emit ProofSubmitted(proofId, msg.sender, modelId, valid);
        emit ProofAnchored(proofId, proofHash, block.timestamp);
    }

    function updateProofResult(bytes32 proofId, bool newValidity) external onlyTrusted {
        ProofRecord storage p = proofs[proofId];
        require(p.submitter != address(0), "Proof not found");
        require(!p.locked, "Proof is locked");

        p.valid = newValidity;
        p.timestamp = block.timestamp;

        emit ProofUpdated(proofId, newValidity, block.timestamp);
    }

    function lockProof(bytes32 proofId) external onlyTrusted {
        proofs[proofId].locked = true;
    }

    function getProof(bytes32 proofId) external view returns (ProofRecord memory) {
        return proofs[proofId];
    }

    function isProofValid(bytes32 proofId) external view returns (bool) {
        return proofs[proofId].valid;
    }
}
```

---
## Deployment
#### *JavaScript*
```JavaScript
//JavaScript
const Verifier = await ethers.getContractFactory("ProofCortexVerifier");
const verifier = await Verifier.deploy("0xYourTrustedService"); // or address(0)
await verifier.waitForDeployment();

```
----
## Quick Usage
#### *JavaScript*
```JavaScript
//JavaScript
await verifier.submitProof(
  "llama-3.1-70b",
  "v2025.08",
  inputHash,
  outputHash,
  traceHash,
  proofBundleHash,
  true
);

// If needed later
await verifier.updateProofResult(proofId, false);
await verifier.lockProof(proofId);
```
---
## Calculate proofId off-chain

#### *JavaScript*
```JavaScript
const proofId = ethers.keccak256(
  ethers.solidityPacked(
    ["string","string","bytes32","address"],
    [modelId, version, proofHash, submitter]
  )
);
```
#### *Rust*
```Rust
use ethers::prelude::*;
use ethers::utils::keccak256;

// Types
type ProofId = H256; // bytes32

fn compute_proof_id(
    model_id: &str,
    version: &str,
    proof_hash: H256,
    submitter: Address,
) -> ProofId {
    // solidityPacked packs exactly like ethers.solidityPacked(...)
    let packed = abi::encode_packed(&[
        abi::Token::String(model_id.to_string()),
        abi::Token::String(version.to_string()),
        abi::Token::FixedBytes(proof_hash.as_bytes().to_vec()),
        abi::Token::Address(submitter),
    ])
    .expect("Packing failed");

    // keccak256 hash of the packed bytes
    H256::from(keccak256(packed))
}

// Example usage
fn main() {
    let model_id = "llama-3.1-70b";
    let version = "v2025.08";
    let proof_hash = H256::random();
    let submitter = Address::random();

    let proof_id = compute_proof_id(model_id, version, proof_hash, submitter);
    println!("Proof ID: {:?}", proof_id);
}
```
---

## Events
```solidity
ProofSubmitted(bytes32 indexed proofId, address indexed submitter, string modelId, bool valid)
ProofUpdated(bytes32 indexed proofId, bool valid, uint256 timestamp)
ProofAnchored(bytes32 indexed proofId, bytes32 proofHash, uint256 time)
```

---
## Security Notes

Contract only anchors hashes – it trusts the trustedSubmitter
Validity is mutable until lockProof() is called
After locking, the record is forever immutable

---
## License

```license
MIT License

Copyright (c) 2025 Vijay Sharma / Cortex Chain Team

Permission is hereby granted, free of charge, to any person obtaining a copy...
```
