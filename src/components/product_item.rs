use crate::Route;
use dioxus::prelude::*;
use dioxus_free_icons::{icons::bs_icons::{BsEyeFill, BsPlus}, Icon};
// use crate::api::Product;
use crate::{Cart, Product};


#[component]
pub(crate) fn product_item(product: Product) -> Element {
// pub(crate) fn product_item(product: Product) -> Element {
    let Product {
    id,
    image,
    category,
    title,
    price,
    ..
  } = product.clone();
  // let CartItem {
  //   product: Product {
  //     id,
  //     image,
  //     category,
  //     title,
  //     price,
  //     ..
  //   },
  //   amount,
  // } = product.clone();
  let cart = use_context::<Signal<Cart>>();

  rsx! {
    div {
      div { class: "border border-[#e4e4e4] h-[300px] mb-4 relative overflow-hidden group transition",
        div { class: "w-full h-full flex justify-center items-center",
          div { class: "w-[200px] mx-auto flex justify-center items-center",
            img {
              class: "max-h-[160px] group-hover:scale-110 transition duration-300",
              src: image,
              alt: "product image",
            }
          }
        }
        div { class: "absolute top-6 -right-11 group-hover:right-5 p-2 flex flex-col justify-center items-center gap-y-2 opacity-0 group-hover:opacity-100 transition-all duration-300",
          button {
            div { 
              class: "flex justify-center items-center text-white w-12 h-12 bg-teal-500",
              onclick: move |_| cart().add(product.clone()),
              Icon {
                width: 30,
                height: 30,
                fill: "black",
                icon: BsPlus,
              }
            }
            Link {
              to: Route::ProductPage {
                  id: id.clone(),
              },
              class: "w-12 h-12 bg-white flex justify-center items-center text-primary drop-shadow-xl",
              Icon {
                width: 30,
                height: 30,
                fill: "black",
                icon: BsEyeFill,
              }
            }
          }
        }
      }
      div {
        div { class: "tex-sm capitalize text-gray-500 mb-1", {category} }
        Link {
          to: Route::ProductPage {
              id: id.clone(),
          },
          h2 { class: "font-semibold mb-1", "{title}" }
        }
        h2 { class: "font-semibbold", "$ {price}" }
      }
    }
  }
}
