#![allow(non_snake_case)]
use dioxus::prelude::*;

use components::Navbar;
use views::{ProductPage, Home};

mod components;
mod views;
mod api;
mod scroll;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/product/:id")]
    ProductPage { id: u32 },
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // Build cool things ✌️
    // use_context_provider(|| Signal::new(Cart {items: vec![]}));

    rsx! {
        // Global app resources
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        Router::<Route> {}
    }
}




// struct Cart {
//     items: Vec<api::Product>,
// }



// struct Cart {
//     items: Vec<Product>,
// }

// struct Product {
//     id: String,
//     image: String,
//     category: String,
//     title: String,
//     price: String,
// }

// impl Cart {
//     fn add(&mut self, product: Product) {
//         self.items.push(product);
//     }
// }

// impl Cart {
//     fn remove(&mut self, product_id: &str) {
//         self.items.retain(|product| product.id != product_id);
//     }
// }