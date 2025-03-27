use std::{
    f32::consts::{E, PI},
    iter,
};

use rand::Rng;
use serde::Serialize;

#[derive(Serialize)]
pub struct Histogram {
    pub data: Vec<f32>,
    pub min: i32,
    pub max: i32,
    pub mean: f32,
    pub stdev: f32,
    pub guassian: Vec<f32>,
}

#[derive(Debug)]
pub struct Data {
    pub data: Vec<i32>,
    pub min: i32,
    pub max: i32,
}

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
    pub fn new() -> Self {
        Data::default()
    }

    pub fn clear(&mut self) {
        self.data.fill(0);
        self.min = 0;
        self.max = 0;
    }

    pub fn add_rectangular(&mut self) {
        const SEMI_RANGE: i32 = 5;
        let mut rng = rand::rng();
        for item in self.data.iter_mut() {
            *item += rng.random_range(-SEMI_RANGE..=SEMI_RANGE);
        }
        self.min -= SEMI_RANGE;
        self.max += SEMI_RANGE;
    }

    pub fn add_ushaped(&mut self) {
        const SEMI_RANGE: i32 = 5;
        let mut rng = rand::rng();
        for item in self.data.iter_mut() {
            let x: f32 = rng.random::<f32>();
            let y: f32 = (PI * x - PI / 2.).sin() * (SEMI_RANGE as f32 + 0.5);
            *item += y.clamp(-5.0, 5.0).round() as i32;
        }
        self.min -= SEMI_RANGE;
        self.max += SEMI_RANGE;
    }

    pub fn create_histogram(&self) -> Histogram {
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
