use dioxus::prelude::*;
use crate::components::Album;

#[component]
pub fn Home() -> Element {
    rsx! {
        Album { }
    }
}
