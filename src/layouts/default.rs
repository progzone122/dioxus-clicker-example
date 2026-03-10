use crate::{components::menu::MenuComponent, Route, MENU_ITEMS};
use dioxus::prelude::*;

#[component]
pub fn DefaultLayout() -> Element {
    let menu_buttons = MENU_ITEMS.read();

    rsx! {
        div { class: "main select-none h-screen overflow-hidden",
            div { class: "flex flex-col h-full relative px-0 sm:px-28 md:px-52 lg:px-82 xl:px-120",

                div { class: "flex-1 flex flex-col overflow-y-auto",
                    Outlet::<Route> {}
                }

                div {
                    class: "fixed bottom-0 left-0 right-0 flex justify-center p-6",
                    MenuComponent { buttons: menu_buttons.clone() }
                }
            }
        }
    }
}
