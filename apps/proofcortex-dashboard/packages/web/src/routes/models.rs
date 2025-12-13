use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::ld_icons::{LdPlusCircle, LdCheckCircle, LdActivity, LdAlertTriangle};

#[component]
pub fn Models() -> Element {
    let mut visible_count = use_signal(|| 3);
    let models = use_signal(|| vec![
        ("LLM Assistant Enterprise", "70B parameter model", "104", "98.7%", "HEALTHY", "emerald-600", "emerald-100", LdCheckCircle),
        ("Transaction Fraud Detector", "XGBoost ensemble", "68", "97.2%", "MONITOR", "emerald-600", "emerald-100", LdActivity),
        ("Credit Risk Scoring v1", "Neural net + rules engine", "12", "89.1%", "ALERT", "red-600", "red-100", LdAlertTriangle),
        // ... more
    ]);

    let load_more = move |_| {
        if *visible_count.read() < models.read().len() {
            visible_count.set(*visible_count.read() + 3);
        }
    };

    rsx! {
        div { class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 pt-8 lg:pt-12",
            h2 { class: "text-4xl font-black text-gray-900", "Registered Models" }
            p { class: "mt-4 text-xl text-gray-600", "All AI models onboarded to ProofCortex" }

            button { onclick: move |_| super::super::show_template("registerModel", "templates/registerModel.html", "models"),
                class: "mt-10 inline-flex items-center gap-4 px-10 py-6 bg-gradient-to-r from-indigo-600 to-purple-600 text-white text-xl font-bold rounded-3xl shadow-2xl hover:shadow-purple-600/50 transform hover:scale-105 transition",
                Icon { icon: LdPlusCircle, class: "w-8 h-8" }
                "Register New Model"
            }

            div { class: "mt-12 grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-10",
                for (i, model) in models.read().iter().enumerate().take(*visible_count.read()) {
                    super::super::components::ModelCard {
                        name: model.0.to_string(),
                        description: model.1.to_string(),
                        proofs: model.2.to_string(),
                        trust_score: model.3.to_string(),
                        status: model.4.to_string(),
                        status_color: model.5.to_string(),
                        status_bg: model.6.to_string(),
                        icon: model.7
                    }
                }
            }

            if *visible_count.read() < models.read().len() {
                div { class: "mt-12 text-center",
                    button { onclick: load_more,
                        class: "px-12 py-6 bg-gradient-to-r from-gray-600 to-gray-800 text-white text-xl font-bold rounded-3xl shadow-2xl hover:shadow-gray-700/50 transform hover:scale-105 transition",
                        "Load More Models"
                    }
                }
            }
        }
    }
}
