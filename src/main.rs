pub mod loupe;
pub mod vector2;

use loupe::Loupe;
use sdl2::{event::Event, pixels::Color, rect::Point};
use std::{cmp::max, time::Duration};
use vector2::Vector2;

fn main() -> Result<(), String> {
    let max_iterations: u16 = 100;
    let win_size = Vector2 { x: 1024, y: 768 };

    let win_max_len = max(win_size.x, win_size.y) as f32;

    let sdl = sdl2::init()?;
    let timer = sdl.timer()?;
    let video = sdl.video()?;

    let performance_frequency = timer.performance_frequency() as f32;
    let mut last: u64;
    let mut now = timer.performance_counter();
    let mut delta: f32;

    let window = video
        .window("mandelbrot-rs", win_size.x, win_size.y)
        .position_centered()
        .build()
        .expect("building window");

    let mut canvas = window.into_canvas().build().expect("building canvas");

    let loupe = Loupe::new();

    let offset = Vector2 {
        x: (win_size.x as f32 / win_max_len) * 0.5,
        y: (win_size.y as f32 / win_max_len) * 0.5,
    };

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

            canvas.set_draw_color(color);
            canvas.draw_point(Point::new(x as i32, y as i32))?;
        }
    }

    canvas.present();

    let mut event_pump = sdl.event_pump()?;
    let mut frame_count: u8 = 0;

    'event_loop: loop {
        for event in event_pump.poll_iter() {
            if let Event::Quit { .. } = event {
                break 'event_loop;
            }
        }

        last = now;
        now = timer.performance_counter();
        delta = ((now - last) * 1000) as f32 / performance_frequency;

        frame_count += 1;
        if frame_count > 100 {
            frame_count = 0;
            println!("delta={delta}");
        }

        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
