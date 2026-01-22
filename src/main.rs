use std::sync::{Arc, Mutex};

use dioxus::{logger::tracing, prelude::*};
use lib::Data;

mod chart;
mod controls;

use chart::Chart;
use controls::Controls;

//----------------------------------------------------------------------------

#[derive(Clone)]
pub struct AppState {
    data: Arc<Mutex<Data>>,
}

impl AppState {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn command(&self, cmd: &str) {
        let mut data = self.data.lock().unwrap();
        match cmd {
            "clear" => data.clear(),
            "rectangular" => data.add_rectangular(),
            "ushaped" => data.add_ushaped(),
            _ => panic!("illegal command received"),
        }
        tracing::debug!("{} => {}", cmd, data);
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            data: Arc::new(Mutex::new(Data::new())),
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
    use_context_provider(AppState::new);
    rsx! {
        document::Link { rel: "stylesheet", href: RESET_CSS }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        main {
            Chart {}
            Controls {}
        }
    }
}
