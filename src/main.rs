extern crate glium;
extern crate glium_sdl2;
extern crate sdl2;
extern crate time;

mod plasma;
mod util;

use glium_sdl2::DisplayBuild;
use glium::Surface;
use time::*;

use plasma::Plasma;
use util::TimeSampler;

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
    let mut e_buffer: Vec<(u8, u8, u8, u8)> = Vec::with_capacity(n_pixels as usize);
    e_buffer.resize(n_pixels as usize, (0u8, 0u8, 0u8, 0u8));

    let mut plasma = Plasma::new(b_width, b_height);

    let texture = glium::texture::Texture2d::empty(&display, b_width, b_height).unwrap();
    let p_buffer = glium::texture::pixel_buffer::PixelBuffer::new_empty(&display, n_pixels as usize);

    let mut t_sampler = TimeSampler::new(1000);
    let start_time_ns = precise_time_ns();
    let mut last_sample_time_ns = start_time_ns;
    let sample_interval_ns = 1000000000;

    while running {
        t_sampler.sample();
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);

        plasma.render(&mut e_buffer, precise_time_ns() - start_time_ns);
        p_buffer.write(&e_buffer);
        texture.main_level().raw_upload_from_pixel_buffer(p_buffer.as_slice(), 0 .. b_width, 0 .. b_height, 0 .. 1);
        texture.as_surface().fill(&target, glium::uniforms::MagnifySamplerFilter::Linear);

        target.finish().unwrap();

        if precise_time_ns() - last_sample_time_ns > sample_interval_ns {
            let avg_fps = t_sampler.avg_per_second();
            println!("avg_fps={}", avg_fps);
            last_sample_time_ns = last_sample_time_ns + sample_interval_ns;
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
