```mermaid
flowchart LR
  subgraph ClientLayer["Client / dApp / Agent"]
    C[Client / Agent]
  end

  subgraph RuntimeLayer["VeriCortex Runtime"]
    R1[Request API / SDK]
    R2[WASM Warm Pool<br/>(Deterministic Instances)]
    R3[Deterministic Trace Recorder]
    R4[Proof Generator<br/>(ZK or Crypto)]
  end

  subgraph VerificationLayer["Verification & Settlement"]
    V1[On-chain Verifier<br/>(BlockDAG)]
    V2[Proof Verifier Service<br/>(Off-chain)]
    Registry[Model Registry<br/>(model_id → model_hash)]
  end

  subgraph AppLayer["Consumers"]
    D[Dashboard / Explorer]
    App[Verified dApp Logic]
  end

  C --> R1
  R1 --> R2
  R2 --> R3
  R3 --> R4
  R4 --> V2
  V2 --> V1
  V1 --> D
  V1 --> App
  Registry --> V1
  style RuntimeLayer fill:#0A102A,stroke:#19D3E0,color:#fff
  style VerificationLayer fill:#07102a,stroke:#7A5CFF,color:#fff
  style ClientLayer fill:#0f1b2d,stroke:#00E5D4,color:#fff
  style AppLayer fill:#091025,stroke:#88FFE3,color:#fff

  classDef smallFont font-size:12px;
  class R2,R3,R4,V1,V2,Registry smallFont;
