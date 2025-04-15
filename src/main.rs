#![allow(non_snake_case)]
use dioxus::prelude::*;

use components::Navbar;
use views::{Home, ProductPage};

mod api;
mod components;
mod scroll;
mod views;

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

#[derive(Clone, Copy)]
pub struct SideBarExpanded(bool);

#[derive(Clone, Default, PartialEq)]
pub struct Product {
    id: u32,
    description: String,
    title: String,
    price: f32,
    image: String,
    amount: usize,
    category: String,
}

#[derive(Clone, Default)]
pub struct Cart {
    // items: Signal<Vec<api::Product>>
    items: Signal<Vec<Product>>,
}

impl Cart {
    // fn add(&mut self, product: api::Product) {
    //     self.items.write().push(product);
    // }
    fn add(&mut self, product: Product) {
        // Check if the product already exists in the cart
        let mut items = self.items.write(); // Create a single mutable borrow
        if let Some(existing_product) = items.iter_mut().find(|p| p.id == product.id) {
            // If it exists, increment the amount
            existing_product.amount += 1;
        } else {
            // If it doesn't exist, add it to the cart
            items.push(product);
        }
    }

    fn increase_amount(&mut self, id: u32) {
        let mut items = self.items.write();
        if let Some(product) = items.iter_mut().find(|p| p.id == id) {
            product.amount += 1;
        }
    }
    fn decrease_amount(&mut self, id: u32) {
        let mut items = self.items.write();
        if let Some(product) = items.iter_mut().find(|p| p.id == id) {
            if product.amount > 1 {
                product.amount -= 1;
            } else {
                // If the amount is 1, remove the product from the cart
                items.retain(|p| p.id != id);
            }
        }
    }
    fn remove(&mut self, id: u32) {
        let mut items = self.items.write();
        items.retain(|p| p.id != id);
    }
}


// const addToCart = (product, id) => {
//     const newItem = { ...product, amount: 1 };
//     // check if the item is already in the cart
//     const cartItem = cart.find((item) => {
//       return item.id === id;
//     });
//     if (cartItem) {
//       const newCart = [...cart].map((item) => {
//         if (item.id === id) {
//           return { ...item, amount: cartItem.amount + 1 };
//         } else return item;
//       });
//       setCart(newCart);
//     } else {
//       setCart([...cart, newItem]);
//     }
//   };
// struct ApplicationData {
//     first_data: Signal<i32>,
//     second_data: Signal<i32>,
//     many_signals: Signal<Vec<Signal<i32>>>,
// }
// pub struct Cart<'a> {
//     items: Vec<&'a api::Product>,
// }

// impl<'a> Cart<'a> {
//     fn add(&mut self, product: &'a api::Product) {
//         self.items.push(product);
//     }
// }

// impl Cart {
//     fn remove(&mut self, product_id: &str) {
//         self.items.retain(|product| product.id != product_id);
//     }
// }

#[component]
fn App() -> Element {
    // Build cool things ✌️
    // use_context_provider(|| Signal::new(Cart {items: vec![]}));
    use_context_provider(|| Signal::new(Cart::default()));
    use_context_provider(|| Signal::new(SideBarExpanded(false)));
    // use_context_provider(|| Signal::new(Cart.add))

    rsx! {
        // Global app resources
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        Router::<Route> {}
    }
}





