use dioxus::prelude::*;
use dioxus_router::Link;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::ld_icons::{
    LdLayoutDashboard, LdPackage, LdArchive, LdShieldCheck, LdTrendingUp,
    LdGlobe, LdWallet, LdSettings, LdMenu
};
use crate::routes::Route;

#[component]
pub fn Layout(children: Element) -> Element {
    let mut sidebar_open = use_signal(|| false);

    rsx! {
        div { class: "min-h-screen bg-gray-50 flex flex-col lg:flex-row",
            // Mobile Header
            header { class: "lg:hidden bg-gradient-to-r from-indigo-950 to-purple-950 text-white p-4 flex justify-between items-center shadow-lg",
                button { onclick: move |_| sidebar_open.set(!sidebar_open()), Icon { icon: LdMenu, class: "w-8 h-8" } }
                img { src: "/assets/ProofCortex.png", alt: "ProofCortex", class: "h-10" }
                div { class: "w-10 h-10 rounded-full bg-gradient-to-r from-indigo-500 to-purple-600 flex-center text-white font-bold", "VV" }
            }

            // Sidebar
            aside { 
                class: "fixed lg:static inset-y-0 left-0 z-50 w-72 bg-gradient-to-b from-indigo-950 to-purple-950 text-white transform transition-transform duration-300 flex flex-col",
                class: if *sidebar_open.read() { "" } else { "-translate-x-full lg:translate-x-0" },
                div { class: "p-8 text-center border-b border-indigo-800",
                    img { src: "/assets/ProofCortex.png", alt: "ProofCortex", class: "mx-auto w-40" }
                }
                nav { class: "flex-1 px-4 py-6 space-y-1",
                    Link { to: Route::Overview {}, class: "flex items-center gap-4 px-5 py-4 rounded-xl hover:bg-white/10 transition text-white bg-white/10 font-medium", Icon { icon: LdLayoutDashboard, class: "w-6 h-6" } "Overview" }
                    Link { to: Route::Models {}, class: "flex items-center gap-4 px-5 py-4 rounded-xl hover:bg-white/10 transition", Icon { icon: LdPackage, class: "w-6 h-6" } "Models" }
                    Link { to: Route::Registry {}, class: "flex items-center gap-4 px-5 py-4 rounded-xl hover:bg-white/10 transition", Icon { icon: LdArchive, class: "w-6 h-6" } "Model Registry" }
                    Link { to: Route::Proofs {}, class: "flex items-center gap-4 px-5 py-4 rounded-xl hover:bg-white/10 transition", Icon { icon: LdShieldCheck, class: "w-6 h-6" } "Proof Submissions" }
                    Link { to: Route::Trust {}, class: "flex items-center gap-4 px-5 py-4 rounded-xl hover:bg-white/10 transition", Icon { icon: LdTrendingUp, class: "w-6 h-6" } "Trust Scores" }
                    Link { to: Route::Explorer {}, class: "flex items-center gap-4 px-5 py-4 rounded-xl hover:bg-white/10 transition", Icon { icon: LdGlobe, class: "w-6 h-6" } "BlockDAG Explorer" }
                    Link { to: Route::Wallet {}, class: "flex items-center gap-4 px-5 py-4 rounded-xl hover:bg-white/10 transition", Icon { icon: LdWallet, class: "w-6 h-6" } "Wallet" }
                    Link { to: Route::Settings {}, class: "flex items-center gap-4 px-5 py-4 rounded-xl hover:bg-white/10 transition", Icon { icon: LdSettings, class: "w-6 h-6" } "Settings" }
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
                        div { class: "w-12 h-12 rounded-full bg-gradient-to-r from-indigo-500 to-purple-600 flex-center text-white text-xl font-bold", "VV" }
                    }
                }
                main { class: "flex-1 p-6 lg:p-10 bg-gradient-to-br from-gray-50 to-indigo-50/20 overflow-y-auto",
                    Outlet::<Route> {}
                }
            }
        }
    }
}
