extern crate glium;
extern crate glium_sdl2;
extern crate sdl2;
extern crate time;

mod util;
mod plasma;

use glium_sdl2::DisplayBuild;
use glium::Surface;
use time::*;

use util::*;
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
    let n_pixels = b_width * b_width;

    // The renderer of the plasma effect
    let mut plasma = Plasma::new(b_width, b_height);

    // Target buffer for for the plasma effect
    let mut e_buffer: Vec<(u8, u8, u8, u8)> = Vec::with_capacity(n_pixels as usize);
    e_buffer.resize(n_pixels as usize, (0u8, 0u8, 0u8, 0u8));

    // Target pixel buffer for the effect
    let p_buffer = glium::texture::pixel_buffer::PixelBuffer::new_empty(&display, n_pixels as usize);
    // Target texture for the effect
    let texture = glium::texture::Texture2d::empty(&display, b_width, b_height).unwrap();

    let mut t_sampler = TimeSampler::new(1000);
    let start_time_ns = precise_time_ns();
    let mut last_stats_time_ns = start_time_ns;
    let stats_interval_ns = 1000000000; // Stats interval is 1 second

    while running {
        t_sampler.sample();

        // Perform rendering of plasa effect
        let render_time_ns = timed_ns(|| {
            plasma.render(&mut e_buffer, precise_time_ns() - start_time_ns);
        });

        // Display it on screen
        let display_time_ns = timed_ns(|| {
            let mut target = display.draw();
            target.clear_color(0.0, 0.0, 0.0, 1.0);
            p_buffer.write(&e_buffer);
            texture.main_level().raw_upload_from_pixel_buffer(p_buffer.as_slice(), 0 .. b_width, 0 .. b_height, 0 .. 1);
            texture.as_surface().fill(&target, glium::uniforms::MagnifySamplerFilter::Linear);
            target.finish().unwrap();
        });

        // Show some statistics
        if precise_time_ns() - last_stats_time_ns > stats_interval_ns {
            let avg_fps = t_sampler.avg_per_second();
            println!("avg_fps={}, render_time_ms={:.5}, display_time_ms={:.1}",
                avg_fps, render_time_ns as f64 / 1000000.0, display_time_ns as f64 / 1000000.0);
            last_stats_time_ns = last_stats_time_ns + stats_interval_ns;
            t_sampler.reset();
        }

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
