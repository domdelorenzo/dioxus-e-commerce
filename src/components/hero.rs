use crate::Route;
use dioxus::prelude::*;

// const HEADER_SVG: Asset = asset!("/assets/header.svg");
const BGD_IMG: Asset = asset!("/assets/img/bghero.jpg");

#[component]
pub fn Hero() -> Element {
    // rsx! {
    //     div {
    //         id: "hero",
    //         img { src: HEADER_SVG, id: "header" }
    //         div { id: "links",
    //             a { href: "https://dioxuslabs.com/learn/0.6/", "📚 Learn Dioxus" }
    //             a { href: "https://dioxuslabs.com/awesome", "🚀 Awesome Dioxus" }
    //             a { href: "https://github.com/dioxus-community/", "📡 Community Libraries" }
    //             a { href: "https://github.com/DioxusLabs/sdk", "⚙️ Dioxus Development Kit" }
    //             a { href: "https://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus", "💫 VSCode Extension" }
    //             a { href: "https://discord.gg/XgGxMSkvUM", "👋 Community Discord" }
    //         }
    //     }
    // }
    rsx! {
        section {
            class: "h-[800px] bg-hero bg-no-repeat bg-cover bg-center py-20",
            style: "background-image: url({BGD_IMG})",
            div { class: "container mx-auto flex justify-around h-full",
                div { class: "flex flex-col justify-center",
                    div { class: "font-semibold flex items-center uppercase", "Hot Trends" }
                    h1 { class: "uppercase text-[55px] md:text-[70px] leading-[1.1] font-semibold mb-4",
                        "Fresh Fashion Finds"
                        br {}
                        span { class: "font-light", "New Collection" }
                    }
                    Link {
                        class: "self-start uppercase font-semibold border-b-2 border-primary",
                        to: Route::Home {},
                        "Discover More"
                    }
                }
            }
        }
    }
}
