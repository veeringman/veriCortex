flowchart LR
    subgraph Off-Chain
        Client[Client / dApp / Agent]
        API[VeriCortex SDK / API]
        Runtime[WASM Runtime Pool]
        Trace[Trace Capture]
        Proof[ZK / Crypto Proof Engine]
    end

    subgraph On-Chain
        Verifier[VeriCortex Verifier Contract]
        BlockDAG[BlockDAG Network]
        ModelRegistry[(Model & Proof Registry)]
        Explorer[Explorer / Dashboard]
        VerifiedApps[Verified dApps / Agents]
    end

    Client --> API --> Runtime --> Trace --> Proof --> Verifier --> BlockDAG
    BlockDAG --> Explorer
    BlockDAG --> ModelRegistry
    BlockDAG --> VerifiedApps
    VerifiedApps --> Client
