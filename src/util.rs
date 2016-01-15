use time::*;

pub struct TimeSampler {
    n_samples: i32,
    samples: Vec<u64>
}

impl TimeSampler {
    pub fn new(n_samples: i32) -> TimeSampler {
        TimeSampler {
            n_samples: n_samples,
            samples: Vec::new()
        }
    }

    pub fn sample(&mut self) {
        self.samples.insert(0, precise_time_ns());
        self.samples.truncate(self.n_samples as usize);
    }

    pub fn reset(&mut self) {
        self.samples.clear();
    }

    pub fn latest(&self) -> u64 {
        match self.samples.first() {
            Some(sample) => *sample,
            None => 0
        }
    }

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

    pub fn avg_per_second(&self) -> u64 {
        let ftime = self.avg_time_ns();
        if ftime > 0 { 1000 / (ftime / 1000000) } else { 0 }
    }
}
