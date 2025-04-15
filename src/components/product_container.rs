use crate::{
  Product,
  api::{fetch_products, Sort, Product as ApiProduct},
  components::product_item::product_item
};
use dioxus::prelude::*;

#[component]
pub fn ProductContainer() -> Element {
  // access product context
  let products = use_server_future(|| fetch_products(20, Sort::Ascending))?;
  let products = products().unwrap()?
    .into_iter()
    .map(|api_product: ApiProduct| Product {
      // Map fields from `api::Product` to `Product`
      id: api_product.id,
      price: api_product.price,
      description: api_product.description,
      category: api_product.category,
      title: api_product.title,
      image: api_product.image,
      amount: 1, 
    }).filter(|product| {
      // Filter on clothing categories
     product.category == "men's clothing" || product.category == "women's clothing" || product.category == "jewelery"
    })
    .collect::<Vec<Product>>();

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