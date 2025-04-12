// use crate::Route;
use dioxus::prelude::*;
use crate::api::{fetch_product, Product};

// const BLOG_CSS: Asset = asset!("/assets/styling/blog.css");

#[component]
pub fn ProductPage(id: u32) -> Element {
    let product = use_server_future(move || fetch_product(id))?;
    let product = product().unwrap()?;

    let Product {
        // id,
        image,
        // category,
        title,
        price,
        description,
        // rating,
        ..
      } = product;
    
    rsx! {
        // document::Link { rel: "stylesheet", href: BLOG_CSS }
        section { class: "pt-[450px] md:pt-32 pb-[400px] md:pb-12 lg:py-32 h-screen flex items-center",
            div { class: "container mx-auto",
                div { class: "flex flex-col lg:flex-row items-center",
                    div { class: "flex flex-1 justify-center items-center mb-8 lg:mb-0",
                        img {
                            class: "max-w-[200px] lg:max-w-xs",
                            src: image,
                            alt: "product image",
                        }
                    }
                    div { class: "flex-1 text-center lg:text-left",
                        h1 { class: "text-[26px] font-medium mb-2 max-w-[450px] mx-auto lg:mx-0",
                            {title}
                        }
                        div { class: "text-2xl text-red-500 font-medium mb-6", "$ {price}" }
                        p { class: "mb-8", "{description}" }
                        button { class: "bg-primary py-4 px-8 text-white", "Add to cart" }
                    }
                }
            }
        }
    }
}
