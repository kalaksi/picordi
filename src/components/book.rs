use dioxus::prelude::*;

#[component]
pub fn Book() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/assets/styling/book.css")}

        div {
            id: "book-background",

            div {
                id: "book",
            }
        }
    }
}