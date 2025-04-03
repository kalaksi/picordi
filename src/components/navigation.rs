use crate::Route;
use dioxus::{prelude::*, router::navigation};


#[component]
pub fn Navigation() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/assets/styling/navigation.css")}

        div {
            id: "left-navigation",

            Link {
                to: Route::Home {},
                "Home"
            }
            // Link {
            //     to: Route::Blog { id: 1 },
            //     "Blog"
            // }
        }

        div {
            id: "bottom-navigation",

            button {
                onclick: |_| {
                },
                "Back"
            }

            div {
                id: "page-number",
                "Page 1"
            }

            button {
                onclick: |_| {
                },
                "Next"
            }
        }

        Outlet::<Route> {}
    }
}
