extern crate sdl2;
use crate::cpu::cpu::CPU;
use crate::cpu::timer::Timer;
use crate::memory::memory::MemoryBus;
use super::inputs::{Keypad, JoypadKey};

use super::gpu::{GPU, PixelColorVal};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

pub const SCREEN_WIDTH: u32 = 160;
pub const SCREEN_HEIGHT: u32 = 144;
pub const SCALE_FACTOR: u32 = 5;

pub struct GameBoy {
    pub cpu: CPU,
    pub gpu: GPU,
    pub memory_bus: MemoryBus,
    pub screen_is_open: bool,
    pub done: bool,
    pub keypad: Keypad,
    pub timer : Timer,
}

impl GameBoy {
    pub fn new() -> Self {
        GameBoy {
            cpu: CPU::new(),
            gpu: GPU::new(),
            memory_bus: MemoryBus::new(),
            screen_is_open: false,
            done: false,
            keypad: Keypad::new(),
            timer : Timer::new(),
        }
       
    }
    pub fn step(&mut self) -> u64{
        let cycles= self.cpu.step(&mut self.memory_bus);
        // Exécutez une étape de l'émulateur ici
        // Par exemple, vous pouvez mettre à jour le CPU, le GPU, la mémoire, etc
        if self.cpu.pc == 0x100 {
            self.memory_bus.bios_run=false;
        }
        self.gpu.step(&mut self.memory_bus, &mut self.cpu, cycles as i16);
        self.timer.step(cycles as i16,&mut self.memory_bus, &mut self.cpu);
       
       if self.cpu.interup_step(&mut self.memory_bus){
           self.cpu.is_halted = false;
        }
        
        
        cycles
       
    }

    pub fn is_halted(&self)->bool{
        self.cpu.is_halted
    }

    pub fn display_screen(&mut self, canvas: &mut Canvas<Window>){
    
        self.draw_screen(canvas);
        canvas.present();  
       // std::thread::sleep(Duration::new(0, 1_000_000_000/60 ));

    }
    pub fn get_speed(&mut self) -> u64 {
		1 << (self.memory_bus.read_byte(0xFF4D) >> 7)
	}

    pub fn update_key_state(&mut self, event_pump: &mut sdl2::EventPump) {
        // Efface l'état actuel des touches
        self.keypad.p14 |= 0x0F;
        self.keypad.p15 |= 0x0F;
    
        // Met à jour l'état des touches en fonction des entrées utilisateur
        for event in event_pump.poll_iter() {
            match event {
                Event::KeyDown { keycode, repeat, .. } => {
                    if !repeat {
                        if let Some(keycode) = keycode {
                            match keycode {
                                Keycode::Right => self.keypad.key_down(JoypadKey::Right),
                                Keycode::Left => self.keypad.key_down(JoypadKey::Left),
                                Keycode::Up => self.keypad.key_down(JoypadKey::Up),
                                Keycode::Down => self.keypad.key_down(JoypadKey::Down),
                                Keycode::A => self.keypad.key_down(JoypadKey::A),
                                Keycode::B => self.keypad.key_down(JoypadKey::B),
                                Keycode::S => self.keypad.key_down(JoypadKey::Select),
                                Keycode::X => self.keypad.key_down(JoypadKey::Start),
                                Keycode::Space => {
                                    self.cpu.is_halted = !self.cpu.is_halted;
                                }
                                _ => {}
                            }
                        }
                    }
                }
                Event::KeyUp { keycode, .. } => {
                    if let Some(keycode) = keycode {
                        match keycode {
                            Keycode::Right => self.keypad.key_up(JoypadKey::Right),
                            Keycode::Left => self.keypad.key_up(JoypadKey::Left),
                            Keycode::Up => self.keypad.key_up(JoypadKey::Up),
                            Keycode::Down => self.keypad.key_up(JoypadKey::Down),
                            Keycode::A => self.keypad.key_up(JoypadKey::A),
                            Keycode::B => self.keypad.key_up(JoypadKey::B),
                            Keycode::S => self.keypad.key_up(JoypadKey::Select),
                            Keycode::Space => self.keypad.key_up(JoypadKey::Start),
                            _ => {}
                        }
                    }
                }
                Event::Quit { .. } => {
                    self.screen_is_open = false;
                    self.done = true;
                    self.cpu.is_halted = true;
                },
                _ => {}
            }
        }
    
        // Met à jour le registre JOYPAD avec l'état des touches
        self.memory_bus.write_byte(0xFF00, self.keypad.read_interrupt());
    }

    pub fn draw_screen(&mut self, canvas: &mut Canvas<Window>){
        let screen = self.gpu.screen;
        for pixel_y in 0..144 {
            for pixel_x in 0..160 {
                let pixel_color = match screen[pixel_y][pixel_x] {
                    PixelColorVal::Three => Color::BLACK,
                    PixelColorVal::Two => Color { r: 190, g: 190, b: 190, a: 255 }, // light grey
                    PixelColorVal::One => Color { r: 80, g: 80, b: 80, a: 255 }, // dark grey
                    PixelColorVal::Zero => Color::WHITE,
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

        self.memory_bus.load_data(game_file);
		let rom_size = header[0x148] ;
        let rom_actual = 32 * (1<<rom_size);

        println!("ROM size : {:}Kb", rom_actual);
        
        let ram_size = header[0x149]as u16 *1024;

        println!("RAM size : {:}", ram_size);
    }

    fn logo(&mut self){
        let mut address = 0x8010;
        
        for i in 0x0104 .. 0x134 {
            let top = Self::decompress(self.memory_bus.read_byte(i)>>4);
            let low = Self::decompress(self.memory_bus.read_byte(i)&0xF);
            self.memory_bus.write_byte(address, top);
            self.memory_bus.write_byte(address + 1, top);
            self.memory_bus.write_byte(address + 2, top);
            self.memory_bus.write_byte(address + 3, top);
            self.memory_bus.write_byte(address + 4, low);
            self.memory_bus.write_byte(address + 5, low);
            self.memory_bus.write_byte(address + 6, low);
            self.memory_bus.write_byte(address + 7, low);
            address+=8;
        }
    }

    fn decompress(value:u8)->u8 {
        (((value&8)>>3)<<7)|(((value&8)>>3)<<6)|(((value&4)>>2)<<5)|(((value&4)>>2)<<4)|(((value&2)>>1)<<3)|(((value&2)>>1)<<2)|((value&1)<<1)|(value&1)

    }

   

   
    pub fn load_bios(&mut self){
       

        match File::open("rom/gb_bios.bin") {
			Ok(mut bios_file) => {
				println!("Found BIOS");
				let _ = bios_file.read_to_end(&mut self.memory_bus.bios);
				println!("Successfully loaded bios\n");
			},
			Err(_) => {
				println!("Could not find BIOS");
				println!("Manually initializing emulator...");

                self.cpu.sp = 0xFFFE;
                self.cpu.resgiters.a = 0;
                self.cpu.resgiters.set_hl(0x8010);

                //nettoie la vram
                for i in 0x8000..0xA000 {
                    self.memory_bus.write_byte(i, 0);
                }

                self.memory_bus.write_byte(0xFF47, 0xFC);
                //Met le logo dans la vram
                self.logo();

                let mut a = 25;
                self.memory_bus.write_byte(0x9910, a);
                let mut address = 0x992F;

                for _i in 0..2 {

                    for _j in 0..12 {
                        a-=1;
                        self.memory_bus.write_byte(address, a);
                        address-=1;
                    }
                    
                    address = 0x990F;
                }


                self.memory_bus.write_byte(0xFF42,100);
                self.memory_bus.write_byte(0xFF40, 0x91);
                self.cpu.pc = 0x0100;
            }
        }       
    }

   
}

fn to_null_terminated(bytes: &[u8]) -> String {
	String::from_utf8_lossy(&bytes.iter()
								  .map(|b| *b)
	 							  .take_while(|&b| b > 0)
	 							  .collect::<Vec<_>>())
							.to_string()
}