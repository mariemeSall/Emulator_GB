
pub mod cpu;
pub mod gpu;
pub mod memory;
use crate::gpu::gameboy::GameBoy;
use std::fs::File;



fn main() {
    let rom_path = "rom/Tetris.gb";
    let mut gameboy = GameBoy::new();


    //Utilise File::open pour ouvrir le fichier ROM en mode lecture
    match File::open(rom_path) {
        Ok(mut file) => {
            //Utilise la fonction read_to_end pour lire le contenu complet du fichier dans un vecteur de bytes
           
                    //Initialise le MemoryBus
                gameboy.load_bios();
                gameboy.load_game(&mut file);
                gameboy.run();

              
            
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
