use std::iter::zip;

pub trait Wave: Clone {
    fn new() -> Self;
    fn with_capacity(capacity: usize) -> Self;
    fn zeros(length: usize) -> Self;
    fn ones(length: usize) -> Self;
    fn from_vec(vec: Vec<f64>) -> Self;
    fn resize(&mut self, new_len: usize, value: f64);
    fn clear(&mut self);

    fn add(&mut self, other: &Self, index: usize);
    fn add_consuming(&mut self, other: Self, index: usize);

    fn scale(&mut self, value: f64);
    fn scale_by_vec(&mut self, vec: Vec<f64>);
    fn mult(&mut self, other: &Self);
    fn mult_consuming(&mut self, other: Self);

    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;

    fn normalize(&mut self);
}

#[derive(Clone)]
pub struct Mono {
    wave: Vec<f64>,
}

impl Mono {
    pub fn get_vec(&self) -> Vec<f64> {
        self.wave.clone()
    }
}

impl Wave for Mono {
    fn new() -> Self {
        Self { wave: Vec::new() }
    }

    fn with_capacity(capacity: usize) -> Self {
        Self {
            wave: Vec::with_capacity(capacity),
        }
    }

    fn zeros(length: usize) -> Self {
        Self {
            wave: vec![0.0; length],
        }
    }

    fn ones(length: usize) -> Self {
        Self {
            wave: vec![1.0; length],
        }
    }

    fn clear(&mut self) {
        self.wave = Vec::new()
    }

    fn from_vec(vec: Vec<f64>) -> Self {
        Self { wave: vec }
    }

    fn scale(&mut self, value: f64) {
        self.wave = self.wave.iter().map(|x| x * value).collect()
    }

    fn add_consuming(&mut self, other: Self, index: usize) {
        if index == 0 && self.len() == other.len() {
            for (e1, e2) in zip(&mut self.wave, other.wave) {
                *e1 += e2;
            }
        } else {
            if self.len() < index + other.len() {
                self.wave.resize(index + other.len(), 0.0)
            }
            for i in 0..other.len() {
                self.wave[i + index] += other.wave[i];
            }
        }
    }

    fn add(&mut self, other: &Self, index: usize) {
        if index == 0 && self.len() == other.len() {
            for (e1, e2) in zip(&mut self.wave, other.wave.iter()) {
                *e1 += e2;
            }
        } else {
            if self.len() < index + other.len() {
                self.resize(index + other.len(), 0.0)
            }
            for i in 0..other.len() {
                self.wave[i + index] += other.wave[i];
            }
        }
    }
    fn scale_by_vec(&mut self, vec: Vec<f64>) {
        debug_assert_eq!(self.len(), vec.len());
        for (e1, e2) in zip(&mut self.wave, vec.into_iter()) {
            *e1 *= e2;
        }
    }

    fn mult_consuming(&mut self, other: Self) {
        debug_assert_eq!(self.len(), other.len());
        for (e1, e2) in zip(&mut self.wave, other.wave) {
            *e1 *= e2;
        }
    }
    fn mult(&mut self, other: &Self) {
        debug_assert_eq!(self.len(), other.len());
        for (e1, e2) in zip(&mut self.wave, other.wave.iter()) {
            *e1 *= e2;
        }
    }

    fn len(&self) -> usize {
        self.wave.len()
    }

    fn is_empty(&self) -> bool {
        self.wave.is_empty()
    }

    fn normalize(&mut self) {
        // TODO bad code
        let norm = (self.wave.iter().map(|x| x * x).fold(0.0, |acc, x| acc + x)
            / (self.wave.len() as f64))
            .sqrt();
        self.wave = self.wave.iter().map(|x| x / norm).collect();
    }

    fn resize(&mut self, new_len: usize, value: f64) {
        self.wave.resize(new_len, value)
    }
}