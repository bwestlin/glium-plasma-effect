extern crate glium;
extern crate glium_sdl2;
extern crate sdl2;

mod plasma;

use glium_sdl2::DisplayBuild;
use glium::Surface;

use plasma::Plasma;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let width = 800;
    let height = 600;

    let display = video_subsystem.window("Glium Plasma", width, height)
        .resizable()
        .build_glium()
        .unwrap();

    let mut running = true;
    let mut event_pump = sdl_context.event_pump().unwrap();

    let b_width = 480 as u32;
    let b_height = 320 as u32;
    let n_pixels = b_width * b_width;
    let mut e_buffer: Vec<(u8, u8, u8, u8)> = Vec::new();
    e_buffer.resize(n_pixels as usize, (0u8, 0u8, 0u8, 0u8));

    let mut plasma = Plasma::new(b_width, b_height);

    let texture = glium::texture::Texture2d::empty(&display, b_width, b_height).unwrap();
    let p_buffer = glium::texture::pixel_buffer::PixelBuffer::new_empty(&display, n_pixels as usize);

    while running {
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);

        plasma.render(&mut e_buffer);
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
