use dioxus::prelude::*;

use crate::components::{Hero, ProductContainer, Sidebar};

#[component]
pub fn Home() -> Element {
    rsx! {
        Hero {}
        ProductContainer {}
        Sidebar {  }
    }
}
