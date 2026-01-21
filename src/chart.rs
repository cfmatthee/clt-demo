use dioxus::prelude::*;

const CHART_CSS: Asset = asset!("/assets/chart.css");

#[component]
pub fn Chart() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: CHART_CSS }
        section {
            class: "chart",
            "Chart goes here"
        }
    }
}
