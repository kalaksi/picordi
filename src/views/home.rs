use dioxus::prelude::*;
use crate::components::Book;

#[component]
pub fn Home() -> Element {
    rsx! {
        Book { }
    }
}
