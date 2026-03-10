use dioxus::prelude::*;

use crate::{COOKIE, COOKIES};

#[component]
pub fn Navbar() -> Element {
    rsx! {
        div {
            class: "w-full p-4 border-white", // bg-[#090a0d]
            div {
                class: "p-4 bg-[#090a0d] w-fit rounded-xl flex gap-2 border-white",
                img {
                    class: "w-[1.5em]",
                    src: COOKIE
                },
                h4 {
                    class: "font-bold",
                    "{COOKIES.read()}"
                }
            }
        }
    }
}
