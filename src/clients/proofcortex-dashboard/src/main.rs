use dioxus::prelude::*;
mod app;

fn main() {
    // bootstrap for web
    dioxus_web::launch(app::app);
}
