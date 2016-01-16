use std::f64::consts::PI;
use time::*;

// Measure execution time of a given code in nanoseconds
pub fn timed_ns<F: FnMut()>(mut closure: F) -> u64 {
    let start = precise_time_ns();
    closure();
    precise_time_ns() - start
}

pub struct MathLookup {
    sin_table: Vec<f64>,
    cos_table: Vec<f64>,
    sqrt_table: Vec<f64>
}

impl MathLookup {
    /// Constructs a new `MathLookup`.
    pub fn new() -> MathLookup {
        let mut sin_table = Vec::new();
        for a in 0 .. 360 {
            sin_table.push((a as f64 * PI / 180.0).sin())
        }
        let mut cos_table = Vec::new();
        for a in 0 .. 360 {
            cos_table.push((a as f64 * PI / 180.0).cos())
        }
        let mut sqrt_table = Vec::new();
        for d in 0 .. 2000000 {
            sqrt_table.push((d as f64).sqrt())
        }
        MathLookup {
            sin_table: sin_table,
            cos_table: cos_table,
            sqrt_table: sqrt_table
        }
    }

    /// Calculeate a sinus value using a lookup table
    pub fn sin(&self, a: f64) -> f64 {
        self.sin_table[(a * 180.0 / PI) as usize % 360]
    }

    /// Calculeate a coinus value using a lookup table
    pub fn cos(&self, a: f64) -> f64 {
        self.cos_table[(a * 180.0 / PI) as usize % 360]
    }

    /// Calculeate a square root value using a lookup table
    pub fn sqrt(&self, d: f64) -> f64 {
        self.sqrt_table[d as usize]
    }
}

pub struct TimeSampler {
    n_samples: i32,
    samples: Vec<u64>
}

impl TimeSampler {
    /// Constructs a new `TimeSampler`.
    pub fn new(n_samples: i32) -> TimeSampler {
        TimeSampler {
            n_samples: n_samples,
            samples: Vec::new()
        }
    }

    /// Add a sample for the current time
    pub fn sample(&mut self) {
        self.samples.insert(0, precise_time_ns());
        self.samples.truncate(self.n_samples as usize);
    }

    /// Reset the samples
    pub fn reset(&mut self) {
        self.samples.clear();
    }

    /// Get the latest sample
    pub fn latest(&self) -> u64 {
        match self.samples.first() {
            Some(sample) => *sample,
            None => 0
        }
    }

    /// Caclulate the average time between samples in nanoseconds
    pub fn avg_time_ns(&self) -> u64 {
        if self.samples.len() < 2 { 0 }
        else {
            let (sum, _) = self.samples.iter().rev().fold((0u64, 0u64), |sum_prev, &sample| {
                let (sum, prev) = sum_prev;
                if prev > 0 { (sum + (sample - prev), sample) }
                else { (0, sample) }
            });
            sum / (self.samples.len() - 1) as u64
        }
    }

    /// Caclulate the average sumber of samples per second
    pub fn avg_per_second(&self) -> u64 {
        let ftime = self.avg_time_ns();
        if ftime > 0 { 1000 / (ftime / 1000000) } else { 0 }
    }
}
