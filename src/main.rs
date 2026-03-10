pub mod components;
pub mod layouts;
pub mod pages;

use std::collections::HashMap;

use crate::pages::shop::{ShopItemValues, ShopItems};
use crate::pages::{clicker::ClickerPage, shop::ShopPage};
use crate::{
    components::{menu_button::MenuButtonProps, *},
    layouts::default::DefaultLayout,
};
use dioxus::prelude::*;

// Load css
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

// Load assets
const FAVICON: Asset = asset!("/assets/favicon.ico");
const COOKIE: Asset = asset!("/assets/cookie.png");
const SHOP_ICON: Asset = asset!("/assets/icons/shop.svg");
const BASKETBALL_ICON: Asset = asset!("/assets/icons/basketball.svg");
const ARROW_UP_ICON: Asset = asset!("/assets/icons/fast-arrow-up.svg");

// Signals
static COOKIES: GlobalSignal<usize> = Signal::global(|| 0);
static MENU_ITEMS: GlobalSignal<Vec<MenuButtonProps>> = Signal::global(|| {
    vec![
        MenuButtonProps::new(BASKETBALL_ICON, Route::ClickerPage),
        MenuButtonProps::new(SHOP_ICON, Route::ShopPage),
    ]
});
static SHOP_ITEMS: GlobalSignal<ShopItems> = Signal::global(|| {
    HashMap::from([
        ("Cool click".into(), ShopItemValues::new(100)),
        ("Auto clicker".into(), ShopItemValues::new(300)),
    ])
});

#[derive(Routable, Clone, PartialEq)]
enum Route {
    #[layout(DefaultLayout)]
    #[route("/")]
    ClickerPage,
    #[route("/shop")]
    ShopPage,
}

fn main() {
    dioxus::launch(app);
}

#[component]
fn app() -> Element {
    // Auto clicker
    use_future(move || async move {
        loop {
            tokio::time::sleep(std::time::Duration::from_millis(1000)).await;

            let amount = {
                let items = SHOP_ITEMS.read();

                items
                    .get("Auto clicker")
                    .map(|item| item.purchased)
                    .unwrap_or(0)
            };

            if amount > 0 {
                let mut cookies = COOKIES.write();
                *cookies += amount;
            }
        }
    });

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS } document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
    }
}
