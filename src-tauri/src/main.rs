// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rand::Rng;
use serde::Serialize;
use std::{iter, sync::Mutex};
use tauri::State;

#[derive(Serialize)]
struct Histogram {
    data: Vec<u32>,
    min: u32,
    max: u32,
}

impl Default for Histogram {
    fn default() -> Self {
        Self {
            data: Vec::new(),
            min: 0,
            max: 0,
        }
    }
}

struct Data {
    data: Vec<u32>,
    min: u32,
    max: u32,
}

impl Default for Data {
    fn default() -> Self {
        Self {
            data: iter::repeat(0).take(1_000_000).collect(),
            min: 0,
            max: 0,
        }
    }
}

impl Data {
    fn new() -> Self {
        Data::default()
    }

    fn clear(&mut self) {
        self.data.fill(0);
        self.min = 0;
        self.max = 0;
    }

    fn add_rectangular(&mut self) {
        const MAX: u32 = 10;
        let mut rng = rand::thread_rng();
        for item in self.data.iter_mut() {
            *item += rng.gen_range(1..=MAX);
        }
        self.min += 1;
        self.max += MAX;
    }

    fn add_ushaped(&mut self) {
        const MAX: u32 = 10;
        let mut rng = rand::thread_rng();
        for item in self.data.iter_mut() {
            let x: f64 = rng.gen::<f64>();
            let y: f64 = (-2.269 * x.powf(3.0) + 3.404 * x.powf(2.0) - 0.1456 * x + 0.006) * 10.0;
            *item += y.ceil() as u32;
        }
        self.min += 1;
        self.max += MAX;
    }

    fn create_histogram(&self) -> Histogram {
        let capacity: usize = (self.max - self.min + 1) as usize;
        if capacity == 1 {
            return Histogram::default();
        }
        let mut histogram: Vec<u32> = iter::repeat(0).take(capacity).collect();
        for val in self.data.iter() {
            histogram[(val - self.min) as usize] += 1;
        }
        Histogram {
            data: histogram,
            min: self.min,
            max: self.max,
        }
    }
}

struct AppState(Mutex<Data>);

#[tauri::command]
fn run_command(command: &str, state: State<AppState>) -> Histogram {
    let mut data = state.0.lock().unwrap();
    match command {
        "clear" => data.clear(),
        "rectangular" => data.add_rectangular(),
        "ushaped" => data.add_ushaped(),
        _ => (),
    };

    data.create_histogram()
}

fn main() {
    tauri::Builder::default()
        .manage(AppState(Mutex::new(Data::new())))
        .invoke_handler(tauri::generate_handler![run_command])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use super::Data;
    use std::iter;

    fn setup() -> Data {
        Data {
            data: iter::repeat(0).take(1_000_000).collect(),
            min: 0,
            max: 0,
        }
    }

    #[test]
    fn rectangular_range() {
        let mut data = setup();
        data.add_rectangular();
        assert_eq!(data.data.len(), 1_000_000);
        assert_eq!(data.data.iter().min(), Some(&1));
        assert_eq!(data.data.iter().max(), Some(&10));
    }

    #[test]
    fn ushaped_range() {
        let mut data = setup();
        data.add_ushaped();
        assert_eq!(data.data.len(), 1_000_000);
        assert_eq!(data.data.iter().min(), Some(&1));
        assert_eq!(data.data.iter().max(), Some(&10));
    }

    #[test]
    fn clear_data() {
        let mut data = setup();
        data.add_rectangular();
        data.clear();
        assert_eq!(data.data.len(), 1_000_000);
        assert_eq!(data.data.iter().min(), Some(&0));
        assert_eq!(data.data.iter().max(), Some(&0));
        assert!(data.data.iter().all(|&x| x == 0));
    }

    #[test]
    fn histogram() {
        let mut data = setup();
        assert!(data.create_histogram().data.is_empty());

        data.add_rectangular();
        let histogram = data.create_histogram();
        assert_eq!(histogram.data.len(), 10);

        data.add_rectangular();
        let histogram = data.create_histogram();
        assert_eq!(histogram.data.len(), 19);

        data.add_rectangular();
        let histogram = data.create_histogram();
        assert_eq!(histogram.data.len(), 28);
    }
}
