pub struct Chip8 {
    pub memory: [u8; 4096],  // 4kb memory
    pub registers: [u8; 16], // 16 general purpose registers
    pub index_register: u16,
    pub program_counter: u16,
    pub screen: [u8; 64 * 32], // 64x32 pixel display
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub stack: [u16; 16],  // stack with 16 levels
    pub stack_pointer: u8, // stack pointer
    pub keys: [u8; 16],
}

impl Chip8 {
    pub fn new() -> Self {
        Chip8 {
            memory: [0; 4096],
            registers: [0; 16],
            index_register: 0,
            program_counter: 0x200, // Programs start at 0x200
            screen: [0; 64 * 32],
            delay_timer: 0,
            sound_timer: 0,
            stack: [0; 16],
            stack_pointer: 0,
            keys: [0; 16],
        }
    }

    pub fn load_program(&mut self, program: &[u8]) {
        for (i, &byte) in program.iter().enumerate() {
            self.memory[0x200 + i] = byte;
        }
    }
}
