// use crate::Route;
use dioxus::prelude::*;
use crate::api::fetch_product;
use crate::{ Cart, Product };
use crate::api::Product as ApiProduct;

// const BLOG_CSS: Asset = asset!("/assets/styling/blog.css");

#[component]
pub fn ProductPage(id: u32) -> Element {
    let product_resource = use_resource(move || fetch_product(id));
    // check if product_resource returns OK, set product equal to result or error
    let product = match *product_resource.read() {
        Some(Ok(ref product)) => product.clone(),
        Some(Err(_)) => {
            return rsx! {
                div { class: "flex justify-center items-center h-screen",
                    p { class: "text-2xl font-semibold", "Failed to load product." }
                }
            };
        }
        None => {
            return rsx! {
                div { class: "flex justify-center items-center h-screen",
                    p { class: "text-2xl font-semibold", "Loading..." }
                }
            };
        }
    };

    let ApiProduct {
        id,
        image,
        title,
        price,
        description,
        category,
        ..
    } = product.clone();
    // cast to Product struct
    let cast_product = Product {
        id: product.id,
        image: product.image,
        title: product.title,
        price: product.price,
        category: product.category,
        description: product.description,
        amount: 1,
    };

      let cart = use_context::<Signal<Cart>>();
    
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
                        button { 
                            onclick: move |_| { cart().add(cast_product.clone());},
                            class: "bg-primary py-4 px-8 text-white", "Add to cart" 
                        }
                    }
                }
            }
        }
    }
}
