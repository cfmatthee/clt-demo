// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rand::Rng;
use std::{iter, sync::Mutex};
use tauri::State;

#[derive(Debug)]
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
        let n = self.data.len();
        self.data = iter::repeat(0).take(n).collect();
        self.min = 0;
        self.max = 0;
    }

    fn add_rectangular(&mut self) {
        const MAX: u32 = 10;
        let mut rng = rand::thread_rng();
        let n = self.data.len();
        for i in 0..n {
            self.data[i] += rng.gen_range(1..=MAX);
        }
        self.min += 1;
        self.max += MAX;
    }

    fn add_ushaped(&mut self) {
        const MAX: u32 = 10;
        let mut rng = rand::thread_rng();
        let n = self.data.len();
        for i in 0..n {
            let x: f64 = rng.gen::<f64>();
            let y: f64 = (-2.269 * x.powf(3.0) + 3.404 * x.powf(2.0) - 0.1456 * x + 0.006) * 10.0;
            self.data[i] += y.ceil() as u32;
        }
        self.min += 1;
        self.max += MAX;
    }

    fn create_histogram(&self) {
        let capacity: usize = (self.max - self.min + 1) as usize;
        if capacity == 1 {
            return ();
        }
        let mut histogram: Vec<u32> = iter::repeat(0).take(capacity).collect();
        for val in self.data.iter() {
            histogram[(val - self.min) as usize] += 1;
        }
        println!("{:?}", histogram);
    }
}

struct AppState(Mutex<Data>);

#[tauri::command]
fn run_command(command: &str, state: State<AppState>) {
    let mut data = state.0.lock().unwrap();
    match command {
        "clear" => data.clear(),
        "rectangular" => data.add_rectangular(),
        "ushaped" => data.add_ushaped(),
        _ => (),
    };

    data.create_histogram();
}

fn main() {
    tauri::Builder::default()
        .manage(AppState(Mutex::new(Data::new())))
        .invoke_handler(tauri::generate_handler![run_command])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// https://blog.moonguard.dev/manage-state-with-tauri

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
}
