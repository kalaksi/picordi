use crate::Route;
use dioxus::{html::g::to, prelude::*, router::navigation};


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

        Outlet::<Route> {}
    }
}