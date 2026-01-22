use std::sync::{Arc, Mutex};

use dioxus::{logger::tracing, prelude::*};
use lib::{Data, Histogram};

mod chart;
mod controls;

use chart::Chart;
use controls::Controls;

//----------------------------------------------------------------------------

#[derive(Clone)]
pub struct AppState {
    data: Arc<Mutex<Data>>,
    histogram: Signal<Histogram>,
}

impl AppState {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn command(&mut self, cmd: &str) {
        let mut data = self.data.lock().unwrap();
        match cmd {
            "clear" => data.clear(),
            "rectangular" => data.add_rectangular(),
            "ushaped" => data.add_ushaped(),
            _ => panic!("illegal command received"),
        }
        tracing::debug!("{} => {}", cmd, data);

        let histogram = data.create_histogram();
        self.histogram.set(histogram);
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            data: Arc::new(Mutex::new(Data::new())),
            histogram: use_signal(Histogram::default),
        }
    }
}

//----------------------------------------------------------------------------

const RESET_CSS: Asset = asset!("/assets/reset.css");
const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let state = AppState::new();

    let mut copy = state.clone();
    let command = use_callback(move |cmd| copy.command(cmd));

    rsx! {
        document::Link { rel: "stylesheet", href: RESET_CSS }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        main {
            Chart { histogram: state.histogram }
            Controls { handle_command: command }
        }
    }
}
