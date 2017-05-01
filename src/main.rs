extern crate sdl2;
extern crate num;

mod cpu;
mod screen;
mod events;
mod instructions;
mod chip8;

use std::thread;
use std::time::Duration;
use std::fs::File;
use std::io::prelude::*;


use events::Events;
use chip8::CHIP8;

const MEMORY_SIZE: usize = 4096;
const START_ADDR: u16 = 512;
const FPS: usize = 16;
const CPU_SPEED : usize = 4;


fn main() {
    let ctx = sdl2::init().unwrap();
    //let mut display = Screen::new(&ctx);
    
    let mut chip = CHIP8::new(&ctx);
    // Prepare the events record
    let mut events = Events::new(ctx.event_pump().unwrap());


    //let file_path = "/home/gael/Programming/rust/chip8/GAMES/MAZE.ch8";
    let file_path = "/home/gael/Programming/rust/chip8/GAMES/TEST/C8PIC.ch8";
    let mut file = File::open(file_path).unwrap();
    let mut game = file.take((MEMORY_SIZE - START_ADDR as usize) as u64);
    game.read(&mut chip.cpu.memory[(START_ADDR as usize) .. MEMORY_SIZE]).unwrap();

    let mut opcode: u16 = 0;

    //chip.screen.clear();
    loop {
        events.pump();

        if events.quit || events.key_escape {
            break;
        }

        for i in 0..CPU_SPEED {
            opcode = chip.cpu.get_opcode();
            chip.decode(opcode);
        }
        chip.screen.update();
        chip.cpu.decrease();
        thread::sleep(Duration::from_millis(FPS as u64));
    }
}
