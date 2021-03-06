use sdl2::render::BlendMode;
use sdl2::rect::Rect;
use sdl2::rect::Point;
use sdl2::event::Event;
use sdl2::pixels::Color;
use std::time::Duration;
use sdl2::pixels::PixelFormatEnum;
use scarlet::colorpoint::ColorPoint;
use scarlet::color::RGBColor;

mod ode;
mod math;

// main loop
pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let width = 800;
    let height = 400;
    let upsampling = 2;
    let t_width = width*upsampling;
    let t_height = height*upsampling;

    // window
    let window = video_subsystem.window("ode-flow", width, height)
        .position_centered()
        .allow_highdpi()
        .build()
        .unwrap();
 
    // canvas
    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_logical_size(width, height).unwrap();

    // textures
    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::RGBA8888, t_width, t_height)
        .map_err(|e| e.to_string()).unwrap();
    
    let mut texture_line = texture_creator
        .create_texture_target(texture_creator.default_pixel_format(), t_width, t_height)
        .unwrap();

    texture.set_blend_mode(BlendMode::Blend);
    texture_line.set_blend_mode(BlendMode::Blend);

    // colormap
    let start = RGBColor::from_hex_code("#00CCFF").unwrap();
    let end = RGBColor::from_hex_code("#222FDA").unwrap();
    let grad = start.gradient(&end);

    // streaming texture
    texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
        for y in 0..height as usize {
            for x in 0..width as usize {
                let offset = y * pitch + x * 4;
                buffer[offset] = 255 as u8;
                buffer[offset + 1] = 0;
                buffer[offset + 2] = 0;
                buffer[offset + 3] = 0;
            }
        }
    }).unwrap();
    

    // init parameters
    let norm_atten = 0.1;
    let extent_x = (-10.0, 10.0);
    let extent_y = (-5.0, 5.0);
    let mut xy_vec: Vec<(f64, f64)> = Vec::new();

    // loop
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        // event loop
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        // generate batches
        for _ in 0..256 {
            xy_vec.push(math::generate_random_tuple(extent_x, extent_y, t_width, t_width));
        }
        // keep fixed len
        while xy_vec.len() > usize::pow(2, 12) {
            xy_vec.drain(0..16);
        }

        // Target texture
        canvas.with_texture_canvas(&mut texture_line, |texture_canvas| {
            texture_canvas.set_draw_color(Color::RGBA(0, 0, 0, 128));
            texture_canvas.clear();

            // process ode paths
            for xy in xy_vec.iter_mut() {
                // pixel positions
                let xy_p = math::state_to_pixels(*xy, extent_x, extent_y, t_width, t_height);

                // state gradient
                let dxy = ode::pendulum(xy.0, xy.1);
                let gxy = (xy.0 + dxy.0 * 0.25, xy.1 + dxy.1 * 0.25);

                // norm
                let norm = math::norm(dxy) / 10.0;

                // pixel gradient
                let gxy_p = math::state_to_pixels(gxy, extent_x, extent_y, t_width, t_height);

                // draw gradient line
                let color = grad(norm);
                let points = [Point::new(xy_p.0 , xy_p.1), Point::new(gxy_p.0, gxy_p.1)];
                texture_canvas.set_draw_color(Color::RGBA(color.int_r(), color.int_g(), color.int_b(), 255));
                texture_canvas.draw_lines(&points[..]).unwrap();

                // update particles
                xy.0 = xy.0 + dxy.0 * norm_atten;
                xy.1 = xy.1 + dxy.1 * norm_atten;
            }
        }).unwrap();
    
        // Post comp
        canvas.copy(&texture, None, Some(Rect::new(0, 0, width, height))).unwrap();
        canvas.copy(&texture_line, None, Some(Rect::new(0, 0, width, height))).unwrap();
        canvas.present();

        // fps
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
