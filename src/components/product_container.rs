use crate::{
  api::{fetch_products, Sort},
  components::product_item::product_item
};
use dioxus::prelude::*;

#[component]
pub fn ProductContainer() -> Element {
  // access product context
  let products = use_server_future(|| fetch_products(10, Sort::Ascending))?;
  let products = products().unwrap()?;

  // filter context 

  rsx! {
    section { class: "py-20",
      div { class: "container mx-auto",
        h1 { class: "text-3xl font-semibold mb-10 text-center", "Explore our products" }
        div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-5 lg:mx-8 gap-[30px] max-w-sm mx-auto md:max-w-none md:mx-0",
          // map over filtered products and return product component
          for product in products {
            product_item { product }
          }
        }
      }
    }
  }
}