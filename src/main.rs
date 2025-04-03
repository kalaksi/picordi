use dioxus::prelude::*;

use components::Navigation;
use views::Home;

mod components;
mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navigation)]
    #[route("/")]
    Home {},
    // #[route("/blog/:id")]
    // Blog { id: i32 },
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        // Global app resources
        document::Link { rel: "icon", href: asset!("/assets/favicon.ico")}
        document::Link { rel: "stylesheet", href: asset!("/assets/styling/main.css")}

        Router::<Route> {}
    }
}
