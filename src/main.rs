use crate::gpu::screen::_screenTest;

pub mod cpu;
pub mod gpu;
use crate::cpu::register::Resgisters;
use std::fs::File;
use std::io::Read;

fn main() {
    let rom_path = "rom/Tetris.gb";

    //Utilise File::open pour ouvrir le fichier ROM en mode lecture
    match File::open(rom_path) {
        Ok(mut file) => {
            //Utilise la fonction read_to_end pour lire le contenu complet du fichier dans un vecteur de bytes
            let mut rom_data = Vec::new();
            match file.read_to_end(&mut rom_data) {
                Ok(_) => {
                    // Vous avez maintenant les données de la ROM dans le vecteur `rom_data`.
                    // Vous pouvez les traiter selon vos besoins, par exemple, les charger dans l'émulateur Game Boy.
                }
                Err(e) => {
                    eprintln!("Erreur lors de la lecture du fichier ROM : {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Erreur lors de l'ouverture du fichier ROM : {}", e);
        }
    }

    _screenTest();
}
