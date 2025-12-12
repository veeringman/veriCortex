// src/routes/overview.rs
use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::ld_icons::{LdLayoutDashboard, LdShieldCheck, LdTrendingUp};

#[component]
pub fn Overview() -> Element {
    rsx! {
        div { class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 pt-8 lg:pt-12",
            // Header
            div { class: "text-left mb-12",
                h2 { class: "text-3xl lg:text-4xl font-black text-gray-900 leading-tight", "Dashboard Overview" }
                p { class: "mt-4 text-lg lg:text-xl text-gray-600 max-w-4xl", "Real-time monitoring of AI model trust and proof integrity across your entire ProofCortex deployment" }
            }

            // Stat Cards
            div { class: "grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-10 mb-16",
                // Card 1: Total Models
                div { class: "bg-white rounded-3xl shadow-2xl p-10 border border-indigo-100 hover:shadow-3xl transition-all duration-300",
                    div { class: "flex justify-between items-start",
                        div {
                            p { class: "text-gray-500 text-sm font-semibold mb-2", "Total Models Onboarded" }
                            p { class: "text-5xl lg:text-6xl font-black text-indigo-600 mt-4", "12" }
                        }
                        Icon { icon: LdLayoutDashboard, class: "w-14 h-14 text-indigo-500 opacity-20" }
                    }
                }

                // Card 2: Proofs Submitted
                div { class: "bg-white rounded-3xl shadow-2xl p-10 border border-indigo-100 hover:shadow-3xl transition-all duration-300",
                    div { class: "flex justify-between items-start",
                        div {
                            p { class: "text-gray-500 text-sm font-semibold mb-2", "Proofs Submitted" }
                            p { class: "text-5xl lg:text-6xl font-black text-emerald-600 mt-4", "184" }
                        }
                        Icon { icon: LdShieldCheck, class: "w-14 h-14 text-emerald-600 opacity-20" }
                    }
                }

                // Card 3: Global Trust Score
                div { class: "bg-white rounded-3xl shadow-2xl p-10 border border-indigo-100 hover:shadow-3xl transition-all duration-300",
                    div { class: "flex justify-between items-start",
                        div {
                            p { class: "text-gray-500 text-sm font-semibold mb-2", "Global Trust Score" }
                            p { class: "text-5xl lg:text-6xl font-black text-teal-600 mt-4", "96.8%" }
                        }
                        Icon { icon: LdTrendingUp, class: "w-14 h-14 text-teal-600 opacity-20" }
                    }
                }
            }

            // Quick Actions
            div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-8",
                // Action 1
                div { class: "bg-gradient-to-r from-indigo-600 to-purple-600 text-white rounded-3xl p-8 shadow-2xl hover:shadow-3xl transition-all duration-300 cursor-pointer",
                    Icon { icon: LdPackage, class: "w-12 h-12 mb-4" }
                    h4 { class: "text-2xl font-bold mb-2", "Register Model" }
                    p { class: "text-indigo-100", "Onboard new AI model with immutable storage" }
                }
                // Add other 3 actions...
            }
        }
    }
}
