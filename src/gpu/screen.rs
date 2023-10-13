extern crate sdl2;
use super::gpu::GPU;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

pub const SCREEN_WIDTH: u32 = 160;
pub const SCREEN_HEIGHT: u32 = 144;
pub const SCALE_FACTOR: u32 = 5;

pub struct GameBoy<'a> {
    //cpu: CPU,
    pub gpu: &'a mut GPU,
    //memory_bus: MemoryBus,
    pub screen_is_open: bool,
}

impl<'a> GameBoy<'a> {
    pub fn new(gpu : &'a mut GPU) -> Self {
        GameBoy {
            //cpu: CPU::new(),
            gpu: gpu,
            //memory_bus: MemoryBus::new(),
            screen_is_open: false,
        }
    }
    pub fn step(&mut self) {
        // Exécutez une étape de l'émulateur ici
        // Par exemple, vous pouvez mettre à jour le CPU, le GPU, la mémoire, etc
        //self.cpu.step();
        //self.gpu.step();
    }

    pub fn run(&mut self) {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window("Game Boy Emulator", SCREEN_WIDTH * SCALE_FACTOR, SCREEN_HEIGHT * SCALE_FACTOR)
            .position_centered()
            .build()
            .unwrap();
        let mut canvas = window.into_canvas().build().unwrap();
        let mut event_pump = sdl_context.event_pump().unwrap();
        self.screen_is_open = true;
        
        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => {
                        self.screen_is_open = false; // Ferme la fenêtre
                        break 'running;
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => {
                        self.screen_is_open = false; // Ferme la fenêtre
                        break 'running;
                    }
                    _ => {}
                }
            }

            if !(self.screen_is_open) {
                break 'running; //Sort de la boucle si la fenêtre est fermée
            }

            canvas.present();

            // Obtenez les données du tile_set depuis le GPU.
            let tile_set = &self.gpu.tile_set;

            // Met à jour l'affichage sur l'écran SDL2
            self.gpu.draw_tile_set(&mut canvas);

            std::thread::sleep(Duration::new(0, 1_000_000_000 / 60));
        }
    }

    /*pub fn get_screen_data(&self) -> [[PixelColorVal; SCREEN_WIDTH as usize]; SCREEN_HEIGHT as usize] {
        self.gpu.get_screen_data()
    }*/
}