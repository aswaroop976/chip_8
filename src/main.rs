extern crate sdl2;

use rand::Rng;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("CHIP-8 Emulator", 800, 600)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut rng = rand::thread_rng();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                _ => {}
            }
        }

        // Draw a random color
        let r: u8 = rng.gen();
        let g: u8 = rng.gen();
        let b: u8 = rng.gen();
        canvas.set_draw_color(Color::RGB(r, g, b));
        canvas.clear();
        canvas.present();

        ::std::thread::sleep(Duration::from_millis(1000));
    }
}

//fn main() {
//    println!("Hello, world!");
//}
