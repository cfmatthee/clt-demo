// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rand::Rng;
use serde::Serialize;
use std::{
    f32::consts::{E, PI},
    iter,
    sync::Mutex,
};
use tauri::State;

#[cfg(debug_assertions)]
use tauri::Manager;

// -----------------------------------------------------------------------------------------------

#[derive(Serialize)]
struct Histogram {
    data: Vec<f32>,
    min: i32,
    max: i32,
    mean: f32,
    stdev: f32,
    guassian: Vec<f32>,
}

#[derive(Debug)]
struct Data {
    data: Vec<i32>,
    min: i32,
    max: i32,
}

struct AppState(Mutex<Data>);

// -----------------------------------------------------------------------------------------------

impl Default for Histogram {
    fn default() -> Self {
        Self {
            data: Vec::new(),
            min: 0,
            max: 0,
            mean: 0.,
            stdev: 0.,
            guassian: Vec::new(),
        }
    }
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
        const SEMI_RANGE: i32 = 5;
        let mut rng = rand::thread_rng();
        for item in self.data.iter_mut() {
            *item += rng.gen_range(-SEMI_RANGE..=SEMI_RANGE);
        }
        self.min -= SEMI_RANGE;
        self.max += SEMI_RANGE;
    }

    fn add_ushaped(&mut self) {
        const SEMI_RANGE: i32 = 5;
        let mut rng = rand::thread_rng();
        for item in self.data.iter_mut() {
            let x: f32 = rng.gen::<f32>();
            let y: f32 = (PI * x - PI / 2.).sin() * (SEMI_RANGE as f32 + 0.5);
            *item += y.max(-5.).min(5.).round() as i32;
        }
        self.min -= SEMI_RANGE;
        self.max += SEMI_RANGE;
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
        let factor: f32 = 1.0 / (self.data.len() as f32);
        let histogram: Vec<f32> = histogram
            .into_iter()
            .map(|x| ((x as f32) * factor))
            .collect();
        let mean: f32 = histogram
            .iter()
            .enumerate()
            .map(|(x, p)| ((x as i32 + self.min) as f32) * p)
            .sum();
        let stdev: f32 = histogram
            .iter()
            .enumerate()
            .map(|(x, p)| (((x as i32 + self.min) as f32) - mean).powf(2.) * p)
            .sum::<f32>()
            .sqrt();
        let factor: f32 = 1. / (stdev * (2. * PI).sqrt());
        let guassian: Vec<f32> = (self.min..=self.max)
            .map(|x| factor * E.powf(-0.5 * (((x as f32) - mean) / stdev).powf(2.)))
            .collect();

        Histogram {
            data: histogram,
            min: self.min,
            max: self.max,
            mean,
            stdev,
            guassian,
        }
    }
}

// -----------------------------------------------------------------------------------------------

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

// -----------------------------------------------------------------------------------------------

fn main() {
    let builder = tauri::Builder::default();

    #[cfg(debug_assertions)]
    let builder = builder.setup(|app| {
        app.get_window("main").unwrap().open_devtools();
        Ok(())
    });

    builder
        .manage(AppState(Mutex::new(Data::new())))
        .invoke_handler(tauri::generate_handler![run_command])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// -----------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::Data;
    use approx::assert_abs_diff_eq;
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
        assert_eq!(data.data.iter().min(), Some(&-5));
        assert_eq!(data.data.iter().max(), Some(&5));
    }

    #[test]
    fn ushaped_range() {
        let mut data = setup();
        data.add_ushaped();
        assert_eq!(data.data.len(), 1_000_000);
        assert_eq!(data.data.iter().min(), Some(&-5));
        assert_eq!(data.data.iter().max(), Some(&5));
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
        assert_eq!(histogram.data.len(), 11);
        assert_abs_diff_eq!(histogram.data.iter().sum::<f32>(), 1.0, epsilon = 0.000001);

        data.add_rectangular();
        let histogram = data.create_histogram();
        assert_eq!(histogram.data.len(), 21);
        assert_abs_diff_eq!(histogram.data.iter().sum::<f32>(), 1.0, epsilon = 0.000001);

        data.add_rectangular();
        let histogram = data.create_histogram();
        assert_eq!(histogram.data.len(), 31);
        assert_abs_diff_eq!(histogram.data.iter().sum::<f32>(), 1.0, epsilon = 0.000001);
    }
}
