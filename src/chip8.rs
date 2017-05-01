extern crate rand;

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
          1 => self.screen.clear(),
          2 => {
              if self.cpu.jump_nbr > 0 {
                  self.cpu.jump_nbr -= 1;
                  self.cpu.pc = self.cpu.jump[self.cpu.jump_nbr as usize];
              }
          },
          3 => {
              self.cpu.pc = ((b3 as u16) << 8) + ((b2 as u16) << 4) + b1 as u16;
              self.cpu.pc -= 2;
          },
          4 => {
              self.cpu.jump[self.cpu.jump_nbr as usize] = self.cpu.pc;

              if self.cpu.jump_nbr < 15 {
                  self.cpu.jump_nbr += 1;
              }

              self.cpu.pc = ((b3 as u16) << 8) + ((b2 as u16) << 4) + b1 as u16;
              self.cpu.pc -= 2;
          },
          5 => {
              if self.cpu.v[b3 as usize] == ((b2 << 4) + b1) {
                  self.cpu.pc += 2;
              }
          },
          6 => {
              if self.cpu.v[b3 as usize] != ((b2 << 4) + b1) {
                  self.cpu.pc += 2;
              }
          },
          7 => {
              if self.cpu.v[b3 as usize] == self.cpu.v[b2 as usize] {
                  self.cpu.pc += 2;
              }
          },
          8 =>{
              self.cpu.v[b3 as usize] = (b2 << 4) + b1;
          },
          9 =>{
              self.cpu.v[b3 as usize] += (b2 << 4) + b1;
          },
          10 => {
              self.cpu.v[b3 as usize] = self.cpu.v[b2 as usize];
          },
          11 => {
              self.cpu.v[b3 as usize] = self.cpu.v[b3 as usize] | self.cpu.v[b2 as usize];
          },
          12 => {
              self.cpu.v[b3 as usize] = self.cpu.v[b3 as usize] & self.cpu.v[b2 as usize];
          },
          13 => {
              self.cpu.v[b3 as usize] = self.cpu.v[b3 as usize] ^ self.cpu.v[b2 as usize];
          },
          14 => {
              if ((self.cpu.v[b3 as usize] as u16) + (self.cpu.v[b2 as usize]) as u16) > 255 {
                  self.cpu.v[0xF] = 1;
              } else {
                  self.cpu.v[0xF] = 0;
              }

              self.cpu.v[b3 as usize] += self.cpu.v[b2 as usize];
          },
          15 => {
              if self.cpu.v[b3 as usize] < self.cpu.v[b2 as usize] {
                  self.cpu.v[0xF] = 0;
              } else {
                  self.cpu.v[0xF] = 1;
              }

              self.cpu.v[b3 as usize] -= self.cpu.v[b2 as usize];
          },
          16 => {
                  self.cpu.v[0xF] = self.cpu.v[b3 as usize] & 0x01;
                  self.cpu.v[b3 as usize] = self.cpu.v[b3 as usize] >> 1;
          }
          17 => {
              if self.cpu.v[b2 as usize] < self.cpu.v[b3 as usize] {
                  self.cpu.v[0xF] = 0;
              } else {
                  self.cpu.v[0xF] = 1;
              }
              self.cpu.v[b3 as usize] = self.cpu.v[b2 as usize] - self.cpu.v[b3 as usize];
          }
          18 => {
              self.cpu.v[0xF] = self.cpu.v[b3 as usize] >> 7;
              self.cpu.v[b3 as usize] = self.cpu.v[b3 as usize] << 1;
          },
          19 => {
              if self.cpu.v[b3 as usize] != self.cpu.v[b2 as usize] {
                  self.cpu.pc += 2;
              }
          },
          20 => {
              self.cpu.i = ((b3 as u16) << 8) + ((b2 as u16) << 4) + b1 as u16;
          },
          21 => {
              self.cpu.pc = ((b3 as u16) << 8) + ((b2 as u16) << 4) + b1 as u16 + self.cpu.v[0] as u16;
              self.cpu.pc -= 2;
          },
          22 => {
              self.cpu.v[b3 as usize] = rand::random::<u8>()%(((b2 as u8) << 4) + b1 as u8 + 1);
          },
          23 => self.draw(b1, b2, b3),
          //24 =>,
          //25 =>,
          26 => {
              self.cpu.v[b3 as usize] = self.cpu.system_counter;
          },
          //27 =>,
          28 => {
              self.cpu.system_counter = self.cpu.v[b3 as usize]; 
          }
          29 => {
              self.cpu.sound_counter = self.cpu.v[b3 as usize]; 
          }
          30 =>{
              if (self.cpu.i + self.cpu.v[b3 as usize] as u16) > 0xFFF {
                  self.cpu.v[0xF] = 1;
              } else {
                  self.cpu.v[0xF] = 0;
              }

              self.cpu.i += self.cpu.v[b3 as usize] as u16;
          },
          31 => {
              self.cpu.i = (self.cpu.v[b3 as usize] as u16)* 5;
          },
          32 =>{
                  self.cpu.memory[self.cpu.i as usize] = (self.cpu.v[b3 as usize] - self.cpu.v[b3 as usize]%100)/100;
                  self.cpu.memory[(self.cpu.i as usize) + 1] = ((self.cpu.v[b3 as usize] - self.cpu.v[b3 as usize]%10)/10)%10;
                  self.cpu.memory[(self.cpu.i as usize) + 2] = self.cpu.v[b3 as usize] - self.cpu.memory[self.cpu.i as usize] * 100 - 10 * self.cpu.memory[(self.cpu.i as usize) + 1];
          },
          33 => {
              for i in 0..b3 as usize{
                  self.cpu.memory[(self.cpu.i as usize) + i] = self.cpu.v[i];
              }
          },
          34 => {
              for i in 0..b3 as usize{
                   self.cpu.v[i] = self.cpu.memory[(self.cpu.i as usize) + i];
              }
          },
          _ =>println!("Opcode did not match"),
      }
      self.cpu.pc += 2;
    }
}
