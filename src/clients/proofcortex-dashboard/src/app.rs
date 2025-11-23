use dioxus::prelude::*;
mod components;
use components::dashboard::Dashboard;

mod components {
    pub mod wallet;
    pub mod proof_submit;
}

pub fn app(cx: Scope) -> Element {
    cx.render(rsx!(
        style { "body { background: #f7fafc; color: #0f172a; }" }
        div { class: "container",
            h1 { "ProofCortex Dashboard" }
            p { "Verifiable AI inference proofs & model registry" }
            Dashboard {}
        }

    ))
}
