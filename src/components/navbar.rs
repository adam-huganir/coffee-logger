use crate::svgs::Svg;
use dioxus::prelude::*;

#[component]
pub fn NavItem(text: String, svg_image: Element, href: String) -> Element {
    rsx! {
        li {
            a {
                class: "group relative flex justify-center rounded-sm px-2 py-1.5 text-gray-500 hover:bg-gray-50 hover:text-gray-700",
                href: "{href}",
                {svg_image}
                span { 
                    class: "invisible absolute start-full top-1/2 ms-4 -translate-y-1/2 rounded-sm bg-gray-900 px-2 py-1.5 text-xs font-medium text-white group-hover:visible",
                    "{text}"
                }
            }
        }
    }
}

#[component]
pub fn NavBar(header_text: String, navitems: Element) -> Element {
    rsx! {

    div { class: "flex h-screen w-16 flex-col justify-between border-e bg-white",
        div {
            div { class: "border-t border-gray-100",
                div { class: "px-2",
                    div { class: "py-4",
                        a {
                            class: "t group relative flex justify-center rounded-sm bg-blue-50 px-2 py-1.5 text-blue-700",
                            href: "#",
                            svg {
                                class: "size-5 opacity-75",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "2",
                                view_box: "0 0 24 24",
                                xmlns: "http://www.w3.org/2000/svg",
                                path {
                                    d: "M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z",
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                }
                                path {
                                    d: "M15 12a3 3 0 11-6 0 3 3 0 016 0z",
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                }
                            }
                            span { class: "invisible absolute start-full top-1/2 ms-4 -translate-y-1/2 rounded-sm bg-gray-900 px-2 py-1.5 text-xs font-medium text-white group-hover:visible",
                                "General"
                            }
                        }
                    }
                    ul { class: "space-y-1 border-t border-gray-100 pt-4",
                        {navitems}
                    }
                }
            }
        }
        div { class: "sticky inset-x-0 bottom-0 border-t border-gray-100 bg-white p-2",
            form { action: "#",
                button {
                    class: "group relative flex w-full justify-center rounded-lg px-2 py-1.5 text-sm text-gray-500 hover:bg-gray-50 hover:text-gray-700",
                    r#type: "submit",
                    svg {}
                    span { class: "invisible absolute start-full top-1/2 ms-4 -translate-y-1/2 rounded-sm bg-gray-900 px-2 py-1.5 text-xs font-medium text-white group-hover:visible",
                        "Home"
                    }
                }
            }
        }
    }
    }
}
