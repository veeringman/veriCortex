// src/app.rs
use dioxus::prelude::*;
use dioxus_router::{Router, Routable};
use crate::routes::Route;

#[component]
pub fn App() -> Element {
    rsx! { Router::<Route> {} }
}
