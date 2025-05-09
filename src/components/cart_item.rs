#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_free_icons::{icons::io_icons::{IoAdd,IoClose,IoRemove}, Icon};
use crate::{ Cart, Product };
// use crate::api::Product;


#[component]
pub fn CartItem(product: Product) -> Element {
  let Product {
    id,
    image,
    title,
    price,
    amount,
    ..
  } = product.clone();

  let cart = use_context::<Signal<Cart>>();
 
  rsx! {
    div { class: "flex gap-x-4 py-2 lg:px-6 border-b border-gray-200 w-full font-light text-gray-500",
      div { class: "w-full min-h-[150px] flex items-center gap-x-4",
        {}
        Link { to: "/product/{id}",
          img { class: "max-w-[80px]", src: image, alt: "" }
        }
        div { class: "w-full flex flex-col",
          {}
          div { class: "flex justify-between mb-2",
            {}
            Link {
              to: "/product/{id}",
              class: "text-sm uppercase font-medium max-w-[240px] text-primary hover:underline",
              "{title}"
            }
            {}
            div {
              onclick: move |_| cart().remove(id),
              class: "text-xl cursor-pointer",
              Icon {
                width: 30,
                height: 30,
                fill: "black",
                icon: IoClose,
                class: "text-gray-500 hover:text-red-500 transition",
              }
            }
          }
          div { class: "flex gap-x-2 h-[36px] text-sm",
            {}
            div { class: "flex flex-1 max-w-[100px] items-center h-full border text-primary font-medium",
              div {
                onclick: move |_| cart().decrease_amount(id),
                class: "h-full flex-1 flex justify-center items-center cursor-pointer",
                Icon {
                  width: 30,
                  height: 30,
                  fill: "black",
                  icon: IoRemove,
                }
              }
              div { class: "h-full flex justify-center items-center px-2",
                "{amount}"
              }
              div {
                onclick: move |_| cart().increase_amount(id),
                class: "h-full flex flex-1 justify-center items-center cursor-pointer",
                Icon {
                  width: 30,
                  height: 30,
                  fill: "black",
                  icon: IoAdd,
                }
              }
            }
            {}
            div { class: "flex flex-1 justify-around items-center", "$ {price}" }
            {}
            div { class: "flex flex-1 justify-end items-center text-primary font-medium",
              {format!("$ {:.02}", (price * amount as f32))}
            }
          }
        }
      }
    }
  }
}


