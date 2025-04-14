#![allow(non_snake_case)]
use crate::Route;
use dioxus::prelude::*;
use dioxus_free_icons::{icons::bs_icons::BsBag, Icon};
// use dioxus_signals::*;
// use gloo::utils::window;
// use web_sys::{Element, EventTarget, HtmlElement, MouseEvent};
// use wasm_bindgen::{
//     closure::Closure,
//     JsCast,
// };
// use dioxus_sdk::utils::scroll::use_root_scroll;
// import scroll_metrics from scroll.rs
use crate::scroll::use_root_scroll;
// const NAVBAR_CSS: Asset = asset!("/assets/styling/navbar.css");
const LOGO: Asset = asset!("/assets/img/logo.svg");


#[component]
pub fn Navbar() -> Element {

    let mut is_active = use_signal(|| false);

    let item_amount: i32 = 0; // Placeholder for item amount, replace with 
    let scroll_metrics = use_root_scroll();
    use_effect(move || {
        let scroll_metrics = scroll_metrics();
        let scroll_value = scroll_metrics.scroll_top;
        // let mut is_active = is_active.clone();
        // let closure = Closure::wrap(Box::new(move || {
        //     let scroll_value = scroll_metrics.scroll_top;
        //     is_active.set(scroll_value > 60.0);
        // }) as Box<dyn FnMut()>);
        // window().add_event_listener_with_callback("scroll", closure.as_ref().unchecked_ref()).unwrap();
        // closure.forget(); // Prevents the closure from being dropped

        is_active.set(scroll_value > 60.0);
    });


    // maybe copy this window.addeventlistener & removeeventlistener from here: https://github.com/DioxusLabs/sdk/blob/a7b261e4fb28c78de894d39284fc0da5dc49c9a4/sdk/src/utils/scroll.rs#L6
    // it's committed to the sdk yesterday, but not yet working when I pull the crate - make a note about how I'm using it here.
    // alternatively use https://github.com/DioxusLabs/dioxus/discussions/2562
    
    rsx! {
        // document::Link { rel: "stylesheet", href: NAVBAR_CSS }
        header {
            // onscroll: move |_| onscroll_listener,
            id: "navbar",
            // class: "bg-none py-6 fixed w-full z-10 lg:px-8 transition-all",
            class: if is_active() {"bg-white py-4 shadow-md fixed w-full z-10 lg:px-8 transition-all"} else {"bg-none py-6 fixed w-full z-10 lg:px-8 transition-all"},
            div { class: "container mx-auto flex items-center justify-between h-full",
                Link { to: Route::Home {},
                    div { class: "w-[40px]",
                        img { src: LOGO }
                    }
                }
                div { class: "cursor-pointer flex relative",
                    Icon {
                        width: 30,
                        height: 30,
                        fill: "black",
                        icon: BsBag,
                    }
                    div { class: "bg-red-500 absolute -right-2 -bottom-2 text-[12px] w-[18px] h-[18px] text-white rounded-full flex justify-center items-center",
                        {item_amount.to_string()}
                    }
                }
            }
        }
        Outlet::<Route> {}
    }
}
