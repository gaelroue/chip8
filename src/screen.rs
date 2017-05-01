use sdl2::render::{Renderer};
use sdl2::{Sdl};
use sdl2::rect::{Rect};
use sdl2::pixels::{Color};
use num::range_step;

pub const BLACK: usize = 0;
pub const WHITE: usize = 1;
pub const WIDTH: usize = 64;
pub const HEIGHT: usize = 32;
pub const DIMPIXEL: usize = 8;
const SCREEN_W: usize = WIDTH * DIMPIXEL;
const SCREEN_H: usize = HEIGHT * DIMPIXEL;


pub struct Screen<'a> {
    pub renderer: Renderer<'a>,
    pub buffer: [u8; WIDTH*HEIGHT],
}

impl<'a> Screen<'a> { 
    pub fn new(ctx: &Sdl) -> Screen<'a> {
        let video   = ctx.video().unwrap();
        let window  = video.window("chip8", SCREEN_W as u32, SCREEN_H as u32)
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        Screen {
            renderer: window.renderer().build().unwrap(),
            buffer: [BLACK as u8; WIDTH*HEIGHT]
        }
    }

    pub fn update(&mut self) {
        // iter over displayed screen
       for h in range_step(0, SCREEN_H, DIMPIXEL) {
           for w in range_step(0, SCREEN_W, DIMPIXEL) {
               // Check buffer to see if pixel is on or not
                if self.buffer[(w/DIMPIXEL) + (h/DIMPIXEL) * WIDTH] == WHITE as u8 {
                    self.set_color(WHITE);
                    self.renderer.fill_rect(
                        Rect::new(w as i32, h as i32, DIMPIXEL as u32, DIMPIXEL as u32)
                    );
                } else {
                    self.set_color(BLACK);
                    self.renderer.fill_rect(
                        Rect::new(w as i32, h as i32, DIMPIXEL as u32, DIMPIXEL as u32)
                    );
                }
            }
        }
        self.renderer.present();
    }

    pub fn clear(&mut self) { 
        self.buffer = [BLACK as u8; WIDTH*HEIGHT];
    }

    // Set renderer color
    fn set_color(&mut self, color:usize) {
        if color == BLACK {
            self.renderer.set_draw_color(Color::RGB(0, 0, 0));
        } else if color == WHITE {
            self.renderer.set_draw_color(Color::RGB(255, 255, 255));
        } else {
            println!("This is not a valid color");
        }
    }
}
