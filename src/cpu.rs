const MEMORY_SIZE: usize = 4096;
const START_ADDR: u16 = 512;

pub struct CPU {
    pub memory: [u8; MEMORY_SIZE],
    pub v: [u8; 16],
    pub i: u16,
    pub jump: [u16; 16],
    pub jump_nbr: u8,
    pub system_counter: u8,
    pub sound_counter: u8,
    pub pc: u16,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            memory: [0; MEMORY_SIZE],
            v: [0; 16],
            i: 0,
            jump: [0; 16],
            jump_nbr: 0,
            system_counter: 0,
            sound_counter: 0,
            pc: START_ADDR
        }
    }

    pub fn decrease(&mut self) {
        if self.system_counter > 0 {
            self.system_counter = self.system_counter - 1;
        }
        if self.sound_counter > 0 {
            self.sound_counter = self.sound_counter - 1;
        }
    }

    pub fn get_opcode(&self) -> u16 {
        ((self.memory[self.pc as usize] as u16 ) << 8) + (self.memory[(self.pc + 1) as usize] as u16)
    }
}
