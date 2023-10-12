#[derive(Debug, Copy, Clone)]
struct Sprite{
    y: u8,         //Coordonnée y de la sprite (0-255).
    x: u8,         //Coordonnée x de la sprite (0-255).
    tile_number: u8,  //Numéro de la tuile de la sprite (0-255).
    flags: u8,      // Flags pour les attributs (priorité, orientation, palette, etc.).
}

impl Sprite {
    fn new() -> Self {
        Sprite{
            y: 0,
            x: 0,
            tile_number: 0,
            flags: 0,
        }
    }
}

struct OAM {
    data: [Sprite; 40], //Tableau de 40 entrées pour les sprites
}

impl OAM {
    fn new() -> Self {
        //Initialisez l'OAM avec des valeurs par défaut pour chaque sprite
        OAM {
            data: [Sprite::new(); 40],
        }
    }

    fn read(&self, index: usize) -> Sprite {
        //Lit les attributs de la sprite à l'index donné
        if index < 40 {
            self.data[index]
        } else {
            //Gere les erreurs si l'index est en dehors de la plage
            panic!("Invalid OAM index");
        }
    }

    fn write(&mut self, index: usize, attribute: SpriteAttribute) {
        //Écrit les attributs de la sprite à l'index donné
        if index < 40 {
            self.data[index] = attribute;
        } else {
            //Gere les erreurs si l'index est en dehors de la plage
            panic!("Invalid OAM index");
        }
    }
}
