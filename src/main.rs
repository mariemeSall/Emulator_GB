use crate::gpu::screen::_screenTest;

pub mod cpu;
pub mod gpu;
//use crate::cpu::register::Resgisters;
use crate::gpu::gpu::MemoryBus;
use crate::gpu::gpu::VRAM_START;
use std::fs::File;
use std::io::Read;

fn main() {

    let rom_path = "rom/Tetris.gb";

    //Ouvre le fichier ROM en mode lecture
    match File::open(rom_path) {
        Ok(mut file) => {
            //Utilise la fonction read_to_end pour lire le contenu complet du fichier dans un vecteur de bytes
            let mut rom_data = Vec::new();
            match file.read_to_end(&mut rom_data) {
                Ok(_) => {
                    //Initialise le MemoryBus
                    let mut memory_bus = MemoryBus::new(); 
                    let vram_start = VRAM_START;

                    //Pour chaque byte de la ROM écrit le dans la VRAM
                    for (address, byte) in rom_data.iter().enumerate() {
                        //Utilise la fonction write_byte pour écrire dans la VRAM
                        memory_bus.write_byte((vram_start + address) as u16, *byte);
                    }
                }
                Err(e) => {
                    eprintln!("Erreur lors de la lecture du fichier ROM : {}", e);
                }
            }

            let mut display_count = 0;  //Compteur pour suivre le nombre d'octets affichés
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
            }

        }
        Err(e) => {
            eprintln!("Erreur lors de l'ouverture du fichier ROM : {}", e);
        }
    }
    _screenTest();
}
