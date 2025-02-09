use dioxus::prelude::server_fn::response::Res;
use dioxus::prelude::*;
use std::fmt::Display;

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Origin {
    Single(String),
    Blend(Vec<String>),
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum RoastLevel {
    City,
    Light,
    Medium,
    Dark,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Coffee {
    brand: String,
    name: String,
    country: String,
    origin: String,
    kind: Origin,
    roast: RoastLevel,
}

#[component]
pub fn CoffeeInstance(
    name: String,
    brand: String,
    roast_level: RoastLevel,
    kind: Origin,
) -> Element {
    let c = Coffee {
        brand: brand.to_string(),
        name: name.to_string(),
        country: "test".to_string(),
        origin: "test".to_string(),
        kind,
        roast: roast_level,
    };
    rsx! {
        div  {
            class: "h-32 rounded-lg bg-gray-200",
            div {
                class: "sm:hidden",
                label {
                    class: "sr-only",
                    r#for: "Tabs",
                    "Roast level"
                }
                select {
                    id: "Tabs",
                    class: "rounded-md border-gray-200",
                    option {
                        "Settings"
                    }
                    option {
                        "Settings2"
                    }
                }
            }
        }
        div {
            class: "hidden sm:block",
            nav {
                class: "flex gap-6",
                aria_label: "Tabs",
                a {
                    class: "shrink-0 rounded-lg p-2 text-sm font-medium text-gray-500 hover:bg-gray-50 hover:text-gray-700",
                    href: "#",
                    "Settings"
                }
                a {
                    class: "shrink-0 rounded-lg p-2 text-sm font-medium text-gray-500 hover:bg-gray-50 hover:text-gray-700",
                    href: "#",
                    "Settings2"
                }
            }
        }
    }
}

#[component]
pub fn Sidebar() -> Element {
    rsx! {
        div {
            class: "flex h-screen flex-col justify-between border-e bg-white",
            div {
                class: "px-4 py-6",
                span {
                    class: "grid h-10 w-32 place-content-center rounded-lg bg-gray-100 text-xs text-gray-600",
                    "Logo"
                }
            }

            div {
                class: "sticky inset-x-0 bottom-0 border-t border-gray-100",
                a {
                    class: "flex items-center gap-2 bg-white p-4 hover:bg-gray-50",
                    href: "#",
                    img {
                        class: "size-10 rounded-full object-cover",
                        alt: "",
                        src: "https://images.unsplash.com/photo-1600486913747-55e5470d6f40?ixlib=rb-1.2.1&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&w=1770&q=80"
                    }

                    div {
                        p {
                            class: "text-xs",
                            strong {
                                class: "block font-medium",
                                "Eric Frusciante"
                            }

                            span {
                                " eric@frusciante.com "
                            }
                        }
                    }
                }
            }
        }
    }
}
