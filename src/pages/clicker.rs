use dioxus::prelude::*;

use crate::{COOKIE, COOKIES, SHOP_ITEMS};

#[component]
pub fn ClickerPage() -> Element {
    rsx! {
        div {
            class: "h-screen w-full flex flex-col items-center justify-center gap-10",
            div {
                class: "flex flex-col gap-10",
                div {
                    class: "w-full flex justify-center text-4xl font-bold",
                    h1 {
                        "{COOKIES.read()}"
                    }
                }
                div {
                    class: "w-full flex justify-center",
                    img {
                        class: "w-[60%] sm:w-[60%] md:w-80 cursor-pointer transition-transform duration-75 active:scale-90",
                        draggable: "false",
                        src: COOKIE,
                        onclick: move |_| click(),
                    }
                }
            }
        }
    }
}

fn click() {
    let amount = {
        let items = SHOP_ITEMS.read();

        items
            .get("Cool click")
            .map(|item| item.purchased)
            .unwrap_or(0)
    };

    let mut cookies = COOKIES.write();
    *cookies += amount + 1;
}
