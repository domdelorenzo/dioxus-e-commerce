use dioxus::prelude::*;

use crate::components::{Hero, ProductContainer, Sidebar};

#[component]
pub fn Home() -> Element {
    // let products = use_server_future(|| fetch_products(10, Sort::Ascending))?;
    // let products = products().unwrap()?;

    // use_context_provider(|| Signal::new(Cart { items: vec![] }));
    rsx! {
        Hero {}
        ProductContainer {}
        Sidebar {  }
    }
}
