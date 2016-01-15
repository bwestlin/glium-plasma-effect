use time::*;

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
