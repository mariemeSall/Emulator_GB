
pub mod cpu;
pub mod gpu;
pub mod memory;
use gpu::gameboy::*;
use sdl2::{video::Window, EventPump, event::Event, keyboard::Keycode};

use crate::gpu::{gameboy::GameBoy, gpu::LINE};
use std::{fs::File, time::Duration};


const FPS: u32 = 60;
// une Gameboy execute autant de cycle en 1 seconde
const CYCLES_PER_SECOND: u64 = 4194304;
const CYCLES_PER_FRAME: u64 = CYCLES_PER_SECOND/FPS as u64;


fn open_window()-> Window{
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    video_subsystem
        .window("Game Boy Emulator", SCREEN_WIDTH * SCALE_FACTOR, SCREEN_HEIGHT * SCALE_FACTOR)
        .position_centered()
        .build()
        .unwrap()
}

pub fn handle_input(events: &mut EventPump, gameboy: &mut GameBoy) {
    for event in events.poll_iter() {
        match event {
            Event::Quit{..} => {
                    gameboy.done = true;
                    gameboy.cpu.is_halted = true;
            },
            Event::KeyDown {
                keycode: Some(Keycode::Space),
                ..
            } => {
                gameboy.cpu.is_halted = !gameboy.is_halted(); // Ferme la fenêtre
            }
            _ => ()
        }
    }
} 

fn main() {
    let rom_path = "rom/Tetris.gb";
    let mut gameboy = GameBoy::new();
    let mut step = 1;


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


                while !gameboy.done{
                    handle_input(&mut event_pump, &mut gameboy);

                    while !gameboy.is_halted()&&cycles_this_frame < CYCLES_PER_FRAME*gameboy.get_speed(){
                        //handle_input(&mut event_pump, &mut gameboy);
                        cycles_this_frame += gameboy.step();                       
                    }

                    gameboy.display_screen(&mut canvas);

                    cycles_this_frame =0;
                  //  gameboy.done=true;
                }
               /* while gameboy.memory_bus.bios_run {
                    gameboy.step()
                }*/
               
                //gameboy.run();

              
            
            /*let mut display_count = 0;  //Compteur pour suivre le nombre d'octets affichés
            for byte in rom_data.iter() {  //Parcours chaque octet dans le vecteur rom_data
                print!("{:02X} ", byte);  //Affiche l'octet en format hexadécimal (2 caractères, préfixés par 0 si nécessaire)
                display_count += 1;  //Incrémente le compteur d'octets affichés

                if display_count % 16 == 0 {
                    //Si 16 octets sont affichés on passe à la ligne suivante
                    println!();
                }

                if display_count >= 256 {
                    // Si nous avons affiché 256 octets arrête la boucle
                    break;
                }
            }*/

        }
        Err(e) => {
            eprintln!("Erreur lors de l'ouverture du fichier ROM : {}", e);
        }
    }


}
