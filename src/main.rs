extern crate sdl2;

mod cpu;
mod screen;
mod events;
mod instructions;
mod chip8;

use std::thread;
use std::time::Duration;

use events::Events;
use chip8::CHIP8;

const FPS: usize = 16;

fn main() {
    let ctx = sdl2::init().unwrap();
    //let mut display = Screen::new(&ctx);
    
    let mut chip = CHIP8::new(&ctx);
    // Prepare the events record
    let mut events = Events::new(ctx.event_pump().unwrap());

    loop {
        events.pump();

        if events.quit || events.key_escape {
            break;
        }

        //display.update();
        chip.screen.update();
        thread::sleep(Duration::from_millis(FPS as u64));
    }
}
