#![allow(non_snake_case)]
use crate::Route;
use dioxus::prelude::*;
use dioxus_free_icons::{icons::bs_icons::BsBag, Icon};

use crate::scroll::use_root_scroll;
const LOGO: Asset = asset!("/assets/img/logo.svg");
use crate::SideBarExpanded;
use crate::Cart;


#[component]
pub fn Navbar() -> Element {

    let mut is_active = use_signal(|| false);
    let item_amount = use_context::<Signal<Cart>>()().items.len();
    let scroll_metrics = use_root_scroll();
    let mut sidebar_expanded = use_context::<Signal<SideBarExpanded>>();

    use_effect(move || {
        let scroll_metrics = scroll_metrics();
        let scroll_value = scroll_metrics.scroll_top;
        is_active.set(scroll_value > 60.0);
    });

    rsx! {

        header {
            id: "navbar",
            class: if is_active() { "bg-white py-4 shadow-md fixed w-full z-10 lg:px-8 transition-all" } else { "bg-none py-6 fixed w-full z-10 lg:px-8 transition-all" },
            div { class: "container mx-auto flex items-center justify-between h-full",
                Link { to: Route::Home {},
                    div { class: "w-[40px]",
                        img { src: LOGO }
                    }
                }
                div {
                    class: "cursor-pointer flex relative",
                    onclick: move |_| {
                        sidebar_expanded.set(SideBarExpanded(!sidebar_expanded().0));
                    },
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
