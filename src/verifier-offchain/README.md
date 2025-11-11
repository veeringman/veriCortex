# Verifier Offchain (Demo)

This crate implements a **starter off-chain verifier** for VeriCortex Buildathon demo.

Features:
- Local recompute & verification of a proof JSON (`tests/sample_proof.json`)
- Offline-only mode (no RPC configured)
- Optional on-chain submission via `ethers-rs` if `.env` contains RPC_URL and SUBMITTER_PRIVATE_KEY

How to run:
1. Copy `.env.example` to `.env` and fill any values if you want on-chain submission
2. `cd src/verifier-offchain`
3. `cargo run --release`

For offline verification, leave `.env` unset. For demo submission, set `RPC_URL` & `SUBMITTER_PRIVATE_KEY` & `VERIFIER_CONTRACT_ADDRESS`.
