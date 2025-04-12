use crate::Route;
use dioxus::prelude::*;
use dioxus_free_icons::{icons::bs_icons::{BsBag}, Icon};
use dioxus_signals::*;

// const NAVBAR_CSS: Asset = asset!("/assets/styling/navbar.css");
const LOGO: Asset = asset!("/assets/img/logo.svg");

#[component]
pub fn Navbar() -> Element {

    // todo add event listener for scroll.

    // const [isActive, setIsActive] = useState(false);
    // const { isOpen, setIsOpen } = useContext(SidebarContext);
    // const { itemAmount } = useContext(CartContext);
  
    // // event listener
    // useEffect(() => {
    //   window.addEventListener("scroll", () => {
    //     window.scrollY > 60 ? setIsActive(true) : setIsActive(false);
    //   });
    // });
    let item_amount: i32 = 0; // Placeholder for item amount, replace with actual context or state
    // let item_amount = use_state(|| 0);

            // set isActive(isActive);
        // use_signal creates a tracked boolean called is_active
        let mut is_active = use_signal(|| false);

        // use onscroll event listener to change the navbar style
        // use_effect(
            // onscroll(move |_| {
            //     if window().scroll_y() > 60 {
            //         is_active.set(true);
            //     } else {
            //         is_active.set(false);
            //     }
            // });
        // );
        // fn set_is_active(is_active: &Signal<bool>) {
        //     is_active.set(!is_active.get());
        // }
        let onscroll_listener = {
            let is_active = is_active.clone(); // Clone the signal for use in the closure
            move |_: Event<ScrollData>| {
                if web_sys::window().unwrap().scroll_y().unwrap() > 60.0 {
                    is_active.set(true);
                } else {
                    is_active.set(false);
                }
            }
        };
  
    rsx! {
        // document::Link { rel: "stylesheet", href: NAVBAR_CSS }
        header {
            onscroll: onscroll_listener,
            id: "navbar",
            class: "bg-none py-6 fixed w-full z-10 lg:px-8 transition-all",
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
