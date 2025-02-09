mod components;
mod models;
pub mod svgs;

use crate::components::header::{Header, HeaderButton};
use crate::components::{header, navbar};
use crate::models::coffee::{CoffeeInstance, Origin, RoastLevel};
use components::{CoffeeMachines, Coffees, Companies, Home};
use dioxus::logger::tracing;
use dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Main {
    #[route("/")]
    Home {},

    #[route("/logs")]
    Companies {},

    #[route("/machines")]
    CoffeeMachines {},
}

const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    tracing::info!("Starting up");
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        div {
            // header::Header {
            //     title: "Coffee Logger",
            //     subtitle: "Keep track of your favorite coffees",
            //     children: {
            //         rsx!{
            //             header::HeaderButton {button_text: "Add Coffee", flex: "flex-none"}
            //             header::HeaderButton {button_text: "Remove Coffee", flex: "flex-auto"}
            //         }
            //     },
            // }
            class: "flex-row flex h-screen",
            navbar::NavBar {
                header_text: "Logger",
                navitems: {
                    rsx!{
                        navbar::NavItem {text: "Logs".to_string(), svg_image: svgs::COFFEE_CUP.rsx(), href: "/logs".to_string() }
                        navbar::NavItem {text: "Machines".to_string(), svg_image: svgs::COFFEE_MACHINE.rsx(), href: "/machines".to_string() }
                    }
                }
            }
            div {
                class: "p-20",
                Router::<Main> {},
            }
        }
    }
}
