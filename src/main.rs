use dioxus::prelude::*;

mod chart;
mod controls;

use chart::Chart;
use controls::Controls;

const RESET_CSS: Asset = asset!("/assets/reset.css");
const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: RESET_CSS }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        main {
            Chart {}
            Controls {}
        }
    }
}
