extern crate sdl2;
use crate::cpu::cpu::CPU;
use crate::gpu::gpu::VRAM_START;
use crate::memory::memory::MemoryBus;

use super::gpu::{GPU, PixelColorVal};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::time::Duration;

pub const SCREEN_WIDTH: u32 = 160;
pub const SCREEN_HEIGHT: u32 = 144;
pub const SCALE_FACTOR: u32 = 5;

pub struct GameBoy {
    pub cpu: CPU,
    pub gpu: GPU,
    pub memory_bus: MemoryBus,
    pub screen_is_open: bool,
}

impl GameBoy {
    pub fn new() -> Self {
        GameBoy {
            cpu: CPU::new(),
            gpu: GPU::new(),
            memory_bus: MemoryBus::new(),
            screen_is_open: false,
        }
       
    }
    pub fn step(&mut self) {
        // Exécutez une étape de l'émulateur ici
        // Par exemple, vous pouvez mettre à jour le CPU, le GPU, la mémoire, etc
        self.gpu.step(&mut self.memory_bus);
        self.cpu.step(&mut self.memory_bus);
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
            

            self.step();

            // Met à jour l'affichage sur l'écran SDL2
            self.draw_screen(&mut canvas);
            canvas.present();

            std::thread::sleep(Duration::new(0, 1_000_000_000 / 60));
        }
    }


    fn draw_screen(&mut self, canvas: &mut Canvas<Window>){
        let screen = self.gpu.screen;
        for pixel_y in 0..144 {
            for pixel_x in 0..160 {
                let pixel_color = match screen[pixel_y][pixel_x] {
                    PixelColorVal::Zero => Color::BLACK,
                    PixelColorVal::One => Color { r: 190, g: 190, b: 190, a: 255 }, // light grey
                    PixelColorVal::Two => Color { r: 80, g: 80, b: 80, a: 255 }, // dark grey
                    PixelColorVal::Three => Color::WHITE,
                };
                 //Dessine le pixel sur le canvas
                 canvas.set_draw_color(pixel_color);
                 canvas
                     .fill_rect(Rect::new(
                         (pixel_x as i32) * SCALE_FACTOR as i32,
                         (pixel_y as i32) * SCALE_FACTOR as i32,
                         SCALE_FACTOR as u32,
                         SCALE_FACTOR as u32,
                     ))
                     .expect("Failed to draw pixel.");

            }
        }

    }

    pub fn draw_tile_set(&mut self, canvas: &mut Canvas<Window>) {
        // Loop a travers tile_set
        for tile_index in 0..384 {
            for row_index in 0..8 {
                for pixel_index in 0..8 {
                    // Determine la couleur du pixel selon la valeur
                    let pixel_color = match self.gpu.tile_set[tile_index][row_index][pixel_index] {
                        PixelColorVal::Zero => Color::BLACK,
                        PixelColorVal::One => Color { r: 190, g: 190, b: 190, a: 255 }, // light grey
                        PixelColorVal::Two => Color { r: 80, g: 80, b: 80, a: 255 }, // dark grey
                        PixelColorVal::Three => Color::WHITE,
                    };

                    // Calcule les coordonnées pour dessiner le pixel
                    let x = (tile_index % 20) * 8 + pixel_index;
                    let y = (tile_index / 20) * 8 + row_index;

                    //Dessine le pixel sur le canvas
                    canvas.set_draw_color(pixel_color);
                    canvas
                        .fill_rect(Rect::new(
                            (x as i32) * SCALE_FACTOR as i32,
                            (y as i32) * SCALE_FACTOR as i32,
                            SCALE_FACTOR as u32,
                            SCALE_FACTOR as u32,
                        ))
                        .expect("Failed to draw pixel.");
                }
            }
        }
    }

    pub fn load_game(&mut self, game_file: &mut File ){
        let mut header = [0; 0x150];
		let _ = game_file.read(&mut header).unwrap();
        let _ =game_file.seek(SeekFrom::Start(0));


        let title = to_null_terminated(&header[0x134..0x144]);

		println!("The title of the game is {}", title);

        let cart_type = header[0x147];

        match cart_type {
            0 =>  println!("The cartridge type is ROM ONLY"),
            1..=3=> println!("The cartridge type is MBC1"),
            _=> println!("The cartridge type is NOT IMPLEMENTED"),
        }

        for i in 0..48 {
            let value = header[0x0104 +i];
            self.memory_bus.vram[i+0x1800]  = value;
        }


        self.memory_bus.load_data(game_file);
		let rom_size = header[0x148] ;
        let rom_actual = 32 * (1<<rom_size);

        println!("ROM size : {:}Kb", rom_actual);
        
        let ram_size = header[0x149]as u16 *1024;

        println!("RAM size : {:}", ram_size);
    }

   
    pub fn load_bios(&mut self){
       

        self.cpu.resgiters.set_af(0x190);
        self.cpu.resgiters.set_bc(0x13);
        self.cpu.resgiters.set_de(0xD8);
        self.cpu.resgiters.set_hl(0x14D);

        self.cpu.sp = 0xFFFE;
        self.cpu.pc = 0x0100;

        self.memory_bus.write_byte(0xFF05, 0x00);
        self.memory_bus.write_byte(0xFF06, 0x00);
        self.memory_bus.write_byte(0xFF07, 0x00);
        self.memory_bus.write_byte(0xFF10, 0x80);
        self.memory_bus.write_byte(0xFF11, 0xBF);
        self.memory_bus.write_byte(0xFF12, 0xF3);
        self.memory_bus.write_byte(0xFF14, 0xBF);
        self.memory_bus.write_byte(0xFF16, 0x3F);
        self.memory_bus.write_byte(0xFF17, 0x00);
        self.memory_bus.write_byte(0xFF19, 0xBF);
        self.memory_bus.write_byte(0xFF1A, 0x7F);
        self.memory_bus.write_byte(0xFF1B, 0xFF);
        self.memory_bus.write_byte(0xFF1C, 0x9F);
        self.memory_bus.write_byte(0xFF1E, 0xBF);
        self.memory_bus.write_byte(0xFF20, 0xFF);
        self.memory_bus.write_byte(0xFF21, 0x00);
        self.memory_bus.write_byte(0xFF22, 0x00);
        self.memory_bus.write_byte(0xFF23, 0xBF);
        self.memory_bus.write_byte(0xFF24, 0x77);
        self.memory_bus.write_byte(0xFF25, 0xF3);
        self.memory_bus.write_byte(0xFF26, 0xF1);
        self.memory_bus.write_byte(0xFF40, 0x91);
        self.memory_bus.write_byte(0xFF42, 0x00);
        self.memory_bus.write_byte(0xFF43, 0x00);
        self.memory_bus.write_byte(0xFF45, 0x00);
        self.memory_bus.write_byte(0xFF47, 0xFC);
        self.memory_bus.write_byte(0xFF48, 0xFF);
        self.memory_bus.write_byte(0xFF49, 0xFF);
        self.memory_bus.write_byte(0xFF4A, 0x00);
        self.memory_bus.write_byte(0xFF4B, 0x00);
        self.memory_bus.write_byte(0xFFFF, 0x00);

        self.memory_bus.bios_run = false;

    }

    
}

fn to_null_terminated(bytes: &[u8]) -> String {
	String::from_utf8_lossy(&bytes.iter()
								  .map(|b| *b)
	 							  .take_while(|&b| b > 0)
	 							  .collect::<Vec<_>>())
							.to_string()
}
