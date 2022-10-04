use sdl2::pixels::PixelFormatEnum;
use sdl2::render::{TextureAccess, TextureQuery};
use sdl2::{event::Event, pixels::Color, rect::Rect};
use std::cmp::max;
use std::collections::HashMap;
use std::path::Path;
use std::sync::mpsc;
use std::thread;

const HEIGHT: usize = 768;
const WIDTH: usize = 1024;
const MAX_ITERATIONS: u8 = 100;
const THREAD_COUNT: usize = 6;

struct Thread {
    available: bool,
    tx: mpsc::Sender<ThreadInput>,
}

struct ThreadInput {
    y: usize,
    zoom: f32,
}

#[derive(Copy, Clone)]
struct ThreadResult {
    thread_id: usize,
    row: [u8; WIDTH * 4],
    y: usize,
}

fn calc(
    y: usize,
    win_max_len: f32,
    center_x: f32,
    center_y: f32,
    offset_x: f32,
    offset_y: f32,
    zoom: f32,
) -> [u8; WIDTH * 4] {
    let mut buffer = [0_u8; WIDTH * 4];
    let mandel_y = center_y + ((y as f32 / win_max_len - offset_y) / zoom);

    for x in 0..WIDTH {
        let pc_x = x as f32 / win_max_len - offset_x;
        let mandel_x = center_x + (pc_x / zoom);

        let iterations = iterations(mandel_x, mandel_y);
        let offset = x * 4;
        let c = if iterations > 50 { 0 } else { 255 };

        buffer[offset] = c;
        buffer[offset + 1] = c;
        buffer[offset + 2] = c;
        buffer[offset + 3] = 255;
    }

    buffer
}

fn iterations(x: f32, y: f32) -> u8 {
    let mut count: u8 = 0;

    let mut tx: f64 = 0.0;
    let mut ty: f64 = 0.0;

    let mut txs: f64 = 0.0;
    let mut tys: f64 = 0.0;

    loop {
        ty = 2.0 * tx * ty + (y as f64);
        tx = txs - tys + (x as f64);

        txs = tx * tx;
        tys = ty * ty;

        count += 1;

        if count >= MAX_ITERATIONS || txs + tys > 4.0 {
            return count;
        }
    }
}

fn main() -> Result<(), String> {
    let win_max_len = max(WIDTH, HEIGHT);

    let sdl = sdl2::init()?;
    let timer = sdl.timer()?;
    let video = sdl.video()?;

    let ttf = sdl2::ttf::init().expect("initialising ttf");

    let performance_frequency = timer.performance_frequency() as f32;
    let mut last: u64;
    let mut now = timer.performance_counter();
    let mut delta: f32;

    let window = video
        .window("mandelbrot-rs", WIDTH as u32, HEIGHT as u32)
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
            TextureAccess::Target,
            WIDTH as u32,
            HEIGHT as u32,
        )
        .expect("creating texture");

    let font_path = Path::new("fonts/SourceSans3-Bold.ttf");
    let font = ttf.load_font(font_path, 14)?;

    let man_center_x = -0.765;
    let man_center_y = 0.0;

    let offset_x: f32 = (WIDTH as f32 / win_max_len as f32) * 0.5;
    let offset_y: f32 = (HEIGHT as f32 / win_max_len as f32) * 0.5;

    let mut event_pump = sdl.event_pump()?;
    let mut zoom: f32 = 0.3;

    let mut fps_texture = texture_creator
        .create_texture_from_surface(
            &font
                .render("-")
                .blended(Color::RGBA(255, 0, 0, 255))
                .expect("rendering font"),
        )
        .expect("creating texture");

    let TextureQuery { width, height, .. } = fps_texture.query();
    let mut fps_texture_target = Rect::new(8, 8, width, height);

    let mut time_until_fps_redraw = 1000.0;

    let pitch = WIDTH * 4; // WIDTH * RGBA

    let mut thread_pool = HashMap::new();

    let (result_tx, result_rx) = mpsc::channel();

    for i in 0..THREAD_COUNT {
        let rtx = result_tx.clone();
        let (work_order_tx, work_order_rx) = mpsc::channel::<ThreadInput>();

        thread::spawn(move || loop {
            let thread_id = i;

            for input in &work_order_rx {
                let row = calc(
                    input.y,
                    win_max_len as f32,
                    man_center_x,
                    man_center_y,
                    offset_x,
                    offset_y,
                    input.zoom,
                );

                rtx.send(ThreadResult {
                    thread_id,
                    y: input.y,
                    row,
                })
                .expect("sending result");
            }
        });

        thread_pool.insert(
            i,
            Thread {
                tx: work_order_tx,
                available: true,
            },
        );
    }

    'event_loop: loop {
        for event in event_pump.poll_iter() {
            if let Event::Quit { .. } = event {
                break 'event_loop;
            }
        }

        last = now;
        now = timer.performance_counter();
        delta = ((now - last) * 1000) as f32 / performance_frequency;

        let mut buffer = Vec::new();
        buffer.resize(HEIGHT * (WIDTH * 4), 0);

        let mut next_y: usize = 0;
        let mut working_count: usize = 0;

        'render: loop {
            if next_y < HEIGHT {
                for i in 0..THREAD_COUNT {
                    let thread: &mut Thread = thread_pool.get_mut(&i).unwrap();
                    if thread.available {
                        thread.available = false;
                        let thread_input = ThreadInput { y: next_y, zoom };
                        thread.tx.send(thread_input).expect("sending y");
                        next_y += 1;
                        working_count += 1;
                        continue 'render;
                    }
                }
            }

            let received = result_rx.try_recv();

            match received {
                Err(..) => {}
                Ok(result) => {
                    let thread: &mut Thread = thread_pool.get_mut(&result.thread_id).unwrap();
                    thread.available = true;
                    working_count -= 1;

                    let offset = result.y * (WIDTH * 4);

                    buffer.splice(offset..offset + result.row.len(), result.row);
                }
            }

            if next_y >= HEIGHT && working_count == 0 {
                break 'render;
            }
        }

        man_texture
            .update(None, &buffer, pitch)
            .expect("updating texture");

        canvas.copy(&man_texture, None, None)?;

        if time_until_fps_redraw < 0.0 {
            time_until_fps_redraw = 1000.0;

            let fps: u8 = (1000.0 / delta).floor() as u8;

            fps_texture = texture_creator
                .create_texture_from_surface(
                    &font
                        .render(&fps.to_string().to_owned())
                        .blended(Color::RGBA(255, 0, 0, 255))
                        .expect("rendering font"),
                )
                .expect("creating texture");

            let TextureQuery { width, height, .. } = fps_texture.query();
            fps_texture_target = Rect::new(8, 8, width, height);
        } else {
            time_until_fps_redraw -= delta;
        }

        canvas.copy(&fps_texture, None, Some(fps_texture_target))?;

        canvas.present();
        zoom += 0.005 * (delta / 1000.0);
    }

    Ok(())
}
