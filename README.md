flowchart LR
    Client[Client / Agent / dApp] --> API[VeriCortex API / SDK]
    API --> Runtime[Deterministic Runtime\n(WASM Warm Pool)]
    Runtime --> Trace[Execution Trace Capture]
    Trace --> Proof[Proof Generator\n(ZK / crypto proofs)]
    Proof --> Verifier[Verifier Service]
    Verifier --> BlockDAG[On-chain Verifier / Block-DAG]
    BlockDAG --> Explorer[Dashboard / Explorer]
    BlockDAG --> dApps[Verified Apps / Agents]

    ModelRegistry[(Model Registry)] --> BlockDAG
