use dioxus::{logger::tracing, prelude::*};
use lib::Histogram;

const CHART_CSS: Asset = asset!("/assets/chart.css");

#[component]
pub fn Chart(histogram: ReadSignal<Histogram>) -> Element {
    tracing::debug!("{histogram}");

    rsx! {
        document::Link { rel: "stylesheet", href: CHART_CSS }
        section {
            class: "chart",
            "Chart goes here"
        }
    }
}
