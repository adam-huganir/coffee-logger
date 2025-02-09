use dioxus::prelude::*;


#[component]
pub fn HeaderButton(button_text: String, flex: String) -> Element {
    rsx! {
        div {
            class: "{flex} p-2 rounded-lg shadow-md border-black bg-gray-100 hover:bg-gray-200",
            button {
                class: "",
                r#type: "button",
                span {
                    class: "text-sm font-medium",
                    {button_text}
                }
            }
        }
    }
}

#[component]
pub fn Header(title: String, subtitle: Option<String>, children: Element) -> Element {
    rsx! {
        header {
            class: "border-b border-gray-200",
            div {
                class: "mx-auto max-w-screen-xl px-2 py-4 sm:px-6 sm:py-12 lg:px-8",
                div {
                    class: "flex flex-col items-start gap-4 md:flex-row md:items-center md:justify-between",
                    div {
                        h1 {
                            class: "text-xl font-bold text-gray-900 sm:text-xl",
                            {title}
                        }

                        p {
                            class: "mt-1.5 text-sm text-gray-500",
                            {subtitle}
                        }
                    }

                    div {
                        class: "flex items-center gap-4",
                        {children}
                    }
                }
            }
        }
    }
}
