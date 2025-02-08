mod models;

use dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Main {
    #[route("/")]
    Home {},
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        Header {}
        Router::<Main> {}
    }
}

#[component]
fn Home() -> Element {
    rsx! {
        div {
            input {
                placeholder: "Type here to echo...",
            }
        }
    }
}

#[component]
fn Header() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        div {
            "Welcome to the Page"
        }
    }
}