// src/app.rs
use dioxus::prelude::*;
use dioxus_router::{Router, Routable};
use crate::routes::{Route, Layout};

#[component]
pub fn App() -> Element {
    rsx! { Router::<Route> {} }
}
