
pub mod cpu;
pub mod gpu;
pub mod memory;
use gpu::gameboy::*;
use sdl2::{video::Window, EventPump, event::Event, keyboard::Keycode};

use crate::gpu::{gameboy::GameBoy, gpu::LINE};
use std::{fs::File, time::Duration};
extern crate fps_clock;


const FPS: u32 = 60;
// une Gameboy execute autant de cycle en 1 seconde
const CYCLES_PER_SECOND: u64 = 4194304;
const CYCLES_PER_FRAME: u64 = CYCLES_PER_SECOND/FPS as u64;




fn main() {
    let rom_path = "rom/Tetris.gb";
    let mut gameboy = GameBoy::new();


    //Utilise File::open pour ouvrir le fichier ROM en mode lecture
    match File::open(rom_path) {
        Ok(mut file) => {
            //Utilise la fonction read_to_end pour lire le contenu complet du fichier dans un vecteur de bytes
           
                //Initialise le MemoryBus
                gameboy.load_game(&mut file);
                gameboy.load_bios();
                let sdl_context = sdl2::init().unwrap();
                let video_subsystem = sdl_context.video().unwrap();
                let window = video_subsystem
                    .window("Game Boy Emulator", SCREEN_WIDTH * SCALE_FACTOR, SCREEN_HEIGHT * SCALE_FACTOR)
                    .position_centered()
                    .build()
                    .unwrap();
                let mut canvas = window.into_canvas().build().unwrap();
                let mut event_pump = sdl_context.event_pump().unwrap();
                let mut cycles_this_frame = 0;
                let mut fps = fps_clock::FpsClock::new(FPS);
                let mut render_frame =0;




                while !gameboy.done{
                    
                    gameboy.update_key_state(&mut event_pump);

                    while !gameboy.is_halted()&&cycles_this_frame < CYCLES_PER_FRAME*gameboy.get_speed(){
                        //handle_input(&mut event_pump, &mut gameboy);
                        cycles_this_frame += gameboy.step();                       
                    }

                    if render_frame==0 {
                        gameboy.display_screen(&mut canvas);
                        fps.tick();
                    }

                    render_frame = (render_frame+1)%gameboy.get_speed();
                    cycles_this_frame =0;
                  //  gameboy.done=true;
                }

        }
        Err(e) => {
            eprintln!("Erreur lors de l'ouverture du fichier ROM : {}", e);
        }
    }


}
