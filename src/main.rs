pub mod loupe;
pub mod vector2;

use loupe::Loupe;
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::{TextureAccess, TextureQuery};
use sdl2::{
    event::Event,
    pixels::Color,
    rect::Rect,
};
use std::path::Path;
use std::cmp::max;
use vector2::Vector2;

fn main() -> Result<(), String> {
    let max_iterations: u16 = 100;
    let win_size = Vector2 { x: 1024, y: 768 };

    let win_max_len = max(win_size.x, win_size.y) as f32;

    let sdl = sdl2::init()?;
    let timer = sdl.timer()?;
    let video = sdl.video()?;

    let ttf = sdl2::ttf::init().expect("initialising ttf");

    let performance_frequency = timer.performance_frequency() as f32;
    let mut last: u64;
    let mut now = timer.performance_counter();
    let mut delta: f32;

    let window = video
        .window("mandelbrot-rs", win_size.x, win_size.y)
        .position_centered()
        .build()
        .expect("building window");

    let mut canvas = window
        .into_canvas()
        .accelerated()
        .build()
        .expect("building canvas");

    let texture_creator = canvas.texture_creator();

    let mut man_texture = texture_creator
        .create_texture(
            PixelFormatEnum::RGB888,
            TextureAccess::Streaming,
            win_size.x,
            win_size.y,
        )
        .expect("creating texture");

    let font_path = Path::new("fonts/SourceSans3-Bold.ttf");
    let font = ttf.load_font(font_path, 14)?;

    let loupe = Loupe::new();

    let offset = Vector2 {
        x: (win_size.x as f32 / win_max_len) * 0.5,
        y: (win_size.y as f32 / win_max_len) * 0.5,
    };

    let mut event_pump = sdl.event_pump()?;

    'event_loop: loop {
        for event in event_pump.poll_iter() {
            if let Event::Quit { .. } = event {
                break 'event_loop;
            }
        }

        // man_texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
        //     for x in 0..win_size.x {
        //         for y in 0..win_size.y {
        //             // Percentage distance from the centre of the canvas.
        //             let pc = Vector2 {
        //                 x: (x as f32 / win_max_len - offset.x),
        //                 y: (y as f32 / win_max_len - offset.y),
        //             };

        //             let iterations = loupe.iterations(&pc, max_iterations);

        //             let color = if iterations > 30 {
        //                 Color::RGB(0, 0, 0)
        //             } else {
        //                 Color::RGB(255, 255, 255)
        //             };

        //             let offset = ((y as usize) * pitch) + ((x as usize) * 4);

        //             buffer[offset] = color.b;
        //             buffer[offset + 1] = color.g;
        //             buffer[offset + 2] = color.r;
        //             buffer[offset + 3] = 255;
        //         }
        //     }
        // })?;

        // canvas.copy(&man_texture, None, None)?;
        // canvas.present();

        last = now;
        now = timer.performance_counter();
        delta = ((now - last) * 1000) as f32 / performance_frequency;

        man_texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
            for x in 0..win_size.x {
                for y in 0..win_size.y {
                    // Percentage distance from the centre of the canvas.
                    let pc = Vector2 {
                        x: (x as f32 / win_max_len - offset.x),
                        y: (y as f32 / win_max_len - offset.y),
                    };

                    let man_coords = loupe.get(&pc);

                    let mut iteration: u16 = 0;

                    let mut t = Vector2 { x: 0.0, y: 0.0 };
                    let mut t_squared = Vector2 { x: 0.0, y: 0.0 };

                    while (t_squared.x + t_squared.y) <= 4.0 && iteration < max_iterations {
                        t_squared.x = f32::powi(t.x, 2);
                        t_squared.y = f32::powi(t.y, 2);

                        t.y = 2.0 * t.x * t.y + man_coords.y;
                        t.x = t_squared.x - t_squared.y + man_coords.x;

                        iteration += 1;
                    }

                    let color = if iteration > 30 {
                        Color::RGB(0, 0, 0)
                    } else {
                        Color::RGB(255, 255, 255)
                    };

                    let offset = ((y as usize) * pitch) + ((x as usize) * 4);

                    buffer[offset] = color.b;
                    buffer[offset + 1] = color.g;
                    buffer[offset + 2] = color.r;
                    buffer[offset + 3] = 255;
                }
            }
        })?;

        canvas.copy(&man_texture, None, None)?;

        let fps: u8 = (1000.0 / delta).floor() as u8;
        let surface = font
            .render(&fps.to_string().to_owned())
            .blended(Color::RGBA(255, 0, 0, 255))
            .expect("rendering font");

        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .expect("creating texture");

        let TextureQuery { width, height, .. } = texture.query();
        let target = Rect::new(8, 8, width, height);
        canvas.set_draw_color(Color::RGBA(255, 0, 0, 255));
        canvas.copy(&texture, None, Some(target))?;

        canvas.present();
    }

    Ok(())
}
