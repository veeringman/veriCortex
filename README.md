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

**Caption / Use:** paste into README; this visually explains the end-to-end PoI flow: client → warm runtime → trace → proof → verifier → chain → app/dashboard.

---

## 2) Deployment / Infrastructure (Mermaid - Subgraph / Components)

Paste this into README or a separate `architecture.md`:

```markdown
```mermaid
graph TB
  subgraph Users["Users & Integrations"]
    U1[Developers / dApps]
    U2[Agents / Oracles]
  end

  subgraph Edge["Edge / Ingest"]
    API[Public API / Gateway]
    SDKs[SDKs (rust / go / js)]
  end

  subgraph Compute["VeriCortex Compute Cluster"]
    direction TB
    subgraph PoolA["Warm WASM Cluster (region A)"]
      WA1[WarmInstance A1]
      WA2[WarmInstance A2]
    end
    subgraph PoolB["Warm WASM Cluster (region B)"]
      WB1[WarmInstance B1]
      WB2[WarmInstance B2]
    end
    Recorder[Trace Recorder]
    ProofGen[Proof Generator (ZK / Crypto)]
    Storage[Artifact Store<br/>(model bundles, traces)]
    KMS[KMS / Key Vault]
  end

  subgraph Verifier["Verifier & Settlement"]
    Relay[Verifier Service / Relayer]
    BlockDAG[BlockDAG Network / Smart Contract]
    Registry[Model Registry (on-chain/off-chain)]
  end

  subgraph Ops["Ops & Observability"]
    Logs[Logging / Metrics]
    Dashboard[Ops Dashboard / Explorer]
    CI[CI / Deployment]
  end

  U1 --> API
  U2 --> API
  API --> SDKs
  SDKs --> WA1
  SDKs --> WB1

  WA1 --> Recorder
  WA2 --> Recorder
  WB1 --> Recorder
  WB2 --> Recorder

  Recorder --> ProofGen
  ProofGen --> Storage
  ProofGen --> Relay
  ProofGen --> KMS

  Relay --> BlockDAG
  Registry --> BlockDAG
  BlockDAG --> Dashboard

  Logs --> Dashboard
  CI --> WA1
  CI --> WB1

  style Compute fill:#07102a,stroke:#00E5D4,color:#fff
  style Verifier fill:#0A102A,stroke:#7A5CFF,color:#fff
  style Ops fill:#081026,stroke:#88FFE3,color:#fff
  classDef small font-size:12px;
  class Recorder,ProofGen,Storage,KMS small;


**Caption / Use:** shows regional warm clusters, trace recorder, proof generation, storage, relayer/verifier and how they interact with BlockDAG and dev tools. This is suitable for architecture slides and technical docs.

---

## Tips for Usage

- Place each block inside a fenced code block with `mermaid` language as shown.
- GitHub will render; some older instances require enabling mermaid rendering in settings — most modern repos render fine.
- To export as SVG/PNG for slides:
  - Option A: Paste into an online Mermaid live editor (mermaid.live) and export SVG/PNG.
  - Option B: Use VS Code with Mermaid preview + export.
- For pitch decks, export the PNG/SVG then polish in Figma/Canva (apply brand colors and icons).

---

## Optional: Polished Visuals
If you want a fully polished graphic (gradient blocks, icons, and export-ready SVG/PNG) I can generate a high-fidelity image (PNG + SVG) for slides. Reply **"Export PNG/SVG"** and I’ll produce a branded diagram (VeriCortex colors, gradients, icons).

---

Would you like me to:
- Export the diagrams as PNG/SVG now? (`yes` / `no`)  
- Or produce a single-slide PPT containing these diagrams? (`ppt`)
::contentReference[oaicite:0]{index=0}

