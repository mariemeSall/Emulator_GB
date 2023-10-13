use sdl2::sys::True;

pub mod cpu;
pub mod gpu;
use crate::cpu::cpu::CPU;
use crate::gpu::gpu::{MemoryBus, GPU};
use crate::gpu::gpu::VRAM_START;
use crate::gpu::screen::{SCALE_FACTOR, SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::gpu::screen::GameBoy;
use std::fs::File;
use std::io::Read;



fn main() {

    let rom_path = "rom/Tetris.gb";
    let mut rom_data = Vec::new();
    let mut cpu = CPU::new();
    let mut gpu = GPU::new();
    //Utilise File::open pour ouvrir le fichier ROM en mode lecture
    match File::open(rom_path) {
        Ok(mut file) => {
            //Utilise la fonction read_to_end pour lire le contenu complet du fichier dans un vecteur de bytes
            match file.read_to_end(&mut rom_data) {
                Ok(_) => {
                    //Initialise le MemoryBus
                    let mut memory_bus = MemoryBus::new(&mut gpu); 
                    let vram_start = VRAM_START; // Définissez l'adresse de début de la VRAM.

                    //Pour chaque byte de la ROM écrit le dans la VRAM
                    for (address, byte) in rom_data.iter().enumerate() {
                        //Utilise la fonction write_byte pour écrire dans la VRAM
                        memory_bus.write_byte((address) as u16, *byte);
                        //print!("0x{:X} ", address);
                    }

                    for i in 0..20 {
                        memory_bus.gpu.vram[i] = 0xFF;
                    }
                    
                    /* 
                    for hexa in memory_bus.gpu.vram {
                        print!("{:02X} ", hexa)
                    } */
                    /*cpu.bus.load_data(rom_data);  
                    
                    while !cpu.is_halted {
                        cpu.step();
                    }*/
                }
                Err(e) => {
                    eprintln!("Erreur lors de la lecture du fichier ROM : {}", e);
                }
            }
            
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

    let mut gameboy = GameBoy::new(&mut gpu);

    gameboy.run();

}
