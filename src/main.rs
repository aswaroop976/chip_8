mod chip8;

use chip8::Chip8;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

fn main() -> io::Result<()> {
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
