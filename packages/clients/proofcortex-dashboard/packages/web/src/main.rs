#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::{Router, Link, Routable};  // Specific imports — no glob to avoid ambiguity
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::ld_icons::{LdLayoutDashboard, LdPackage, LdArchive, LdShieldCheck, LdTrendingUp, LdGlobe, LdWallet, LdSettings, LdMenu, LdCirclePlus, LdCircleCheck, LdActivity, LdTriangleAlert};

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! { Router::<Route> {} }
}

#[derive(Routable, Clone, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Layout)]
    #[route("/")]
    Overview {},
    #[route("/models")]
    Models {},
    #[route("/registry")]
    Registry {},
    #[route("/proofs")]
    Proofs {},
    #[route("/trust")]
    Trust {},
    #[route("/explorer")]
    Explorer {},
    #[route("/wallet")]
    WalletConnect {},
    #[route("/settings")]
    AppSettings {},
}

#[component]
fn Layout(children: Element) -> Element {
    let mut sidebar_open = use_signal(|| false);

    rsx! {
        div { class: "min-h-screen bg-gray-50 flex flex-col lg:flex-row",
            // Mobile Header
            header { class: "lg:hidden bg-gradient-to-r from-indigo-950 to-purple-950 text-white p-4 flex justify-between items-center shadow-lg",
                button { onclick: move |_| sidebar_open.set(!sidebar_open()), Icon { icon: LdMenu , class: "w-8 h-8" } }
                img { src: "assets/ProofCortex.png", alt: "ProofCortex", class: "h-10" }
                div { class: "w-10 h-10 rounded-full bg-gradient-to-r from-indigo-500 to-purple-600 flex items-center justify-center text-white font-bold", "VV" }
            }

            // Sidebar
            aside { 
                class: "fixed lg:static inset-y-0 left-0 z-50 w-72 bg-gradient-to-b from-indigo-950 to-purple-950 text-white transform transition-transform duration-300 flex flex-col",
                class: if *sidebar_open.read() { "" } else { "-translate-x-full lg:translate-x-0" }, 
                div { class: "p-8 text-center border-b border-indigo-800",
                    img { src: "assets/ProofCortex.png", alt: "ProofCortex", class: "mx-auto w-40" }
                }
                nav { class: "flex-1 px-4 py-6 space-y-1",
                    Link { to: Route::Overview {}, class: "menu-item flex items-center gap-4 px-5 py-4 rounded-xl hover:bg-white/10 transition", Icon { icon: LdLayoutDashboard, class: "w-6 h-6" } "Overview" }
                    Link { to: Route::Models {}, class: "menu-item flex items-center gap-4 px-5 py-4 rounded-xl hover:bg-white/10 transition", Icon { icon: LdPackage , class: "w-6 h-6" } "Models" }
                    Link { to: Route::Registry {}, class: "menu-item flex items-center gap-4 px-5 py-4 rounded-xl hover:bg-white/10 transition", Icon { icon: LdArchive , class: "w-6 h-6" } "Model Registry" }
                    Link { to: Route::Proofs {}, class: "menu-item flex items-center gap-4 px-5 py-4 rounded-xl hover:bg-white/10 transition", Icon { icon: LdShieldCheck , class: "w-6 h-6" } "Proof Submissions" }
                    Link { to: Route::Trust {}, class: "menu-item flex items-center gap-4 px-5 py-4 rounded-xl hover:bg-white/10 transition", Icon { icon: LdTrendingUp , class: "w-6 h-6" } "Trust Scores" }
                    Link { to: Route::Explorer {}, class: "menu-item flex items-center gap-4 px-5 py-4 rounded-xl hover:bg-white/10 transition", Icon { icon: LdGlobe, class: "w-6 h-6" } "BlockDAG Explorer" }
                    Link { to: Route::WalletConnect {}, class: "menu-item flex items-center gap-4 px-5 py-4 rounded-xl hover:bg-white/10 transition", Icon { icon: LdWallet, class: "w-6 h-6" } "Wallet" }
                    Link { to: Route::AppSettings {}, class: "menu-item flex items-center gap-4 px-5 py-4 rounded-xl hover:bg-white/10 transition", Icon { icon: LdSettings, class: "w-6 h-6" } "Settings" }
                }
            }

            // Overlay
            if *sidebar_open.read() {
                div { class: "fixed inset-0 bg-black/50 z-40 lg:hidden", onclick: move |_| sidebar_open.set(false) }
            }

            // Main Content
            div { class: "flex-1 flex flex-col",
                header { class: "hidden lg:flex bg-white border-b px-10 py-6 justify-between items-center shadow-sm",
                    h1 { class: "text-3xl font-black bg-gradient-to-r from-indigo-600 to-purple-600 bg-clip-text text-transparent", "ProofCortex • Trust Platform for AI" }
                    div { class: "flex items-center gap-6",
                        span { class: "hidden md:block text-gray-600", "veer.vijay@proofcortex.com" }
                        div { class: "w-12 h-12 rounded-full bg-gradient-to-r from-indigo-500 to-purple-600 flex items-center justify-center text-white text-xl font-bold", "VV" }
                    }
                }
                main { class: "flex-1 p-6 lg:p-10 bg-gradient-to-br from-gray-50 to-indigo-50/20 overflow-y-auto",
                    {children}
                }
            }
        }
    }
}

#[component]
fn Overview() -> Element {
    rsx! {
        div { class: "max-w-7xl mx-auto",
            h2 { class: "text-4xl font-black text-gray-900 mb-6", "Dashboard Overview" }
            p { class: "text-xl text-gray-600 mb-12", "Real-time monitoring of AI model trust and proof integrity" }

            div { class: "grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-10",
                div { class: "bg-white rounded-3xl shadow-2xl p-10 border border-indigo-100 hover:shadow-3xl transition",
                    div { class: "flex justify-between items-start",
                        div {
                            p { class: "text-gray-500 text-sm font-semibold mb-2", "Total Models" }
                            p { class: "text-6xl font-black text-indigo-600", "12" }
                        }
                        Icon { icon: LdLayoutDashboard, class: "w-14 h-14 text-indigo-500 opacity-20" }
                    }
                }
                div { class: "bg-white rounded-3xl shadow-2xl p-10 border border-indigo-100 hover:shadow-3xl transition",
                    div { class: "flex justify-between items-start",
                        div {
                            p { class: "text-gray-500 text-sm font-semibold mb-2", "Proofs Submitted" }
                            p { class: "text-6xl font-black text-emerald-600", "184" }
                        }
                        Icon { icon: LdLayoutDashboard , class: "w-14 h-14 text-indigo-500 opacity-20" } 
                    }
                }
                div { class: "bg-white rounded-3xl shadow-2xl p-10 border border-indigo-100 hover:shadow-3xl transition",
                    div { class: "flex justify-between items-start",
                        div {
                            p { class: "text-gray-500 text-sm font-semibold mb-2", "Global Trust Score" }
                            p { class: "text-6xl font-black text-teal-600", "96.8%" }
                        }
                        Icon { icon: LdTrendingUp , class: "w-14 h-14 text-teal-600 opacity-20" }
                    }
                }
            }
        }
    }
}

#[component]
fn Models() -> Element {
    rsx! {
        div { class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 pt-8 lg:pt-12",
            h2 { class: "text-4xl font-black text-gray-900", "Registered Models" }
            p { class: "mt-4 text-xl text-gray-600", "All AI models onboarded to ProofCortex with immutable proof trails" }

            button { class: "mt-10 inline-flex items-center gap-4 px-10 py-6 bg-gradient-to-r from-indigo-600 to-purple-600 text-white text-xl font-bold rounded-3xl shadow-2xl hover:shadow-purple-600/50 transform hover:scale-105 transition",
                Icon { icon: LdPackage , class: "w-8 h-8" } "Register New Model"
            }

            div { class: "mt-12 grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-10",
                // Model cards (your HTML design in rsx!)
                div { class: "bg-white rounded-3xl shadow-2xl border border-indigo-100 hover:shadow-3xl transition",
                    div { class: "p-10",
                        div { class: "flex items-center gap-5 mb-6",
                            div { class: "w-20 h-20 bg-gradient-to-br from-purple-500 to-pink-500 rounded-2xl flex-center text-white text-3xl font-black shadow-lg", "LLM" }
                            div {
                                h3 { class: "text-2xl font-black text-gray-900", "LLM Assistant Enterprise" }
                                p { class: "text-gray-600", "70B parameter model" }
                            }
                        }
                        div { class: "space-y-4",
                            div { class: "flex justify-between", span { class: "text-gray-600", "Proofs" } span { class: "text-2xl font-bold text-indigo-600", "104" } }
                            div { class: "flex justify-between", span { class: "text-gray-600", "Trust Score" } span { class: "text-3xl font-black text-emerald-600", "98.7%" } }
                            div { class: "pt-4 border-t border-gray-100",
                                span { class: "inline-flex items-center gap-2 px-5 py-2 bg-emerald-100 text-emerald-800 rounded-full text-sm font-bold",
                                    Icon { icon: LdCircleCheck , class: "w-5 h-5" } "HEALTHY"
                                }
                            }
                        }
                    }
                }
                // Add other model cards...
            }
        }
    }
}

// Add other components (Proofs, Trust, Explorer, etc.) — copy your HTML into rsx! blocks
#[component]
fn Proofs() -> Element {
    rsx! {
        div { class: "max-w-7xl mx-auto",
            h2 { class: "text-4xl font-black text-gray-900", "Proof Submissions" }
            // Your full Proofs section as rsx! — exact HTML design
        }
    }
}

// Continue for Registry, Trust, Explorer, Wallet, Settings...
#[component]
fn Registry() -> Element {
    rsx! {
        div { class: "max-w-7xl mx-auto",
            h2 { class: "text-4xl font-black text-gray-900", "Proof Submissions" }
            // Your full Proofs section as rsx! — exact HTML design
        }
    }
}

#[component]
fn Trust() -> Element {
    rsx! {
        div { class: "max-w-7xl mx-auto",
            h2 { class: "text-4xl font-black text-gray-900", "Proof Submissions" }
            // Your full Proofs section as rsx! — exact HTML design
        }
    }
}
#[component]
fn Explorer() -> Element {
    rsx! {
        div { class: "max-w-7xl mx-auto",
            h2 { class: "text-4xl font-black text-gray-900", "Proof Submissions" }
            // Your full Proofs section as rsx! — exact HTML design
        }
    }
}
#[component]
fn WalletConnect() -> Element {
    rsx! {
        div { class: "max-w-7xl mx-auto",
            h2 { class: "text-4xl font-black text-gray-900", "Proof Submissions" }
            // Your full Proofs section as rsx! — exact HTML design
        }
    }
}
#[component]
fn AppSettings() -> Element {
    rsx! {
        div { class: "max-w-7xl mx-auto",
            h2 { class: "text-4xl font-black text-gray-900", "Proof Submissions" }
            // Your full Proofs section as rsx! — exact HTML design
        }
    }
}

