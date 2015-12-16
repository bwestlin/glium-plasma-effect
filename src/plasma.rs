use std::cmp;

use util::MathLookup;

pub struct Plasma {
    width: u32,
    height: u32,
    ml: MathLookup
}

impl Plasma {
    /// Constructs a new `Plasma`.
    pub fn new(w: u32, h: u32) -> Plasma {
        Plasma {
            width: w,
            height: h,
            ml: MathLookup::new()
        }
    }

    // Render the plasma effect on a buffer
    pub fn render(&self, buf: &mut [(u8, u8, u8, u8)], dt_ns: u64, y_offset: u32, y_height: u32) {

        let time = dt_ns as f64 / 1000000000.0;
        let w = self.width;
        let h = self.height;

        let sin = |a: f64| self.ml.sin(a);
        let cos = |a: f64| self.ml.cos(a);
        let dist = |x: f64, y: f64, w: f64, h: f64| -> f64 {
            //self.ml.sqrt((x - w / 2.0) * (x - w / 2.0) + (y - h / 2.0) * (y - h / 2.0))
            ((x - w / 2.0) * (x - w / 2.0) + (y - h / 2.0) * (y - h / 2.0)).sqrt()
        };

        let c_mul = 16.5 + sin(time / 2.0) * 15.5;

        for y in y_offset..(y_offset + y_height) {
            for x in 0..w {
                let fx = x as f64;
                let fy = y as f64;

                let value =
                    sin(dist(fx + time, fy, 128.0 * sin(time), 128.0 * sin(time)) / 8.0) +
                    sin(dist(fx, fy, 64.0 * cos(time), 64.0 * cos(time)) / 16.0) +
                    sin(dist(fx, fy + time / 7.0, 192.0, 64.0) / 17.0) +
                    sin(dist(fx, fy, 192.0 + 200.0 + sin(time) * 200.0, 100.0 + 200.0 + sin(time / 2.0) * 200.0) / 8.0);

                let color = (((4.0 + value) * (32.0 / c_mul)).floor() * c_mul) as u32;

                let p_pos = (((y_height - (y - y_offset) - 1) * w) + (x)) as usize;
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
