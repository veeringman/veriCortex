use dioxus::prelude::*;
use serde::Deserialize;
use crate::api;

#[derive(Deserialize, Debug, Clone)]
struct ProofEntry { proofId: String, modelId: String, valid: bool }

pub fn ProofList(cx: Scope) -> Element {
    let proofs = use_state(&cx, || Vec::<ProofEntry>::new());
    use_effect(&cx, (), { let proofs = proofs.clone(); async move {
        // fetch events from public API
        if let Ok(v) = api::fetch_recent_events().await {
            proofs.set(v);
        }
    }});

    cx.render(rsx!(
        ul {
            for proof in proofs.get().iter().map(|p| rsx!(
                li { key: "{p.proofId}", style: "padding:0.5rem 0; border-bottom:1px solid #eee;",
                    div { strong { "{p.modelId}" } }
                    div { small { "Proof: {p.proofId} — " } span { if p.valid { "VALID" } else { "INVALID" } } }
                }
            ))
        }
    ))
}
