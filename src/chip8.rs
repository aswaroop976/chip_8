use std::usize;

const MEMORY_SIZE: usize = 4096;
const REGISTER_COUNT: usize = 16;
const STACK_SIZE: usize = 16;
const SCREEN_WIDTH: usize = 64;
const SCREEN_HEIGHT: usize = 32;
const FONTSET_SIZE: usize = 80;
const FONTSET_START_ADDRESS: usize = 0x50;
const PROGRAM_START_ADDRESS: usize = 0x200;

pub struct Chip8 {
    pub memory: [u8; MEMORY_SIZE],       // 4kb memory
    pub registers: [u8; REGISTER_COUNT], // 16 general purpose registers
    pub index_register: u16,
    pub program_counter: u16,
    pub screen: [u8; SCREEN_WIDTH * SCREEN_HEIGHT], // 64x32 pixel display
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub stack: [u16; STACK_SIZE], // stack with 16 levels
    pub stack_pointer: u8,        // stack pointer
    pub keys: [u8; REGISTER_COUNT],
}

impl Chip8 {
    pub fn new() -> Self {
        let mut chip8 = Chip8 {
            memory: [0; MEMORY_SIZE], //figure it retard
            registers: [0; REGISTER_COUNT],
            index_register: 0,
            program_counter: PROGRAM_START_ADDRESS as u16, // Programs start at 0x200
            screen: [0; SCREEN_WIDTH * SCREEN_HEIGHT],
            delay_timer: 0,
            sound_timer: 0,
            stack: [0; STACK_SIZE],
            stack_pointer: 0,
            keys: [0; REGISTER_COUNT],
        };
        chip8.load_fonts();
        chip8
    }

    pub fn load_fonts(&mut self) {
        let fontset: [u8; FONTSET_SIZE] = [
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80, // F
        ];
        for (i, &byte) in fontset.iter().enumerate() {
            self.memory[FONTSET_START_ADDRESS + i] = byte;
        }
    }

    pub fn load_program(&mut self, program: &[u8]) {
        for (i, &byte) in program.iter().enumerate() {
            self.memory[0x200 + i] = byte;
        }
    }
    // need to implement the fetch, decode, and execute instructions

    // instruction implementation
    // CLS - 00E0
    // Instruction: clear the display
    fn cls(&mut self) {
        self.screen = [0; SCREEN_WIDTH * SCREEN_HEIGHT]
    }
    // RET - 00EE
    // Instruction: return from a subroutine
    fn ret(&mut self) {
        self.stack_pointer -= 1;
        self.program_counter = self.stack[self.stack_pointer as usize];
    }
    // JP - 1NNN
    // Instruction: jump to address NNN
    fn jp(&mut self, address: u16) {
        self.program_counter = address
    }
    // CALL - 2NNN
    // Instruction: call subroutine at NNN
    fn call(&mut self, address: u16) {
        self.stack[self.stack_pointer as usize] = self.program_counter;
        self.stack_pointer += 1;
        self.program_counter = address
    }
    // SE Vx, byte - 3XNN
    // Instruction: skip next instruction if Vx equals NN
    fn se_vx_byte(&mut self, x: u16, byte: u8) {
        if self.registers[x as usize] == byte {
            self.program_counter += 2;
        }
    }
    // SNE Vx, byte - 4XNN
    // Instruction: skip next instruction if Vx doesn't equal NN
    fn sne_vx_byte(&mut self, x: u16, byte: u8) {
        if self.registers[x as usize] != byte {
            self.program_counter += 2;
        }
    }
    // SE Vx, Vy - 5XY0
    // Instruction: skip next instruction if Vx equals Vy
    fn se_vx_vy(&mut self, x: u16, y: u16) {
        if self.registers[x as usize] == self.registers[y as usize] {
            self.program_counter += 2;
        }
    }
    // LD Vx, byte - 6XNN
    // Instruction: set Vx to NN
    fn ld_vx_byte(&mut self, x: u16, byte: u8) {
        self.registers[x as usize] = byte;
    }
    // ADD Vx, byte - 7XNN
    // Instruction: add NN to Vx
    fn add_vx_byte(&mut self, x: u16, byte: u8) {
        self.registers[x as usize] = self.registers[x as usize].wrapping_add(byte);
    }
    // LD Vx, Vy - 8XY0
    // Instruction: set Vx to the value of Vy
    fn ld_vx_vy(&mut self, x: u16, y: u16) {
        self.registers[x as usize] = self.registers[y as usize];
    }
}
