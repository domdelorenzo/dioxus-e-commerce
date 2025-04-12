// use crate::components::Hero;
// use crate::components::ProductContainer;
use dioxus::prelude::*;

use crate::{
    components::Hero,
    components::ProductContainer,
};


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

#[component]
pub fn Home() -> Element {
    // let products = use_server_future(|| fetch_products(10, Sort::Ascending))?;
    // let products = products().unwrap()?;

    // use_context_provider(|| Signal::new(Cart { items: vec![] }));
    rsx! {
        Hero {}

        ProductContainer {}
    }
}
