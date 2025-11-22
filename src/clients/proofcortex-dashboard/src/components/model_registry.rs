use dioxus::prelude::*;
use serde::{Serialize, Deserialize};
use crate::api;

#[derive(Serialize, Deserialize, Clone, Debug)]
struct ModelInfo { model_id: String, name: String, owner: String, description: String, trust: Option<f64> }

pub fn ModelRegistry(cx: Scope) -> Element {
    let models = use_state(&cx, || Vec::<ModelInfo>::new());
    use_effect(&cx, (), { let models = models.clone(); async move {
        if let Ok(ms) = api::fetch_models().await {
            models.set(ms);
        }
    }});

    cx.render(rsx!(
        div {
            for m in models.get().iter().map(|m| rsx!(
                div { key: "{m.model_id}", style: "margin-bottom:1rem;",
                    h4 { "{m.name}" }
                    p { "{m.description}" }
                    p { small { "Owner: {m.owner} — Trust: {m.trust.unwrap_or(0.0)}" } }
                }
            ))
        }
    ))
}
