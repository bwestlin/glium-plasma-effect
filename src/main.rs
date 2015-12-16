extern crate glium;
extern crate glium_sdl2;
extern crate sdl2;
extern crate time;
extern crate scoped_threadpool;

mod util;
mod plasma;

use glium_sdl2::DisplayBuild;
use glium::Surface;
use time::*;
use std::thread;

use util::*;
use plasma::Plasma;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let b_width = 480 as u32;
    let b_height = 320 as u32;
    let n_pixels = b_width * b_height;

    let scr_width = b_width * 2;
    let scr_height = b_height * 2;

    let display = video_subsystem.window("Glium Plasma", scr_width, scr_height)
        .resizable()
        .build_glium()
        .unwrap();

    let mut running = true;
    let mut event_pump = sdl_context.event_pump().unwrap();

    // The renderer of the plasma effect
    let plasma = &(Plasma::new(b_width, b_height));

    // Target buffer for for the plasma effect
    let mut e_buffer: Vec<(u8, u8, u8, u8)> = Vec::with_capacity(n_pixels as usize);
    e_buffer.resize(n_pixels as usize, (0u8, 0u8, 0u8, 0u8));
    let e_buf_len = e_buffer.len();

    // Target pixel buffer for the effect
    let p_buffer = glium::texture::pixel_buffer::PixelBuffer::new_empty(&display, n_pixels as usize);
    // Target texture for the effect
    let texture = glium::texture::Texture2d::empty(&display, b_width, b_height).unwrap();

    let mut t_sampler = TimeSampler::new(1000);
    let start_time_ns = precise_time_ns();
    let mut e_timer_render = ExecutionTimer::new();
    let mut e_timer_display = ExecutionTimer::new();
    let mut last_stats_time_ns = start_time_ns;
    let stats_interval_ns = 1000000000; // Stats interval is 1 second

    let n_chunks = 4;
    let mut pool = scoped_threadpool::Pool::new(n_chunks);

    while running {
        t_sampler.sample();

        // Perform rendering of plasa effect
        e_timer_render.measure(|| {
            let delta_t = precise_time_ns() - start_time_ns;

            pool.scoped(|scope| {
                for eb_slice in e_buffer.chunks_mut(e_buf_len / n_chunks as usize) {
                    scope.execute(move || plasma.render(eb_slice, delta_t, 0, b_height / n_chunks));
                }
            });
            //plasma.render(&mut e_buffer, precise_time_ns() - start_time_ns);
        });

        // Display it on screen
        e_timer_display.measure(|| {
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
            println!("avg_fps={}, render_time_ms(avg min max)=({:5.2} {:5.2} {:5.2}), display_time_ms(avg min max)=({:5.2} {:5.2} {:5.2})",
                avg_fps,
                e_timer_render.avg() as f64 / 1000000.0, e_timer_render.min() as f64 / 1000000.0, e_timer_render.max() as f64 / 1000000.0,
                e_timer_display.avg() as f64 / 1000000.0, e_timer_display.min() as f64 / 1000000.0, e_timer_display.max() as f64 / 1000000.0);
            last_stats_time_ns = last_stats_time_ns + stats_interval_ns;
            t_sampler.reset();
            e_timer_render.reset();
            e_timer_display.reset();
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
