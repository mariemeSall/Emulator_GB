//première adresse consacrée aux tuiles
pub const VRAM_START: usize = 0x8000;
//dernière adresse consacrée aux tuiles
pub const VRAM_END: usize = 0x9FFF;
//taille utilisée par les données des tuiles
pub const VRAM_SIZE: usize = VRAM_END - VRAM_START + 1;
use crate::gpu::screen::{SCREEN_WIDTH, SCREEN_HEIGHT};

#[derive(Copy, Clone)]
pub enum PixelColorVal{
    Zero,
    One,
    Two,
    Three,
}

//tableau de 8x8 pour une tuile
type Tile = [[PixelColorVal; 8]; 8 ];

//Met les valeurs des pixels à zero : noir
pub fn black_tile() -> Tile{
    [[PixelColorVal::Zero; 8]; 8]
}

pub struct GPU{
    //video ram
    vram: [u8; VRAM_SIZE],
    tile_set: [Tile; 384],
}

impl GPU {
    pub fn new() -> Self {
        GPU {
            vram: [0; VRAM_SIZE],
            tile_set: [black_tile(); 384],
        }
    }

    pub fn read_vram(&self, address: usize) -> u8{
        self.vram[address]
    }

    pub fn write_vram(&mut self, address: usize, value: u8) {
        //Si l'index de l'addresse est supérieur ou égal à 0x1800, nous n'écrivons pas dans le stockage des tuiles
        if address >= 0x1800 {
            return;
        }

        //Écrit la valeur dans la mémoire VRAM à l'addresse 
        self.vram[address] = value;

        //Une ligne de tuiles est encodée sur deux 2 bytes, le premier octet est toujours une adresse paire
        //En utilisant un & avec 0xFFFE, on obtient l'adresse du premier octet
        let normalized_index = address & 0xFFFE;

        //Les 2 bytes de la ligne de tuiles
        let byte1 = self.vram[normalized_index];
        let byte2 = self.vram[normalized_index + 1];

        //Une tuile mesure 16 octets au total
        let tile_index = address / 16;
        //Tous les deux octets correspond à une nouvelle ligne.
        let row_index = (address % 16) / 2;

        //Boucle pour obtenir les 8 pixels qui composent une ligne donnée
        for pixel_index in 0..8 {
            // 1111 1111
            // 0123 4567
            //Masque pour mettre à 0 les bits qui ne servent pas au codage du pixel
            let mask = 1 << (7 - pixel_index);
            let lsb = byte1 & mask;
            let msb = byte2 & mask;
            
            //Correspondance des valeurs des bits et des couleurs
            let value = match (lsb != 0, msb != 0) {
                (true, true) => PixelColorVal::Three,
                (false, true) => PixelColorVal::Two,
                (true, false) => PixelColorVal::One,
                (false, false) => PixelColorVal::Zero,
            };

            //Affecte la valeur du pixel dans le tableau de tuiles.
            self.tile_set[tile_index][row_index][pixel_index] = value;
        }
    }

    //Récupère les données de l'écran et les renvoie sous forme de matrice de PixelColorVal
    pub fn get_screen_data(&self) -> [[PixelColorVal; SCREEN_WIDTH as usize]; SCREEN_HEIGHT as usize] {
        let mut screen_data = [[PixelColorVal::Zero; SCREEN_WIDTH as usize]; SCREEN_HEIGHT as usize];

        // Vous devrez implémenter la logique pour obtenir les données de l'écran à partir de votre GPU.
        // Par exemple, si vous avez une mémoire vidéo VRAM et un écran de 160x144 pixels,
        // vous pouvez itérer à travers la VRAM pour remplir screen_data avec les valeurs appropriées.

        // Exemple simplifié pour le rendre fonctionnel :
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                // Remplacez cette logique par la récupération réelle des données de l'écran depuis votre GPU.
                screen_data[y as usize][x as usize] = self.tile_set[0][0][0];
            }
        }

        screen_data
    }
}

struct MemoryBus {
    gpu: GPU
}

impl MemoryBus{
    pub fn new() -> Self {
        MemoryBus { gpu: GPU::new() }
    }

    //Lit un byte à partir d'une adresse donnée
    pub fn read_byte(&self, address: u16) -> u8 {
        //Convertie en usize pour le match
        let address = address as usize;
        //Lit le byte différemment selon son emplacement mémoire
        match address{
            //Video RAM
            VRAM_START ..= VRAM_END => {
                self.gpu.read_vram(address - VRAM_START)
            }
            _ => panic!("TODO; support others areas of the memory")
        }
    }

    pub fn write_byte(&mut self, address: u16, value : u8){
        let address = address as usize;
        match address {
            VRAM_START ..= VRAM_END => {
                self.gpu.write_vram(address - VRAM_START, value)
            }
            _ => panic!("TODO; support others areas of the memory")
        }
    }
}
