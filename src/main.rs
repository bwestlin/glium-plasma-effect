extern crate glium;
extern crate glium_sdl2;
extern crate sdl2;
extern crate time;
extern crate scoped_threadpool;
extern crate num_cpus;

mod plasma;

use glium_sdl2::DisplayBuild;
use glium::Surface;
use time::*;
use std::thread;

use plasma::Plasma;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let width = 480 * 2;
    let height = 320 * 2;

    let display = video_subsystem.window("Glium Plasma", width, height)
        .resizable()
        .build_glium()
        .unwrap();

    let mut running = true;
    let mut event_pump = sdl_context.event_pump().unwrap();

    let b_width = 480 as u32;
    let b_height = 320 as u32;
    let n_pixels = b_width * b_height;
    let mut e_buffer: Vec<(u8, u8, u8, u8)> = Vec::with_capacity(n_pixels as usize);
    e_buffer.resize(n_pixels as usize, (0u8, 0u8, 0u8, 0u8));
    let e_buf_len = e_buffer.len();

    let plasma = &(Plasma::new(b_width, b_height));

    let texture = glium::texture::Texture2d::empty(&display, b_width, b_height).unwrap();
    let p_buffer = glium::texture::pixel_buffer::PixelBuffer::new_empty(&display, n_pixels as usize);

    let mut fps_counter = FpsCounter::new();
    let start_time_ns = precise_time_ns();

    let n_chunks = num_cpus::get() as u32 * 2;
    println!("rendering {} chunks in parallel", n_chunks);
    let mut pool = scoped_threadpool::Pool::new(n_chunks);

    while running {
        fps_counter.frame_time(precise_time_ns());
        let avg_fps = fps_counter.avg_fps();
        println!("avg_fps={}", avg_fps);
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);

        let delta_t = precise_time_ns() - start_time_ns;

        pool.scoped(|scope| {
            for (i, eb_slice) in e_buffer.chunks_mut(e_buf_len / n_chunks as usize).enumerate() {
                scope.execute(move || plasma.render(eb_slice, delta_t, b_height - b_height / n_chunks * i as u32, b_height / n_chunks));
            }
        });

        //plasma.render(&mut e_buffer, precise_time_ns() - start_time_ns);
        p_buffer.write(&e_buffer);
        texture.main_level().raw_upload_from_pixel_buffer(p_buffer.as_slice(), 0 .. b_width, 0 .. b_height, 0 .. 1);
        texture.as_surface().fill(&target, glium::uniforms::MagnifySamplerFilter::Linear);

        target.finish().unwrap();

        // Event loop: includes all windows
        for event in event_pump.poll_iter() {
            use sdl2::event::Event;
            use ::sdl2::keyboard::Keycode::*;

            match event {
                Event::Quit { .. } => {
                    running = false;
                },
                Event::KeyDown { keycode: Some(Escape), .. } => {
                    running = false;
                },
                _ => ()
            }
        }
    }
}

struct FpsCounter {
    n_samples: i32,
    time_samples: Vec<u64>
}

impl FpsCounter {
    fn new() -> FpsCounter {
        FpsCounter {
            n_samples: 100,
            time_samples: Vec::new()
        }
    }

    fn frame_time(&mut self, sample: u64) {
        self.time_samples.insert(0, sample);
        self.time_samples.truncate(self.n_samples as usize);
    }

    fn latest(&self) -> u64 {
        match self.time_samples.first() {
            Some(sample) => *sample,
            None => 0
        }
    }

    fn avg_ftime_ns(&self) -> u64 {
        if self.time_samples.len() < 2 { 0 }
        else {
            let (sum, _) = self.time_samples.iter().rev().fold((0u64, 0u64), |sum_prev, &sample| {
                let (sum, prev) = sum_prev;
                if prev > 0 { (sum + (sample - prev), sample) }
                else { (0, sample) }
            });
            sum / (self.time_samples.len() - 1) as u64
        }
    }

    fn avg_fps(&self) -> u64 {
        let ftime = self.avg_ftime_ns();
        if ftime > 0 { 1000 / (ftime / 1000000) } else { 0 }
    }
}
