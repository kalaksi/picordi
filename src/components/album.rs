use dioxus::prelude::*;

#[component]
pub fn Album() -> Element {
    // Number of the current page, starts from 1 and displays the highest visible page number.
    // So normally odd-numbers (right-side page) unless last page ends up being on left and even.
    let mut current_page = use_signal(|| 1);
    let mut total_pages = use_signal(|| 10);
    let mut left_page_classes = use_signal(|| "");

    let go_next = move |_event| {
        let distance_to_end = *total_pages.read() - *current_page.read();

        // Jump 2 pages normally, but close to end jump only 1 page.
        current_page += std::cmp::min(2, distance_to_end);
    };

    let go_prev = move |_event| {
        let distance_to_start = *current_page.read() - 1;

        // Normally the page number is odd (so the right-side page), but with even number of pages,
        // we end up to even number at the end, so need to correct it when going backwards.
        if *current_page.read() == *total_pages.read() && *current_page.read() % 2 == 0 {
            current_page -= std::cmp::min(1, distance_to_start);
        }
        else {
            current_page -= std::cmp::min(2, distance_to_start);
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

                    // Pages are initially laid on the right side.
                    // Even-numbered pages are facing down before they are turned.
                    {(1..*total_pages.read()).rev().map(|i| {
                        let mut classes = "page".to_string();

                        // Pages belonging to the left side (facing up then) are even-numbered.
                        let for_left_side = i % 2 == 0;

                        if for_left_side {
                            classes.push_str(" left-side");

                            if i <= *current_page.read() {
                                classes.push_str(" left-flipped");
                            }
                        } else {
                            classes.push_str(" right-side");

                            if i < *current_page.read() {
                                classes.push_str(" right-flipped");
                            }
                        }

                        // Control the stacking order of the pages so old pages won't overlap.
                        if i > *current_page.read() || i < *current_page.read() - 1 ||
                           (*current_page.read() == *total_pages.read() && for_left_side && i != *total_pages.read()) {

                            classes.push_str(" covered");
                        }

                        rsx! {
                            div {
                                class: classes,

                                // Page number.
                                div {
                                    class: "page-number",
                                    "{i}"
                                }
                            }
                        }
                    })}
                }

                // Clicking areas for page turning.
                div {
                    class: "left-page-turn-area",
                    onclick: go_prev,
                }

                div {
                    class: "right-page-turn-area",
                    onclick: go_next,
                }
            }
        }
    }
}