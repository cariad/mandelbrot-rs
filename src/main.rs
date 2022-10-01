pub mod vector2;

use sdl2::event::Event;
use std::time::Duration;
use vector2::Vector2;

fn main() -> Result<(), String> {
    let win_size = Vector2 { x: 800, y: 600 };

    let sdl = sdl2::init()?;
    let video = sdl.video()?;

    let _window = video
        .window("mandelbrot-rs", win_size.x, win_size.y)
        .position_centered()
        .build()
        .expect("initialising SDL video subsystem");

    let mut event_pump = sdl.event_pump()?;

    'event_loop: loop {
        for event in event_pump.poll_iter() {
            if let Event::Quit { .. } = event {
                break 'event_loop;
            }
        }

        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
