pub const VRAM_START: usize = 0x8000;   //première adresse consacrée aux tuiles
pub const VRAM_END: usize = 0x9FFF;     //dernière adresse consacrée aux tuiles
pub const VRAM_SIZE: usize = VRAM_END - VRAM_START + 1;     //taille utilisée par les données des tuiles
pub const LCDC_ADDR: usize = 0xFF40;    //addresse du lcdc

use crate::gpu::gameboy::SCALE_FACTOR;

use sdl2::render::Canvas;
use sdl2::pixels::Color;
use sdl2::video::Window;
use sdl2::rect::Rect;

#[derive(Copy, Clone, Debug)]
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
    pub vram: [u8; VRAM_SIZE],
    pub tile_set: [Tile; 384],
    pub lcdc: LCDC,
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
        //print!("{: } ", value);

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

        /*for i in 0 .. 8 {
            self.tile_set[tile_index][i][i] = PixelColorVal::Two;
        }*/
    }

    pub fn draw_tile_set(&mut self, canvas: &mut Canvas<Window>) {
        // Loop a travers tile_set
        for tile_index in 0..384 {
            for row_index in 0..8 {
                for pixel_index in 0..8 {
                    // Determine la couleur du pixel selon la valeur
                    let pixel_color = match self.tile_set[tile_index][row_index][pixel_index] {
                        PixelColorVal::Zero => Color { r: 50, g: 100, b: 190, a: 255 },
                        PixelColorVal::One => Color { r: 200, g: 100, b: 50, a: 255 }, // light grey
                        PixelColorVal::Two => Color { r: 35, g: 200, b: 70, a: 255 }, // dark grey
                        PixelColorVal::Three => Color { r: 60, g: 150, b: 50, a: 255 },
                    };

                    // Calcule les coordonnées pour dessiner le pixel
                    let x = (tile_index % 20) * 8 + pixel_index;
                    let y = (tile_index / 20) * 8 + row_index;

                    //Dessine le pixel sur le canvas
                    canvas.set_draw_color(pixel_color);
                    canvas
                        .fill_rect(Rect::new(
                            (x as i32) * SCALE_FACTOR as i32,
                            (y as i32) * SCALE_FACTOR as i32,
                            SCALE_FACTOR as u32,
                            SCALE_FACTOR as u32,
                        ))
                        .expect("Failed to draw pixel.");
                }
            }
        }
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

pub struct MemoryBus<'a> {
    pub gpu: &'a mut GPU,
    pub lcdc: LCDC
}

impl<'a> MemoryBus<'a>{
    pub fn new( gpu : &'a mut GPU) -> Self {
        MemoryBus { 
            gpu: gpu,
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
                self.gpu.read_vram(address)
            },
            LCDC_ADDR => self.lcdc.read_byte(),
            _ => 0x00,//panic!("TODO: support others areas of the memory")
        }
    }

    pub fn write_byte(&mut self, address: u16, value : u8){
        let address = address as usize;

        match address {
            VRAM_START ..= VRAM_END => {
                self.gpu.write_vram(address , value)
            },
            LCDC_ADDR => self.lcdc.write_byte(value),
            _ => {if address< 0x1800 {
                self.gpu.write_vram(address , value);
                //print!("{}", value);
            }}//panic!("TODO: support others areas of the memory")
        }
    }
}

pub struct LCDC {
    pub display_enable: bool,       // Bit 7
    pub window_tile_map: usize,     // Bit 6
    pub window_display_enable: bool, // Bit 5
    pub bg_and_window_tile_data: usize, // Bit 4
    pub bg_tile_map: usize,         // Bit 3
    pub sprite_size: usize,         // Bit 2
    pub sprite_display_enable: bool, // Bit 1
    pub bg_display_enable: bool,    // Bit 0
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
    