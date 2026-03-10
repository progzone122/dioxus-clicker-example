use std::collections::HashMap;

use dioxus::prelude::*;

use crate::{components::navbar::Navbar, ARROW_UP_ICON, COOKIE, COOKIES, SHOP_ITEMS};

#[derive(PartialEq, Props, Clone)]
pub struct ShopItemValues {
    pub cost: usize,
    pub purchased: usize,
}
impl ShopItemValues {
    pub fn new(cost: usize) -> Self {
        Self { cost, purchased: 0 }
    }
    pub fn up(&mut self) {
        self.purchased += 1;
        self.cost *= 2;
    }
}

pub type ShopItems = HashMap<String, ShopItemValues>;

#[component]
pub fn ShopPage() -> Element {
    rsx! {
        Navbar {  },
        div {
            class: "p-4 flex flex-col items-center gap-6",
            h1 {
                class: "font-bold text-3xl",
                "SHOP"
            }
            div {
                class: "w-full flex flex-col gap-4",
                for (name, item) in SHOP_ITEMS.read().iter() {
                    ShopItem { name, cost: item.cost, purchased: item.purchased }
                }
            }
        }
    }
}

#[component]
pub fn ShopItem(name: String, cost: usize, purchased: usize) -> Element {
    rsx! {
        div {
            class: "h-full w-full flex p-2 bg-[#090a0d] rounded-xl group",
            div {
                class: "w-fit p-3 rounded-xl text-[#e0dfde] flex items-center",
                h4 {
                    class: "w-fit",
                    "{purchased}"
                }
            },
            div {
                class: "w-full p-4 flex flex-col gap-2 rounded-xl text-[#e0dfde]",
                h4 {
                    class: "w-fit",
                    "{name}"
                }
            },
            div { class: "relative w-40 flex items-center justify-end mr-4",
                div {
                    class: "flex items-center gap-2 text-[#e0dfde] group-hover:hidden transition-all",
                    h4 { "{cost}" }
                    img { class: "w-[1.5em]", src: COOKIE }
                }

                button {
                    class: "hidden group-hover:flex items-center justify-center bg-orange-500 px-4 py-2 rounded-lg text-white font-bold transition-all duration-100 active:scale-80",
                    onclick: move |_| buy(&name, cost),
                        img {
                            class: "w-[1.5em]",
                            src: ARROW_UP_ICON,
                        }
                    }
                }
        }
    }
}

fn buy(name: &str, cost: usize) {
    if *COOKIES.read() >= cost {
        *COOKIES.write() -= cost;
        SHOP_ITEMS.write().get_mut(name).map(|item| item.up());
    }
}
