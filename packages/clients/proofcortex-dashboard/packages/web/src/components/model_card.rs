// src/components/model_card.rs
use dioxus::prelude::*;
use dioxus_free_icons::Icon;

#[component]
pub fn ModelCard(
    name: String,
    description: String,
    proofs: String,
    trust_score: String,
    status: String,
    status_color: String,
    status_bg: String,
    icon: fn() -> Element,
) -> Element {
    rsx! {
        div { class: "bg-white rounded-3xl shadow-2xl border border-indigo-100 hover:shadow-3xl transition",
            div { class: "p-10",
                div { class: "flex items-center gap-5 mb-6",
                    div { class: "w-20 h-20 bg-gradient-to-br from-purple-500 to-pink-500 rounded-2xl flex-center text-white text-3xl font-black shadow-lg", "{name[..3]}" }
                    div {
                        h3 { class: "text-2xl font-black text-gray-900", "{name}" }
                        p { class: "text-gray-600", "{description}" }
                    }
                }
                div { class: "space-y-4",
                    div { class: "flex justify-between", span { class: "text-gray-600", "Proofs" } span { class: "text-2xl font-bold text-indigo-600", "{proofs}" } }
                    div { class: "flex justify-between", span { class: "text-gray-600", "Trust Score" } span { class: "text-3xl font-black text-{status_color}", "{trust_score}" } }
                    div { class: "pt-4 border-t border-gray-100",
                        span { class: "inline-flex items-center gap-2 px-5 py-2 bg-{status_bg} text-{status_color} rounded-full text-sm font-bold",
                            Icon { icon: icon, class: "w-5 h-5" }
                            "{status}"
                        }
                    }
                }
            }
        }
    }
}
