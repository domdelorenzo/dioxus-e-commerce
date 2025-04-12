

#[component]
pub fn sidebar()-> Element {
  // store isopen state in context
  // set class for entire wrapper based on statee

  // access cart context
  rsx! {
    div { class: "w-full bg-white fixed top-0 h-full shadow-2xl md:w-[35vw] lg:w-[40vw] xl:max-w-[30vw] transition-all duration-300 z-20 px-4 lg:px-[35px]",
      div { class:"flex items-center justify-between py-6 border-b",
      div { class: "uppercase text-sm font-semibold", "Shopping Bag ({itemAmount})" }
      div {
        onClick: {handleClose}, 
        class:"cursor-poniter w-8 h-8 flex justify-center items-center",
        Icon {
          width: 30,
          height: 30,
          fill: "black",
          icon: IoMdArrowForward,
        }
      }
    }
  div { class: "flex flex-col gap-y-2 h-[360px] md:h-[480px] lg:h-[420px] overflow-y-auto overflow-x-hidden border-b"
    // {cart.map((item) => {
    //   return <CartItem item={item} key={item.id} />;
    // })}
  }
  div { class: "flex flex-col gap-y-3  mt-4",
    div { class:"flex w-full justify-between items-center",
      {/* total */}
      div {  class: "font-semibold",
        span { class: "mr-2", "Subtotal: ${parseFloat(total).toFixed(2)}" }
      {/* clear cart icon */}
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
