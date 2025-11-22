use dioxus::prelude::*;
use crate::components::proof_list::ProofList;
use crate::components::model_registry::ModelRegistry;

pub fn Dashboard(cx: Scope) -> Element {
    cx.render(rsx!(
        div {
            style: "display:flex; gap:2rem;",
            div { style: "flex:2;",
                h2 { "Recent Proofs" }
                ProofList {}
            }
            div { style: "flex:1; background:#fff; padding:1rem; border-radius:8px; box-shadow:0 2px 6px rgba(0,0,0,0.06);",
                h3 { "Models & Trust" }
                ModelRegistry {}
            }
        }
    ))
}
