use cpu::CPU;
use instructions::Opcodes;
use screen::Screen;
use screen::HEIGHT;
use screen::WIDTH;
use screen::DIMPIXEL;
use screen::BLACK;
use screen::WHITE;
use sdl2::{Sdl};

pub struct CHIP8<'a> {
    pub cpu: CPU,
    pub instructions:Opcodes,
    pub screen:Screen<'a>
}

impl<'a> CHIP8<'a> {
    pub fn new(ctx: &Sdl) -> CHIP8<'a> {
        CHIP8 {
            cpu: CPU::new(),
            instructions: Opcodes::new(),
            screen: Screen::new(&ctx)
        }
    }

    fn draw(&mut self, b1: u8, b2: u8, b3: u8) {
        let (mut x, mut y, mut coding, mut j, mut shift): (u8, u8, u8, u8, u8) = (0, 0, 0, 0, 0);
        self.cpu.v[0x16] = 0;

        for k in 0..b1 {
            coding = self.cpu.memory[(self.cpu.i + k as u16) as usize]; // code of the line to draw

            y = ( self.cpu.v[b2 as usize] + k) % HEIGHT as u8;

            shift = 7;
            for j in 0..b3 {
                x = (self.cpu.v[b3 as usize] + j) % WIDTH as u8;

                if ((coding & (0x1 << shift)) != 0) {
                    if (self.screen.buffer[(x as usize/DIMPIXEL) + (y as usize/DIMPIXEL) * WIDTH] == WHITE as u8) {
                        self.screen.buffer[(x as usize/DIMPIXEL) + (y as usize/DIMPIXEL) * WIDTH] = BLACK as u8;
                        self.cpu.v[0xF] = 1;
                    } else {
                        self.screen.buffer[(x as usize/DIMPIXEL) + (y as usize/DIMPIXEL) * WIDTH] = WHITE as u8;
                    }
                }
                shift-=1;
            }
        }
    }

    pub fn decode(&mut self, opcode: u16) {
        let b4 = self.instructions.get_action(opcode);
        let b3 : u8 = ((opcode & (0x0F00)) >> 8) as u8;
        let b2 : u8 = ((opcode & (0x00F0)) >> 4) as u8;
        let b1 : u8 = (opcode & (0x000F)) as u8;

      match b4 {
          //0 =>,
          //1 =>,
          //2 =>,
          //3 =>,
          //4 =>,
          //5 =>,
          //6 =>,
          //7 =>,
          //8 =>,
          //9 =>,
          //10 =>,
          //11 =>,
          //12 =>,
          //13 =>,
          //14 =>,
          //15 =>,
          //16 =>,
          //17 =>,
          //18 =>,
          //19 =>,
          //20 =>,
          //21 =>,
          //22 =>,
          23 => self.draw(b1, b2, b3),
          //24 =>,
          //25 =>,
          //26 =>,
          //27 =>,
          //28 =>,
          //29 =>,
          //30 =>,
          //31 =>,
          //32 =>,
          //33 =>,
          //34 =>,
          _ =>println!("Opcode did not match"),
      }
      self.cpu.pc += 2;
    }
}
