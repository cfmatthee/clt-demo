use dioxus::{logger::tracing, prelude::*};

const PANEL_CSS: Asset = asset!("/assets/control_panel.css");

#[component]
pub fn Controls() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: PANEL_CSS }
        section {
            class: "control-panel",
            Button {
                onclick: move |_| handle_command("clear"),
                "Clear" }
            Button {
                onclick: move |_| handle_command("rectangular"),
                filled: true,
                "Rectangular"
            }
            Button {
                onclick: move |_| handle_command("ushaped"),
                filled: true,
                "U-shaped"
            }
        }
    }
}

fn handle_command(command: &str) {
    tracing::debug!(command);
}

//----------------------------------------------------------------------------

#[derive(Clone, PartialEq, Props)]
struct ButtonProps {
    #[props(default)]
    filled: bool,
    children: Element,
    #[props(default)]
    onclick: EventHandler<MouseEvent>,
}

#[allow(non_snake_case)]
fn Button(props: ButtonProps) -> Element {
    rsx! {
        button {
            class: if props.filled { "filled" },
            onclick: move |evt| props.onclick.call(evt),
            { props.children }
        }
    }
}
