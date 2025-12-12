// src/routes.rs
use dioxus::prelude::*;
use dioxus_router::Routable;
use crate::components::layout::Layout;

// Import all your page components
pub mod overview;
pub mod models;
pub mod proofs;
pub mod registry;
pub mod trust;
pub mod explorer;
pub mod wallet;
pub mod settings;

use overview::Overview;
use models::Models;
use proofs::Proofs;
use registry::Registry;
use trust::Trust;
use explorer::Explorer;
use wallet::Wallet;
use settings::Settings;

#[derive(Routable, Clone, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Layout)]
        #[route("/")]
        Overview {},
        #[route("/models")]
        Models {},
        #[route("/proofs")]
        Proofs {},
        #[route("/registry")]
        Registry {},
        #[route("/trust")]
        Trust {},
        #[route("/explorer")]
        Explorer {},
        #[route("/wallet")]
        Wallet {},
        #[route("/settings")]
        Settings {},
}
