#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_free_icons::{icons::fi_icons::FiTrash2, icons::io_icons::IoArrowForward, Icon};

#[component]
pub fn Sidebar()-> Element {
  let item_amount: i32 = 0; // Placeholder for item amount, replace with 
  let total: f32 = 0.0; // Placeholder for total, replace with actual value
  // store isopen state in context
  // set class for entire wrapper based on statee

  // access cart context
  rsx! {
    div { class: "w-full bg-white fixed top-0 h-full shadow-2xl md:w-[35vw] lg:w-[40vw] xl:max-w-[30vw] transition-all duration-300 z-20 px-4 lg:px-[35px]",
      div { class:"flex items-center justify-between py-6 border-b",
        div { class: "uppercase text-sm font-semibold", "Shopping Bag ({item_amount})" }
        div {
          // onClick: {handleClose}, 
          class:"cursor-poniter w-8 h-8 flex justify-center items-center",
          Icon {
            width: 30,
            height: 30,
            fill: "black",
            icon: IoArrowForward,
          }
        }
      }
      div { 
        class: "flex flex-col gap-y-2 h-[360px] md:h-[480px] lg:h-[420px] overflow-y-auto overflow-x-hidden border-b",
        "cart placeholder"
        // {cart.map((item) => {
        //   return <CartItem item={item} key={item.id} />;
        // })}
      }
      div { class: "flex flex-col gap-y-3  mt-4",
        div { class:"flex w-full justify-between items-center",
        // {/* total */}
          div {  
            class: "font-semibold",
            span { class: "mr-2", "Subtotal: ${total}" }
          }
          // {/* clear cart icon */}
          div {
            // onClick={clearCart}
            class:"cursor-pointer py-4 bg-red-500 text-white w-12 h-12 flex justify-center items-center text-xl",
            Icon {
              width: 30,
              height: 30,
              fill: "black",
              icon: FiTrash2,
            }
          }
        }
        Link {
          to:{"/"},
          class: "bg-gray-200 flex p-3 justify-center items-center text-primary w-full font-medium",
          "View Cart"
        }
        Link {
          class: "bg-primary flex p-3 justify-center items-center text-white w-full font-medium",
          to: {"/"},
          "Checkout"
        }
      }
    }
  }
}