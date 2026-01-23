use dioxus::prelude::*;
use lib::Histogram;

const CHART_CSS: Asset = asset!("/assets/chart.css");

#[component]
pub fn Chart(histogram: ReadSignal<Histogram>) -> Element {
    let max_value = histogram
        .read()
        .data
        .iter()
        .fold(0.0_f32, |max, x| max.max(*x));
    let bar_height: Vec<_> = histogram
        .read()
        .data
        .iter()
        .map(|y| y / max_value)
        .map(|y| format!("{:1}%", y * 100.0))
        .collect();

    rsx! {
        document::Link { rel: "stylesheet", href: CHART_CSS }
        section {
            class: "chart",
            for (idx, value) in bar_height.iter().enumerate() {
                div {
                    key: "{idx}",
                    class: "bar",
                    height: value.as_str(),
                }
            }
        }
    }
}
