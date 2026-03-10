use dioxus::prelude::*;

use crate::{menu_button, menu_button::MenuButtonComponent};

#[component]
pub fn MenuComponent(buttons: Vec<menu_button::MenuButtonProps>) -> Element {
    rsx! {
        div {
            class: "mt-10 flex gap-4",
            for button in buttons {
                MenuButtonComponent {
                    key: "{button.to}",
                    ..button
                }
            }
        }
    }
}
