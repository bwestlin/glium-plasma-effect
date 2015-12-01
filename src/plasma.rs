use std::cmp;
use std::f64::consts::PI;

pub struct MathLookup {
    sin_table: Vec<f64>,
    cos_table: Vec<f64>,
    sqrt_table: Vec<f64>
}

impl MathLookup {
    fn new() -> MathLookup {
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

    fn sin(&self, a: f64) -> f64 {
        self.sin_table[(a * 180.0 / PI) as usize % 360]
    }

    fn cos(&self, a: f64) -> f64 {
        self.cos_table[(a * 180.0 / PI) as usize % 360]
    }

    fn sqrt(&self, d: f64) -> f64 {
        self.sqrt_table[d as usize]
    }
}

pub struct Plasma {
    width: u32,
    height: u32,
    time: f64,
    ml: MathLookup
}

impl Plasma {
    pub fn new(w: u32, h: u32) -> Plasma {
        Plasma {
            width: w,
            height: h,
            time: 0.0,
            ml: MathLookup::new()
        }
    }

    pub fn render(&mut self, buf: &mut Vec<(u8, u8, u8, u8)>, dt_ns: u64) {

        let time = dt_ns as f64 / 1000000000.0;
        let w = self.width;
        let h = self.height;

        let sin = |a: f64| self.ml.sin(a);
        let cos = |a: f64| self.ml.cos(a);
        let dist = |x: f64, y: f64, w: f64, h: f64| -> f64 {
            self.ml.sqrt((x - w / 2.0) * (x - w / 2.0) + (y - h / 2.0) * (y - h / 2.0))
        };

        let c_mul = 16.5 + sin(time / 2.0) * 15.5;

        for y in 0..h {
            for x in 0..w {
                let fx = x as f64;
                let fy = y as f64;

                let value =
                    sin(dist(fx + time, fy, 128.0 * sin(time), 128.0 * sin(time)) / 8.0) +
                    sin(dist(fx, fy, 64.0 * cos(time), 64.0 * cos(time)) / 16.0) +
                    sin(dist(fx, fy + time / 7.0, 192.0, 64.0) / 17.0) +
                    sin(dist(fx, fy, 192.0 + 200.0 + sin(time) * 200.0, 100.0 + 200.0 + sin(time / 2.0) * 200.0) / 8.0);

                let color = (((4.0 + value) * (32.0 / c_mul)).floor() * c_mul) as u32;

                let p_pos = (((h - y - 1) * w) + (x)) as usize;
                buf[p_pos] = (
                    (cmp::min(color << 1, 255)) as u8,
                    color as u8,
                    (255 - color) as u8,
                    255u8
                );
            }
        }
    }
}
