use std::time::Instant;

use minifb::{Key, Window, WindowOptions};

use crate::app::App;

mod app;
mod camera;
mod renderer;
mod scene;
mod ray;
mod text;
mod utils;

const WIDTH: usize = 712;
const HEIGHT: usize = 400;

fn main() {
    let mut app = App::new(WIDTH, HEIGHT);
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });


    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let mut ts = Instant::now();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        app.on_update(ts.elapsed(), &mut window);

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
        app.render(&mut buffer);

        ts = Instant::now();
    }
}
