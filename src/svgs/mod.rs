use dioxus::prelude::*;
use std::string::ToString;
use std::sync::LazyLock;

pub static COFFEE_CUP: LazyLock<Svg> = LazyLock::new(|| {
    Svg {
        height: "24".to_string(),
        view_box: "0 0 24 24".to_string(),
        width: "24".to_string(),
        xmlns: "http://www.w3.org/2000/svg".to_string(),
        path: SvgPath {
            d: "M11 18q-2.925 0-4.962-2.037T4 11V5q0-.825.588-1.412T6 3h12.5q1.45 0 2.475 1.025T22 6.5t-1.025 2.475T18.5 10H18v1q0 2.925-2.037 4.963T11 18M6 8h10V5H6zm5 8q2.075 0 3.538-1.463T16 11v-1H6v1q0 2.075 1.463 3.538T11 16m7-8h.5q.625 0 1.063-.437T20 6.5t-.437-1.062T18.5 5H18zM4 21v-2h16v2zm7-11".to_string(),
            fill: "currentColor".to_string(),
        }
    }
});

pub static COFFEE_MACHINE: LazyLock<Svg> = LazyLock::new(|| {
    Svg {
        height: "24".to_string(),
        view_box: "0 0 24 24".to_string(),
        width: "24".to_string(),
        xmlns: "http://www.w3.org/2000/svg".to_string(),
        path: SvgPath {
            d: "M6 22q-.825 0-1.412-.587T4 20V4q0-.825.588-1.412T6 2h13q.425 0 .713.288T20 3t-.288.713T19 4h-1v2q0 .425-.288.713T17 7H9q-.425 0-.712-.288T8 6V4H6v16h4.05q-.95-.675-1.5-1.713T8 16v-3q0-.825.588-1.412T10 11h6q.825 0 1.413.588T18 13v3q0 1.25-.55 2.288T15.95 20H19q.425 0 .713.288T20 21t-.288.713T19 22zm7-3q1.25 0 2.125-.875T16 16v-3h-6v3q0 1.25.875 2.125T13 19m0-9q.425 0 .713-.288T14 9t-.288-.712T13 8t-.712.288T12 9t.288.713T13 10m0 3".to_string(),
            fill: "currentColor".to_string(),
        }
    }
});


#[derive(Debug, Clone, PartialEq)]
pub struct SvgPath {
    d: String,
    fill: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Svg {
    height: String,
    view_box: String,
    width: String,
    xmlns: String,
    path: SvgPath,
}

impl Svg {
    pub fn rsx(&self) -> Element {
        rsx! {
            svg {
                height: "{self.height}",
                view_box: "{self.view_box}",
                width: "{self.width}",
                xmlns: "{self.xmlns}",
                path {
                    d: "{self.path.d}",
                    fill: "{self.path.fill}",
                }
            }
        }
    }
}
