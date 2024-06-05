extern crate sdl2;
mod chip8;

use crate::chip8::SCREEN_HEIGHT;
use crate::chip8::SCREEN_WIDTH;
use chip8::Chip8;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{self, Read};
use std::time::Duration;
use std::time::Instant;

const WINDOW_SCALE: u32 = 10;
const WINDOW_WIDTH: u32 = (SCREEN_WIDTH as u32) * WINDOW_SCALE;
const WINDOW_HEIGHT: u32 = (SCREEN_HEIGHT as u32) * WINDOW_SCALE;

fn main() -> Result<(), Box<dyn Error>> {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("CHIP-8 Emulator", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    // Create a new CHIP-8 emulator instance
    let mut chip8 = Chip8::new();
    let rom_path = "test_opcode.ch8";
    let current_dir = env::current_dir()?;
    println!("Current working directory: {:?}", current_dir);

    // Print the file path to ensure it's correct
    println!("Loading ROM from path: {}", rom_path);

    // Read the .ch8 file
    let mut file = File::open(rom_path)
        .map_err(|e| format!("Failed to open file: {} - Error: {}", rom_path, e))?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)
        .map_err(|e| format!("Failed to read file: {} - Error: {}", rom_path, e))?;
    // Read the program file into a byte vector

    // Load the program into the CHIP-8 emulator
    chip8.load_program(&buffer);

    let mut last_cycle_time = Instant::now();

    // Main emulation loop (simplified for this example)
    'running: loop {
        // Poll for events and handle key presses/releases
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                Event::KeyDown { keycode, .. } => {
                    if let Some(key) = map_keycode_to_chip8_key(keycode) {
                        chip8.keys[key as usize] = 1;
                    }
                }
                Event::KeyUp { keycode, .. } => {
                    if let Some(key) = map_keycode_to_chip8_key(keycode) {
                        chip8.keys[key as usize] = 0;
                    }
                }
                _ => {}
            }
        }

        // Run CPU cycles
        if last_cycle_time.elapsed().as_millis() >= 2 as u128 {
            chip8.emulate_cycle();
            last_cycle_time = Instant::now();
        }

        draw_screen(&chip8, &mut canvas);

        ::std::thread::sleep(Duration::from_millis(2)); // Sleep to avoid high CPU usage
    }

    Ok(())
}

fn draw_screen(chip8: &Chip8, canvas: &mut Canvas<Window>) {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    for (i, &pixel) in chip8.screen.iter().enumerate() {
        if pixel == 1 {
            let x = (i % SCREEN_WIDTH) as u32 * WINDOW_SCALE;
            let y = (i / SCREEN_WIDTH) as u32 * WINDOW_SCALE;

            canvas.set_draw_color(Color::RGB(255, 255, 255));
            let _ = canvas.fill_rect(Rect::new(x as i32, y as i32, WINDOW_SCALE, WINDOW_SCALE));
        }
    }

    canvas.present();
}

fn map_keycode_to_chip8_key(keycode: Option<Keycode>) -> Option<u8> {
    match keycode {
        Some(Keycode::Num1) => Some(0x1),
        Some(Keycode::Num2) => Some(0x2),
        Some(Keycode::Num3) => Some(0x3),
        Some(Keycode::Num4) => Some(0xC),
        Some(Keycode::Q) => Some(0x4),
        Some(Keycode::W) => Some(0x5),
        Some(Keycode::E) => Some(0x6),
        Some(Keycode::R) => Some(0xD),
        Some(Keycode::A) => Some(0x7),
        Some(Keycode::S) => Some(0x8),
        Some(Keycode::D) => Some(0x9),
        Some(Keycode::F) => Some(0xE),
        Some(Keycode::Z) => Some(0xA),
        Some(Keycode::X) => Some(0x0),
        Some(Keycode::C) => Some(0xB),
        Some(Keycode::V) => Some(0xF),
        _ => None,
    }
}

// Function to read a chip-8 program file into a byte vector
//fn read_program_file<P: AsRef<Path>>(path: P) -> io::Result<Vec<u8>> {
//    println!("fuck you");
//    if !std::path::Path::new(path).exists() {
//        eprintln!("File does not exist: {}", path);
//        return Ok(());
//    }
//    let mut file = File::open(path)?;
//    println!("fuck you again");
//    let mut buffer = Vec::new();
//    file.read_to_end(&mut buffer)?;
//    Ok(buffer)
//}
