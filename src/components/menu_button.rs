use dioxus::prelude::*;

use crate::Route;

#[derive(PartialEq, Props, Clone)]
pub struct MenuButtonProps {
    pub icon: Asset,
    #[allow(private_interfaces)]
    pub to: Route,
    #[props(default = false)]
    pub disabled: bool,
}

impl MenuButtonProps {
    #[allow(private_interfaces)]
    pub fn new(icon: Asset, to: Route) -> Self {
        Self {
            icon,
            to,
            disabled: false,
        }
    }
}

#[component]
pub fn MenuButtonComponent(props: MenuButtonProps) -> Element {
    let current_route: Route = use_route();
    let is_disabled = props.disabled || current_route == props.to;

    rsx! {
        if !is_disabled {
            Link {
                to: props.to,
                button {
                    class: "w-12 h-12 p-2 bg-transparent border-2 border-[#e0dfde] cursor-pointer rounded-xl
                            opacity-70 transition-all duration-200
                            hover:opacity-100 hover:border-white
                            active:scale-90",

                    div {
                        class: "w-full h-full bg-[#e0dfde] transition-colors duration-200",

                        style: "
                            mask-image: url({props.icon});
                            mask-size: contain;
                            mask-repeat: no-repeat;
                            mask-position: center;
                        ",

                        class: "hover:bg-white"
                    }
                }
            }
        } else {
            button {
                class: "w-12 h-12 p-2 bg-transparent border-2 border-[#e0dfde] cursor-pointer rounded-xl
                        opacity-100 border-white",

                div {
                    class: "w-full h-full bg-white",

                    style: "
                        mask-image: url({props.icon});
                        mask-size: contain;
                        mask-repeat: no-repeat;
                        mask-position: center;
                    ",
                }
            }
        }
    }
}
