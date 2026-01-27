use dioxus::prelude::*;
use lib::Histogram;

#[component]
pub fn Chart(histogram: ReadSignal<Histogram>) -> Element {
    const WIDTH: f32 = 500.0;
    const HEIGHT: f32 = 200.0;

    let len = histogram.read().data.len();
    let max_value = histogram
        .read()
        .data
        .iter()
        .fold(0.0_f32, |max, x| max.max(*x));
    let max_value = histogram
        .read()
        .guassian
        .iter()
        .fold(max_value, |max, x| max.max(*x));
    let max_value = max_value * 1.1;

    let bar_width: f32 = WIDTH / (len as f32);
    let bar_height: Vec<_> = histogram
        .read()
        .data
        .iter()
        .map(|y| (y * HEIGHT) / max_value)
        .collect();
    let guassian_points = histogram
        .read()
        .guassian
        .iter()
        .enumerate()
        .map(|(x, y)| {
            (
                ((x as f32) + 0.5) * bar_width,
                HEIGHT - (y * HEIGHT) / max_value,
            )
        })
        .map(|(x, y)| format!("{x:.1},{y:.1}"))
        .collect::<Vec<_>>()
        .join(" ");

    rsx! {
        section {
            flex_grow: 1,
            margin: "10px",
            svg {
                width: "100%",
                height: "auto",
                view_box: format!("0 0 {WIDTH:.0} {HEIGHT:.0}"),
                preserve_aspect_ratio: "none",
                for (idx, value) in bar_height.iter().enumerate() {
                    rect {
                        x: format!("{:.1}", (idx as f32) * bar_width),
                        y: format!("{:.1}", HEIGHT - value),
                        width: format!("{:.1}", bar_width),
                        height: format!("{:.1}", *value),
                        fill: "blue",
                        stroke: "black",
                        stroke_width: "1",
                    }
                }
                if histogram.read().fit < 0.011 && len > 0 {
                    polyline {
                        points: guassian_points,
                        fill: "none",
                        stroke: "red",
                        stroke_width: "2",
                        stroke_dasharray: "5, 5",
                    }
                }
            }
        }
    }
}

// for (idx, window) in guassian.windows(2).enumerate() {
//     line {
//         x1: ((idx as f32) + 0.5) * bar_width,
//         x2: ((idx as f32) + 1.5) * bar_width,
//         y1: HEIGHT - window[0],
//         y2: HEIGHT - window[1],
//         stroke: "red",
//         stroke_width: "2",
//         stroke_dasharray: "10,10",
//     }
// }
