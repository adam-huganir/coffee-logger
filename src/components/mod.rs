pub mod header;
pub mod navbar;

use crate::models::coffee::{CoffeeInstance, Origin, RoastLevel};
use crate::TAILWIND_CSS;
use dioxus::prelude::*;
use log::log;
use std::ops::Deref;

#[component]
pub fn Home() -> Element {
    rsx! {
        div {
            input {
                placeholder: "Type here to echo...",
            }
            a {
                href: "/coffees",
                "this is a link"
            }
        }
    }
}

#[component]
pub fn Coffees() -> Element {
    rsx! {
        div {
            "these are where the coffees are"
        }
    }
}

#[component]
pub fn Companies() -> Element {
    rsx! {
        "Companies"
    }
}

#[component]
pub fn CoffeeMachines() -> Element {
    rsx! {
        h1 {
            class: "font-bold text-3xl",
            "Coffee Machines"
        }
    }
}