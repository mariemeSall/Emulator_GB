extern crate sdl2;
use crate::cpu::cpu::{CPU, MemoryBus};
use super::gpu::{GPU, PixelColorVal, PixelColorVal::Zero};

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

struct Emulator {
    //cpu: CPU,
    gpu: GPU,
    //memory_bus: MemoryBus,
}

impl Emulator {
    fn new() -> Self {
        Emulator {
            //cpu: CPU::new(),
            gpu: GPU::new(),
            //memory_bus: MemoryBus::new(),
        }
    }
    fn step(&mut self) {
        // Exécutez une étape de l'émulateur ici.
        // Par exemple, vous pouvez mettre à jour le CPU, le GPU, la mémoire, etc.
        //self.cpu.step();
        //self.gpu.step();
    }

    fn get_screen_data(&self) -> [[PixelColorVal; SCREEN_WIDTH as usize]; SCREEN_HEIGHT as usize] {
        // Obtenez les données d'écran de l'émulateur ici.
        // Par exemple, vous pouvez récupérer les données du GPU.
        self.gpu.get_screen_data()
    }

    // Implémentez les fonctions nécessaires pour faire fonctionner l'émulateur.
    // Gérer la mise à jour de l'écran ici.
}

pub fn screenTest() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Game Boy Emulator", SCREEN_WIDTH * SCALE_FACTOR, SCREEN_HEIGHT * SCALE_FACTOR)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut emulator = Emulator::new();

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

        emulator.step();

        let screen_data = emulator.get_screen_data();

        for y in 0..SCREEN_HEIGHT as usize {
            for x in 0..SCREEN_WIDTH as usize {
                let pixel_color = screen_data[y][x];
                let color = match pixel_color {
                    Zero => Color::BLACK,
                    _ => Color::WHITE, //TODO : Définir notre propre palette de couleur
                };
                canvas.set_draw_color(color);
                canvas.fill_rect(Rect::new(
                    (x as u32 * SCALE_FACTOR) as i32,
                    (y as u32 * SCALE_FACTOR) as i32,
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