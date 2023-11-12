pub const VRAM_START: usize = 0x8000;   //première adresse consacrée aux tuiles
pub const VRAM_END: usize = 0x9FFF;     //dernière adresse consacrée aux tuiles
pub const VRAM_SIZE: usize = VRAM_END - VRAM_START + 1;     //taille utilisée par les données des tuiles
pub const LCDC:usize = 0xFF40;
pub const LCDSTAT: u16 = 0xFF41; 
pub const SCROLLX: usize = 0xFF43;
pub const SCROLLY: usize = 0xFF42;
pub const LINE: usize = 0xFF44; //adresse de la ligne à dessiner
pub const WINDOWX: usize = 0xFF4B;
pub const WINDOWY: usize = 0xFF4A;

pub const OAM: usize = 0xFE00; //première adresse de l'OAM
pub const OAM_END: usize = 0xFE9F;
pub const BGP: usize = 0xFF47;
pub const OBP0: usize = 0xFF48;
pub const OBP1: usize = 0xFF49;
pub const IFLAG: u16 = 0xFF0F;

pub const IENABLE: u16 = 0xFFFF;

use crate::gpu::gameboy::SCALE_FACTOR;
use crate::memory::memory::MemoryBus;

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

impl PixelColorVal {
    pub fn color(value:u8, mode: u8)->PixelColorVal{
        let (hi, lo) = (2*value+1, 2*value);
		let color = ((mode & (1 << hi)) >> (hi-1)) | ((mode & (1 << lo)) >> lo);  
       // println!("color {}, value {}", color, value);
        match color {
            3 => PixelColorVal::Three,
            2 => PixelColorVal::Two,
            1 => PixelColorVal::One,
            0 => PixelColorVal::Zero,
            _=> panic!("Invalid color input")
        }


    } 
    
}

//tableau de 8x8 pour une tuile
type Tile = [[PixelColorVal; 8]; 8 ];

//Met les valeurs des pixels à zero : noir
pub fn black_tile() -> Tile{
    [[PixelColorVal::Zero; 8]; 8]
}

pub struct GPU{
    pub tile_set: [Tile; 384],
    pub screen: [[PixelColorVal; 160];144],
    pub bg: [[u8;160];144],
    pub lcdc: LCDC,
}

impl GPU {
    pub fn new() -> Self {
        GPU {
            tile_set: [black_tile(); 384],
            screen: [[PixelColorVal::Zero;160];144],
            bg: [[0; 160]; 144],
            lcdc: LCDC::new(),
        }
    }

    pub fn step(&mut self, memory : &mut MemoryBus ){
        //Passe à la ligne suivante en wrappant la valeur
        let line = (memory.read_byte(LINE) + 1)%154;
        memory.write_byte(LINE, line);
        //update le lcdc 
        self.lcdc.write(memory.read_byte(LCDC));

       // println!("LCDC : {:02X}", memory.read_byte(LCDC));
        //Si la ligne est incluse dans l'affichage (<144), on la dessine
        if line<144 {
            
            self.draw_tiles(memory);
            //Si le lcdc autorise les objets, on dessine les objets
            if self.lcdc.sprite_display_enable {
               // println!("object");
                self.draw_objects(memory);
            }
        }
    }

    pub fn generate_tile_set(&mut self, memory : &mut MemoryBus) {

        for i in VRAM_START..=VRAM_END {
            self.write_vram(memory, i)
        }
    }
   
    pub fn write_vram(&mut self,  memory: &mut MemoryBus, address: usize) {
        if address-VRAM_START >= 1800 {
            return;
        }

        //Une ligne de tuiles est encodée sur deux 2 bytes, le premier octet est toujours une adresse paire
        //En utilisant un & avec 0xFFFE, on obtient l'adresse du premier octet
        let normalized_index = address & 0xFFFE;

        //Les 2 bytes de la ligne de tuiles
        let byte1 = memory.read_vram(normalized_index , false);
        let byte2 = memory.read_vram(normalized_index + 1, false);

        //Une tuile mesure 16 octets au total
        let tile_index = (address-VRAM_START) / 16;

        //Tous les deux octets correspond à une nouvelle ligne.
        let row_index = ((address-VRAM_START) % 16) / 2;

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
                        PixelColorVal::Zero => Color::BLACK,
                        PixelColorVal::One => Color { r: 190, g: 190, b: 190, a: 255 }, // light grey
                        PixelColorVal::Two => Color { r: 80, g: 80, b: 80, a: 255 }, // dark grey
                        PixelColorVal::Three => Color::WHITE,
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

    pub fn draw_tiles(&mut self, memory : &mut MemoryBus){
        self.lcdc.write(memory.read_byte(LCDC));
        let line = memory.read_byte(LINE);
        let scroll_x = memory.read_byte(SCROLLX);
        let scroll_y = memory.read_byte(SCROLLY);
        let window_x = memory.read_byte(WINDOWX).wrapping_sub(7);
        let window_y = memory.read_byte(WINDOWY);

        let background_map_range = if self.lcdc.bg_tile_map {0x9C00}else{0x9800};
        let window_map_range = if self.lcdc.window_tile_map {0x9C00}else{0x9800};
        let bw_tile_data_range = if self.lcdc.bg_display_enable {0x8000}else{0x8800};
        let using_window = self.lcdc.window_display_enable && window_y<=line;
        let background = if using_window {window_map_range} else {background_map_range};

        
        if !using_window && !self.lcdc.bg_display_enable {
            self.bg_prio_zero(line as usize);
            return;
        }
        let y_offset = if using_window { line - window_y} else { scroll_y.wrapping_add(line)};
        
        let tile_row = y_offset/8;
        
        for x in 0..160u8 {
            let mut x_offset = x.wrapping_add(scroll_x);
			if using_window && x >= window_x {
                x_offset = x - window_x;
			}
            
            
            let tile_col = x_offset/8;
            
            let address = background + (tile_row as u16)*32 + (tile_col as u16);
           
            let tile = if self.lcdc.bg_display_enable {
				bw_tile_data_range + (memory.read_vram(address as usize, false) as u16)*16
            } else {
				bw_tile_data_range + ((memory.read_vram(address as usize, false)as i8 as i16 +128 )as u16)*16
            };
            let tile_line = (y_offset%8 )as u16;

            let address = (tile + tile_line*2) as usize;
            
            let lsb = memory.read_byte(address);
            let msb = memory.read_byte(address+1);

            let i = 7-x_offset%8;

            let value = if i ==0 {
               // println!("I == 0");
               (lsb&1)|(msb&1<<1)
            } else {
                ((lsb&(1<<i))>>i) | ((msb&(1<<i))>>(i-1)) 
            };

            self.bg[line as usize][x as usize] = value<<1;

            self.screen[line as usize][x as usize] = PixelColorVal::color(value, memory.read_byte(BGP));

        }

    }


    pub fn draw_objects(&mut self, memory : &mut MemoryBus){
        self.lcdc.write(memory.read_byte(LCDC));
        for object in 0..40 {
            //Dans l'OAM chaque objet utilise 4 octets
            let offset = object*4;

            let x = memory.read_byte(OAM+offset+1).wrapping_sub(8);
            let y = memory.read_byte(OAM+offset).wrapping_sub(16);

            let tile_index = memory.read_byte(OAM+offset+2);
            let attributes = Attributes::new(memory.read_byte(OAM+offset+3));

            let line = memory.read_byte(LINE);

            let object_lenght = if self.lcdc.sprite_size {16} else {8};

            if y <=line &&line< y+object_lenght {

                let object_line = if attributes.y_flip{object_lenght+y - line -1}else {line - y};
                let address = VRAM_START + (tile_index as usize) *16 + (object_line as usize)*2;
                //println!("ADDRESS OBJ {:02X}", address);
                let lsb = memory.read_byte(address);
                let msb = memory.read_byte(address+1);

                for i in 0..8 {
                    //Correspondance des valeurs des bits et des couleurs
                    let value = if i ==0 {
                        (lsb&1)|(msb&1<<1)
                     } else {
                         ((lsb&(1<<i))>>i) | ((msb&(1<<i))>>(i-1)) 
                     };
                  
         
                    let pixel = if attributes.x_flip {
                        x.wrapping_add(i)
                    } else {
                        x.wrapping_add(7-i)
                    };

                    let line =line as usize;
                    let pixel = pixel as usize;

                    //println!("VALUE OBJ {}", value);
                    let palette = if attributes.dmg_pallette {OBP1}else {OBP0};
                    if pixel<160 && !self.bg_prio( line, pixel, attributes.priority){
                        self.screen[line][pixel] = PixelColorVal::color(value, memory.read_byte(palette));
                    }

                }

            }

        }

    }


    fn bg_prio(&self, line: usize, pixel: usize, priority: bool)-> bool{
        self.lcdc.bg_display_enable && (self.bg[line][pixel] & 1 !=0 || priority) && (self.bg[line][pixel]>1)

    }

    fn bg_prio_zero(&mut self, line: usize){
        for pixel in 0..160u8 {
            self.bg[line as usize][pixel as usize] = 0;
        }
    }


   /*  fn majAffichage(&mut self) {
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
 */
}

pub struct Attributes {

    pub priority : bool,
    pub y_flip: bool,
    pub x_flip: bool,
    pub dmg_pallette: bool,
}


impl Attributes {
    pub fn new(value: u8)-> Self {
        Attributes { 
            priority: (value&0x80>0), 
            y_flip: (value&0x40>0), 
            x_flip: (value&0x20>0), 
            dmg_pallette: (value&0x10>0),
        }

    }

}
pub struct LCDC {
    pub display_enable: bool,       // Bit 7
    pub window_tile_map: bool,     // Bit 6
    pub window_display_enable: bool, // Bit 5
    pub bg_and_window_tile_data: bool, // Bit 4
    pub bg_tile_map: bool,         // Bit 3
    pub sprite_size: bool,         // Bit 2
    pub sprite_display_enable: bool, // Bit 1
    pub bg_display_enable: bool,    // Bit 0
}

impl LCDC {
    pub fn new() -> Self{
        LCDC { 
            display_enable: false,
            window_tile_map: false,
            window_display_enable: false,
            bg_and_window_tile_data: false,
            bg_tile_map: false,
            sprite_size: false,
            sprite_display_enable: false,
            bg_display_enable: false,
        }
    }
    
    pub fn write(&mut self, value: u8) {
        //Bit 7 : Display Enable
        //Récupère le bit 7 de la valeur entrée, change la valeur de display_enable
        self.display_enable = (value & 0x80) != 0;
    
        //Bit 6 : Window Tile Map Display Select
        //Le bit 6 est extrait de la valeur entrée et mis dans window_tile_map
        self.window_tile_map = (value & 0x40) != 0;
    
        //Bit 5 : Window Display Enable
        //Récupère le bit 5 de la valeur entrée, change la valeur de window_display_enable
        self.window_display_enable = (value & 0x20) != 0;
    
        //Bit 4 : BG and Window Tile Data Select
        //Le bit 4 est extrait de la valeur entrée et mis dans bg_and_window_tile_data
        self.bg_and_window_tile_data = (value & 0x10) != 0;
    
        //Bit 3 : BG Tile Map Display Select
        //Le bit 3 est extrait de la valeur entrée et mis dans bg_tile_map
        self.bg_tile_map = (value & 0x08) != 0;
    
        //Bit 2 : Sprite Size
        //Le bit 2 est extrait de la valeur entrée et mis dans sprite_size
        self.sprite_size = (value & 0x04) != 0;
    
        //Bit 1 : Sprite Display Enable
        //Récupère le bit 1 de la valeur entrée, change la valeur de sprite_display_enable
        self.sprite_display_enable = (value & 0x02) != 0;
    
        //Bit 0 : BG Display Enable
        //Récupère le bit 0 de la valeur entrée, change la valeur de bg_display_enable
        self.bg_display_enable = (value & 0x01) != 0;
    }
    
}
