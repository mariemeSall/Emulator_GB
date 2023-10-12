extern crate sdl2;
use crate::cpu::cpu::{CPU, MemoryBus};
use super::gpu::{GPU, PixelColorVal, PixelColorVal::Zero, PixelColorVal::One, PixelColorVal::Two, PixelColorVal::Three};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;
use std::time::Duration;

pub const SCREEN_WIDTH: u32 = 160;
pub const SCREEN_HEIGHT: u32 = 144;
pub const SCALE_FACTOR: u32 = 4;

pub struct GameBoy {
    //cpu: CPU,
    gpu: GPU,
    //memory_bus: MemoryBus,
}

impl GameBoy {
    pub fn new() -> Self {
        GameBoy {
            //cpu: CPU::new(),
            gpu: GPU::new(),
            //memory_bus: MemoryBus::new(),
        }
    }
    pub fn step(&mut self) {
        // Exécutez une étape de l'émulateur ici
        // Par exemple, vous pouvez mettre à jour le CPU, le GPU, la mémoire, etc
        //self.cpu.step();
        //self.gpu.step();
    }

    pub fn get_screen_data(&self) -> [[PixelColorVal; SCREEN_WIDTH as usize]; SCREEN_HEIGHT as usize] {
        self.gpu.get_screen_data()
    }

    // Implémentez les fonctions nécessaires pour faire fonctionner l'émulateur
    // Gérer la mise à jour de l'écran ici
}

pub struct Screen {
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
    event_pump: sdl2::EventPump,
}

impl Screen {
    pub fn new(
        sdl_context: &sdl2::Sdl,
        scale_factor: u32,
        screen_width: u32,
        screen_height: u32,
    ) -> Screen {
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window("Game Boy Emulator", screen_width * scale_factor, screen_height * scale_factor)
            .position_centered()
            .build()
            .unwrap();
        let canvas = window.into_canvas().build().unwrap();
        let event_pump = sdl_context.event_pump().unwrap();

        Screen { 
            canvas, event_pump 
        }
    }

    pub fn run(&mut self, gameboy: &mut GameBoy, scale_factor: u32) {
        'running: loop {
            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => break 'running,
                    Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    _ => {}
                }
            }

            gameboy.step();

            let screen_data = gameboy.get_screen_data();

            for y in 0..SCREEN_HEIGHT as usize {
                for x in 0..SCREEN_WIDTH as usize {
                    let pixel_color = screen_data[y][x];
                    let color = match pixel_color {
                        PixelColorVal::Zero => Color::BLACK,
                        PixelColorVal::One => Color {
                            r: 190,
                            g: 190,
                            b: 190,
                            a: 255,
                        }, //light grey
                        PixelColorVal::Two => Color {
                            r: 80,
                            g: 80,
                            b: 80,
                            a: 255,
                        }, //dark grey
                        PixelColorVal::Three => Color::WHITE,
                    };
                    self.canvas.set_draw_color(color);
                    self.canvas
                        .fill_rect(Rect::new(
                            (x as u32 * scale_factor) as i32,
                            (y as u32 * scale_factor) as i32,
                            scale_factor,
                            scale_factor,
                        ))
                        .expect("Failed to draw pixel.");
                }
            }

            self.canvas.present();

            std::thread::sleep(Duration::new(0, 1_000_000_000 / 60));
        }
    }
}
