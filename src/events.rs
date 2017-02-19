use sdl2::EventPump;
use sdl2::event::Event::*;
use sdl2::keyboard::Keycode::*;

pub struct Events{
    pump: EventPump,
    pub quit: bool,
    pub key_escape: bool
}

impl Events {
    pub fn new(pump: EventPump) -> Events {
        Events{
            pump: pump,
            quit: false,
            key_escape: false
        }
    }

    pub fn pump(&mut self) {
        for event in self.pump.poll_iter() {

            match event {
                Quit {..} => self.quit = true,

                // Key event does not seem to work, see issue #101 on github
                //
                KeyDown {keycode, .. } => match keycode {
                    Some(Escape) => self.key_escape = true,
                    _ => {}
                },

                KeyUp {keycode, .. } => match keycode {
                    Some(Escape) => self.key_escape = true,
                    _ => {}
                },

                _ => {}
            }
        }
    }
}
