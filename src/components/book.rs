use dioxus::prelude::*;

#[component]
pub fn Book() -> Element {
    let go_next = move |event| {

    };

    let go_back = move |event| {

    };

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/assets/styling/book.css")}

        div {
            id: "book-background",

            div {
                id: "book",

                div {
                    class: "page",
                    onclick: go_back,
                }
                div {
                    class: "page",
                    onclick: go_next,
                }
            }
        }
    }
}