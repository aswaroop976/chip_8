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
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
use std::time::Duration;

const WINDOW_SCALE: u32 = 10;
const WINDOW_WIDTH: u32 = (SCREEN_WIDTH as u32) * WINDOW_SCALE;
const WINDOW_HEIGHT: u32 = (SCREEN_HEIGHT as u32) * WINDOW_SCALE;

fn main() -> io::Result<()> {
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

    // Specify the path to the CHIP-8 program file
    let program_path = "path_to_your_chip8_program.ch8";

    // Read the program file into a byte vector
    let program = read_program_file(program_path)?;

    // Load the program into the CHIP-8 emulator
    chip8.load_program(&program);

    // Main emulation loop (simplified for this example)
    loop {
        chip8.emulate_cycle();

        // Add timing control, input handling, and rendering here
    }
}

// Function to read a chip-8 program file into a byte vector
fn read_program_file<P: AsRef<Path>>(path: P) -> io::Result<Vec<u8>> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}
