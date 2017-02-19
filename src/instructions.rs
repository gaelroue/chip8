const NBR_OPCODES: usize = 35;

pub struct Opcodes {
    pub mask: [u16; NBR_OPCODES],
    pub id: [u16; NBR_OPCODES]
}

impl Opcodes{
    pub fn new() -> Opcodes {
        Opcodes {
            mask : [0x0000,
                    0xFFFF,
                    0xFFFF,
                    0xF000,
                    0xF000,
                    0xF000,
                    0xF000,
                    0xF00F,
                    0xF000,
                    0xF000,
                    0xF00F, 
                    0xF00F, 
                    0xF00F, 
                    0xF00F, 
                    0xF00F, 
                    0xF00F, 
                    0xF00F, 
                    0xF00F, 
                    0xF00F, 
                    0xF00F, 
                    0xF000, 
                    0xF000, 
                    0xF000, 
                    0xF000, 
                    0xF0FF, 
                    0xF0FF, 
                    0xF0FF, 
                    0xF0FF, 
                    0xF0FF, 
                    0xF0FF, 
                    0xF0FF, 
                    0xF0FF, 
                    0xF0FF, 
                    0xF0FF, 
                    0xF0FF], 

            id : [0x0FFF,
                  0x00E0,
                  0x00EE,
                  0x1000,
                  0x2000,
                  0x3000,
                  0x4000,
                  0x5000,
                  0x6000,
                  0x7000,
                  0x8000,
                  0x8001,
                  0x8002,
                  0x8003,
                  0x8004,
                  0x8005,
                  0x8006,
                  0x8007,
                  0x800E,
                  0x9000,
                  0xA000,
                  0xB000,
                  0xC000,
                  0xD000,
                  0xE09E,
                  0xE0A1,
                  0xF007,
                  0xF00A,
                  0xF015,
                  0xF018,
                  0xF01E,
                  0xF029,
                  0xF033,
                  0xF055,
                  0xF065],
        }
    }

    pub fn get_action(&self, opcode: u16) -> u8 {
        for action in 0..self.mask.len() {
               let result = self.mask[action as usize] & opcode; 
               if result == self.id[action as usize] {
                    return action as u8;
               }
        }
        0 as u8
    }

}
