pub const VRAM_START: usize = 0x8000;   //première adresse consacrée aux tuiles
pub const VRAM_END: usize = 0x9FFF;     //dernière adresse consacrée aux tuiles
pub const VRAM_SIZE: usize = VRAM_END - VRAM_START + 1;     //taille utilisée par les données des tuiles
pub const LCDC_ADDR: usize = 0xFF40;    //addresse du lcdc
use crate::gpu::screen::{SCREEN_WIDTH, SCREEN_HEIGHT};
const VRAM_WIDTH: usize = SCREEN_WIDTH as usize;

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
    lcdc: LCDC,
}

impl GPU {
    pub fn new() -> Self {
        GPU {
            vram: [0; VRAM_SIZE],
            tile_set: [black_tile(); 384],
            lcdc: LCDC::new(),
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

        // Les coordonnées de l'écran Game Boy vont de 0 à 159 (horizontalement) et de 0 à 143 (verticalement).
        for y in 0..SCREEN_HEIGHT as usize {
            for x in 0..SCREEN_WIDTH as usize {
                // Calculez l'adresse dans la VRAM en fonction des coordonnées de l'écran.
                let tile_x = x / 8;
                let tile_y = y / 8;
                let tile_offset = (tile_y * (VRAM_WIDTH / 8) + tile_x) * 16; // Chaque tuile fait 16 octets (8x8 pixels).

                // Obtenez les valeurs des pixels à partir de la VRAM.
                let row_within_tile = y % 8;
                let pixel_within_tile = x % 8;

                let lsb_byte = self.vram[tile_offset + (row_within_tile * 2)];
                let msb_byte = self.vram[tile_offset + (row_within_tile * 2) + 1];

                let lsb_bit = (lsb_byte >> (7 - pixel_within_tile)) & 0x01;
                let msb_bit = (msb_byte >> (7 - pixel_within_tile)) & 0x01;

                // Calculez la valeur du pixel en fonction des bits LSB et MSB.
                let pixel_value = match (msb_bit, lsb_bit) {
                    (0, 0) => PixelColorVal::Zero,
                    (0, 1) => PixelColorVal::One,
                    (1, 0) => PixelColorVal::Two,
                    (1, 1) => PixelColorVal::Three,
                    _ => panic!("Invalid pixel value"), // Gérez les erreurs au besoin.
                };

                screen_data[y][x] = pixel_value;
            }
        }
        screen_data
    }

    fn majAffichage(&mut self) {
        if self.lcdc.display_enable {
            //Affiche le contenu à l'écran en fonction des réglages du LCDC
            if self.lcdc.bg_display_enable {
                //Effectue le rendu du fond en utilisant la carte de tuiles appropriée (bit 3)
                let bg_tile_map: i32;
                if self.lcdc.bg_tile_map == 0 {
                    bg_tile_map = 0x9800 
                } else { 
                    bg_tile_map = 0x9C00 
                };

                let bg_and_window_tile_data: i32;
                if self.lcdc.bg_and_window_tile_data == 0 { 
                    bg_and_window_tile_data = 0x8800 
                } else { 
                    bg_and_window_tile_data = 0x8000 
                };
                // Utilisez le tile_data (bit 4) pour accéder aux données de tuiles du fond (VRAM)

                // Effectuez le rendu du fond en utilisant les coordonnées x et y (position de l'écran)

                // Vous pouvez utiliser d'autres informations du LCDC pour personnaliser davantage le rendu du fond
            }

            // Vérifiez si l'affichage des sprites est activé (bit 1).
            if self.lcdc.sprite_display_enable {
                // Effectuez le rendu des sprites en fonction des coordonnées des sprites dans OAM

                // Vous pouvez également utiliser la taille des sprites (bit 2) pour déterminer la hauteur des sprites

                // Personnalisez davantage le rendu des sprites en fonction des autres bits du LCDC
            }
        }
    }

}

pub struct MemoryBus {
    gpu: GPU,
    lcdc: LCDC
}

impl MemoryBus{
    pub fn new() -> Self {
        MemoryBus { 
            gpu: GPU::new(),
            lcdc: LCDC::new(),
        }
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
            },
            LCDC_ADDR => self.lcdc.read_byte(),
            _ => 0x00,//panic!("TODO: support others areas of the memory")
        }
    }

    pub fn write_byte(&mut self, address: u16, value : u8){
        let address = address as usize;
        match address {
            VRAM_START ..= VRAM_END => {
                self.gpu.write_vram(address - VRAM_START, value)
            },
            LCDC_ADDR => self.lcdc.write_byte(value),
            _ => {}//panic!("TODO: support others areas of the memory")
        }
    }
}

pub struct LCDC {
    display_enable: bool,       // Bit 7
    window_tile_map: usize,     // Bit 6
    window_display_enable: bool, // Bit 5
    bg_and_window_tile_data: usize, // Bit 4
    bg_tile_map: usize,         // Bit 3
    sprite_size: usize,         // Bit 2
    sprite_display_enable: bool, // Bit 1
    bg_display_enable: bool,    // Bit 0
}

impl LCDC {
    pub fn new() -> Self{
        LCDC { 
            display_enable: false,
            window_tile_map: 0,
            window_display_enable: false,
            bg_and_window_tile_data: 0,
            bg_tile_map: 0,
            sprite_size: 0,
            sprite_display_enable: false,
            bg_display_enable: false,
        }
    }

    pub fn read_byte(&self) -> u8 {
        let mut result: u8 = 0;
    
        //Bit 7 : Display Enable
        //Si display_enable est vrai, le bit 7 est mis à 1, sinon il reste à 0
        if self.display_enable {
            result |= 0x80;
        } else {
            result &= !0x80;
        }
    
        //Bit 6 : Window Tile Map Display Select
        //Les bits de l'octet 6 sont copiés depuis window_tile_map
        result |= (self.window_tile_map as u8) << 6;
    
        // Bit 5 : Window Display Enable
        //Si window_display_enable est vrai le bit 5 est mis à 1, sinon il reste à 0
        if self.window_display_enable {
            result |= 0x20;
        } else {
            result &= !0x20;
        }
    
        //Bit 4 : BG and Window Tile Data Select
        //Les bits de l'octet 4 sont copiés depuis bg_and_window_tile_data
        result |= (self.bg_and_window_tile_data as u8) << 4;
    
        //Bit 3 : BG Tile Map Display Select
        //Les bits l'octet 3 sont copiés depuis bg_tile_map
        result |= (self.bg_tile_map as u8) << 3;
    
        //Bit 2 : Sprite Size
        //Les bits de l'octet 2 sont copiés depuis sprite_size
        result |= (self.sprite_size as u8) << 2;
    
        //Bit 1 : Sprite Display Enable
        //Si sprite_display_enable est vrai le bit 1 est mis à 1, sinon il reste à 0
        if self.sprite_display_enable {
            result |= 0x02;
        } else {
            result &= !0x02;
        }
    
        //Bit 0 : BG Display Enable
        //Si bg_display_enable est vrai le bit 0 est mis à 1, sinon il reste à 0
        if self.bg_display_enable {
            result |= 0x01;
        } else {
            result &= !0x01;
        }
    
        //La valeur finale du lcdc est renvoyée
        result
    }
    
    pub fn write_byte(&mut self, value: u8) {
        //Bit 7 : Display Enable
        //Récupère le bit 7 de la valeur entrée, change la valeur de display_enable
        self.display_enable = (value & 0x80) != 0;
    
        //Bit 6 : Window Tile Map Display Select
        //Le bit 6 est extrait de la valeur entrée et mis dans window_tile_map
        self.window_tile_map = ((value >> 6) & 0x01) as usize;
    
        //Bit 5 : Window Display Enable
        //Récupère le bit 5 de la valeur entrée, change la valeur de window_display_enable
        self.window_display_enable = (value & 0x20) != 0;
    
        //Bit 4 : BG and Window Tile Data Select
        //Le bit 4 est extrait de la valeur entrée et mis dans bg_and_window_tile_data
        self.bg_and_window_tile_data = ((value >> 4) & 0x01) as usize;
    
        //Bit 3 : BG Tile Map Display Select
        //Le bit 3 est extrait de la valeur entrée et mis dans bg_tile_map
        self.bg_tile_map = ((value >> 3) & 0x01) as usize;
    
        //Bit 2 : Sprite Size
        //Le bit 2 est extrait de la valeur entrée et mis dans sprite_size
        self.sprite_size = ((value >> 2) & 0x01) as usize;
    
        //Bit 1 : Sprite Display Enable
        //Récupère le bit 1 de la valeur entrée, change la valeur de sprite_display_enable
        self.sprite_display_enable = (value & 0x02) != 0;
    
        //Bit 0 : BG Display Enable
        //Récupère le bit 0 de la valeur entrée, change la valeur de bg_display_enable
        self.bg_display_enable = (value & 0x01) != 0;
    }
    
}
    