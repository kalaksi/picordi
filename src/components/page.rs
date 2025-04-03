use dioxus::prelude::*;

#[component]
pub fn Page() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/assets/styling/book.css")}

        div {
        }
    }
}