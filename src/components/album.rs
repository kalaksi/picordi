use dioxus::prelude::*;

#[component]
pub fn Album() -> Element {
    let mut current_page = use_signal(|| 1);
    let mut total_pages = use_signal(|| 10);
    let mut left_page_classes = use_signal(|| "");
    let mut right_page_classes = use_signal(|| "");

    let go_next = move |_event| {
        if *current_page.read() < *total_pages.read() {
            current_page += 1;
            right_page_classes.set("page-turn-next");
        }
    };

    let go_prev = move |_event| {
        if *current_page.read() > 1 {
            current_page -= 1;
            left_page_classes.set("page-turn-prev");
        }
    };

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/assets/styling/album.css")}

        div {
            id: "bottom-navigation",

            button {
                onclick: go_prev,
                "Back",
            }

            div {
                id: "page-number",
                "Page {current_page} of {total_pages}"
            }

            button {
                onclick: go_next,
                "Next"
            }
        }

        div {
            id: "album-background",

            div {
                id: "album",

                div {
                    id: "page-container",

                    // Pages are initially laid on the right side. Even numbered pages are facing down before they are turned.
                    {(1..*total_pages.read()).rev().map(|i| {
                        let is_flipped = i < *current_page.read();

                        rsx! {
                            div {
                                class: if is_flipped { "page flipped" } else { "page" },
                                onclick: go_next,

                                // Page number.
                                div {
                                    class: "page-number",
                                    "{i}"
                                }
                            }
                        }
                    })}
                }
            }
        }
    }
}