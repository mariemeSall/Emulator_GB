extern crate sdl2;
use crate::cpu::cpu::{CPU, MemoryBus};
use super::gpu::{GPU, PixelColorVal, PixelColorVal::Zero, PixelColorVal::One, PixelColorVal::Two, PixelColorVal::Three};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Rect, Point};
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;
use std::time::Duration;

pub const SCREEN_WIDTH: u32 = 160;
pub const SCREEN_HEIGHT: u32 = 144;
pub const SCALE_FACTOR: u32 = 6;

pub struct Emulator {
    pub cpu: CPU,
    pub gpu: GPU,
    //memory_bus: MemoryBus,
}

impl Emulator {
    pub fn new() -> Self {
        Emulator {
            cpu: CPU::new(),
            gpu: GPU::new(),
            //memory_bus: MemoryBus::new(),
        }
    }
    pub fn step(&mut self) {
        // Exécutez une étape de l'émulateur ici.
        // Par exemple, vous pouvez mettre à jour le CPU, le GPU, la mémoire, etc.
        //self.cpu.step();
        //self.gpu.step();
    }

    pub fn get_screen_data(&self) -> [[PixelColorVal; SCREEN_WIDTH as usize]; SCREEN_HEIGHT as usize] {
        // Obtenez les données d'écran de l'émulateur ici.
        // Par exemple, vous pouvez récupérer les données du GPU.
        self.gpu.create_tile()
    }

    pub fn load_data(&mut self, data: Vec<u8>){
        self.cpu.bus.load_data(data);
        self.gpu.write_vram2(self.cpu.bus.memory);
        

        //self.gpu.write_vram2(self.cpu.bus.memory);
    }

    // Implémentez les fonctions nécessaires pour faire fonctionner l'émulateur.
    // Gérer la mise à jour de l'écran ici.

    pub fn screenTest(&mut self) {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window("Game Boy Emulator", SCREEN_WIDTH * SCALE_FACTOR, SCREEN_HEIGHT * SCALE_FACTOR)
            .position_centered()
            .build()
            .unwrap();
        let mut canvas = window.into_canvas().build().unwrap();
        let mut event_pump = sdl_context.event_pump().unwrap();
    
    
        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => break 'running,
                    Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    _ => {}
                }
            }
            self.step();
    
            let screen_data = self.get_screen_data();
            for y in 0..SCREEN_HEIGHT as usize {
                for x in 0..SCREEN_WIDTH as usize {
                    let pixel_color = screen_data[y][x];
    
                   /* 
                       if x<=7&&y<=7{ match pixel_color {
                            PixelColorVal::Zero => println!("Black"),
                            PixelColorVal::One => print!("Light Grey"),
                            PixelColorVal::Two => println!("Dark Grey"),
                            PixelColorVal::Three => println!("White"),
                        };} */
    
    
                    let color = match pixel_color {
                        Zero => Color::BLACK,
                        One => {
                            if x<=7&&y<=7 { Color {
                                r: 80, g: 190, b: 190, a: 255 
                            }}else {
                                Color { r: 190, g: 190, b: 190, a: 255 }
                            }
                            },    //light grey
                        Two => {
                            if x<=7&&y<=7 { Color {
                                r: 190, g: 80, b: 190, a: 255 
                            }
                        }else {
                                Color { r: 80, g: 80, b: 80, a: 255 }
                            }
                            },      //dark grey
                        Three => Color::WHITE,
                    };
                    canvas.set_draw_color(color);
                    canvas.fill_rect(Rect::new(
                        (x as i32) * SCALE_FACTOR as i32,
                        (y as i32) * SCALE_FACTOR as i32,
                        SCALE_FACTOR,
                        SCALE_FACTOR,
                    ))
                    .expect("Failed to draw pixel.");
    
                }
            }
            canvas.present();
    
    
            std::thread::sleep(Duration::new(0, 1_000_000_000 / 60));
        }
    }

}

