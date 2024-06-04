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
    fn jp(&mut self, opcode: u16) {
        let address = opcode & 0x0FFF;
        self.program_counter = address;
    }
    // CALL - 2NNN
    // Instruction: call subroutine at NNN
    fn call(&mut self, opcode: u16) {
        let address = opcode & 0x0FFF;
        self.stack[self.stack_pointer as usize] = self.program_counter;
        self.stack_pointer += 1;
        self.program_counter = address;
    }
    // SE Vx, byte - 3XNN
    // Instruction: skip next instruction if Vx equals NN
    fn se_vx_byte(&mut self, opcode: u16) {
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let byte = (opcode & 0x00FF) as u8;
        if self.registers[x] == byte {
            self.program_counter += 2;
        }
    }
    // SNE Vx, byte - 4XNN
    // Instruction: skip next instruction if Vx doesn't equal NN
    fn sne_vx_byte(&mut self, opcode: u16) {
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let byte = (opcode & 0x00FF) as u8;
        if self.registers[x] != byte {
            self.program_counter += 2;
        }
    }
    // SE Vx, Vy - 5XY0
    // Instruction: skip next instruction if Vx equals Vy
    fn se_vx_vy(&mut self, opcode: u16) {
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;
        if self.registers[x] == self.registers[y] {
            self.program_counter += 2;
        }
    }
    // LD Vx, byte - 6XNN
    // Instruction: set Vx to NN
    fn ld_vx_byte(&mut self, opcode: u16) {
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let byte = (opcode & 0x00FF) as u8;
        self.registers[x] = byte;
    }
    // ADD Vx, byte - 7XNN
    // Instruction: add NN to Vx
    fn add_vx_byte(&mut self, opcode: u16) {
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let byte = (opcode & 0x00FF) as u8;
        self.registers[x] = self.registers[x].wrapping_add(byte);
    }
    // LD Vx, Vy - 8XY0
    // Instruction: set Vx to the value of Vy
    fn ld_vx_vy(&mut self, opcode: u16) {
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;
        self.registers[x] = self.registers[y];
    }
    // OR Vx, Vy - 8XY1
    // Instruction: set Vx to Vx OR Vy
    fn or_vx_vy(&mut self, opcode: u16) {
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;
        self.registers[x] |= self.registers[y];
    }
    // AND Vx, Vy - 8XY2
    // Instruction: set Vx to Vx AND Vy
    fn and_vx_vy(&mut self, opcode: u16) {
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;
        self.registers[x] &= self.registers[y];
    }
    // XOR Vx, Vy - 8XY3
    // Instruction: set Vx to Vx XOR Vy
    fn xor_vx_vy(&mut self, opcode: u16) {
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;
        self.registers[x] ^= self.registers[y];
    }
    // ADD Vx, Vy - 8XY4
    // Instruction: Add Vy to Vx, set VF = carry
    fn add_vx_vy(&mut self, opcode: u16) {
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;
        let (result, carry) = self.registers[x].overflowing_add(self.registers[y]);
        self.registers[x] = result;
        self.registers[0xF] = if carry { 1 } else { 0 };
    }
    // SUB Vx, Vy - 8XY5
    // Instruction: subtract Vy from Vx, set VF = NOT borrow
    fn sub_vx_vy(&mut self, opcode: u16) {
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;
        let (result, borrow) = self.registers[x].overflowing_sub(self.registers[y]);
        self.registers[x] = result;
        self.registers[0xF] = if borrow { 0 } else { 1 };
    }
    // SHR Vx - 8XY6
    // Instruction: set Vx = Vx SHR 1
    fn shr_vx(&mut self, opcode: u16) {
        let x = ((opcode & 0x0F00) >> 8) as usize;
        self.registers[0xF] = self.registers[x] & 0x1;
        self.registers[x] >>= 1;
    }
    // SUBN Vx, Vy - 8XY7
    // Instruction: set Vx = Vy - Vx, set VF = NOT borrow
    fn subn_vx_vy(&mut self, opcode: u16) {
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;
        self.registers[0xF] = if self.registers[y] > self.registers[x] {
            1
        } else {
            0
        };
        self.registers[x] = self.registers[y].wrapping_sub(self.registers[x]);
    }
    // SHL Vx - 8XYE
    // Instruction: set Vx = Vx SHL 1
    fn shl_vx(&mut self, opcode: u16) {
        let x = ((opcode & 0x0F00) >> 8) as usize;
        self.registers[0xF] = (self.registers[x] & 0x80) >> 7;
        self.registers[x] <<= 1;
    }
    // SNE Vx, Vy - 9XY0
    // Instruction: skip the next instruction if Vx != Vy
    fn sne_vx_vy(&mut self, opcode: u16) {
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;
        if self.registers[x] != self.registers[y] {
            self.program_counter += 2;
        }
    }
    // LD I, addr - ANNN
    // Instruction: set I = NNN
    fn ld_i_addr(&mut self, opcode: u16) {
        let address = opcode & 0x0FFF;
        self.index_register = address;
    }
}
