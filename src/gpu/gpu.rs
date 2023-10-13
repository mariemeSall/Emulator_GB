pub const VRAM_START: usize = 0x8000;   //première adresse consacrée aux tuiles
pub const VRAM_END: usize = 0x9FFF;     //dernière adresse consacrée aux tuiles
pub const VRAM_SIZE: usize = VRAM_END - VRAM_START + 1;     //taille utilisée par les données des tuiles
pub const LCDC_ADDR: usize = 0xFF40;    //addresse du lcdc
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
    pub vram: [u8; VRAM_SIZE],
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
        let tile_index = address / 16; //faut pas toucher a la valeur de address, on travail sur les values et c'est pas la qu'il faut diviser par 16
        //tile index devrait etre un compteur en dehors de write_vram car t'es en 1 octect par octect
        //Tous les deux octets correspond à une nouvelle ligne.
        let row_index = (address % 16) / 2; //deux octets c'est récupérer deux value de la vram
        //pareil ça devrait  etre un compteur

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

    pub fn write_vram2(&mut self, memory : [u8; 0xFFFF+1]){
        for i in 0..0x2000   {
            self.vram[i] = memory[i+VRAM_START];
        }
    }

    pub fn create_tile(&self) -> [[PixelColorVal; SCREEN_WIDTH as usize]; SCREEN_HEIGHT as usize] {
        let mut screen_data = [[PixelColorVal::Zero; SCREEN_WIDTH as usize]; SCREEN_HEIGHT as usize];
        let mut y = 0;
        let mut tile_x = 1;
        let mut tile_y= 1;
        let tile_by_width = SCREEN_WIDTH/8;
        let tile_by_height = SCREEN_HEIGHT/8;
        let mut i = 0;

        while tile_x<=tile_by_width&&tile_y<=tile_by_height{

            //A la fin d'une tile, y est egale a 8*tile_y
            if(8*tile_y)==y {
                //on incremente tile_x pour passer a la tile suivante
                tile_x+=1;
                y=8*(tile_y-1);
                //on verifie qu'on ne passe pas la taille de l'ecran
                if tile_x >tile_by_width {
                    //si on depasse l'ecran, on repasse tile_x a 1 et on increment tile_y
                    tile_x = 1;
                    tile_y += 1;
                }
            }


            let value1 = self.vram[i];
            let value2 = self.vram[i+1];

            for j in 0..8 {
                let value = ((value1>>j)&0x01)<<1|(value2>>j)&0x01;
                let value_color = match value {
                    0 => PixelColorVal::Zero ,
                    1 => PixelColorVal::One,
                    2 => PixelColorVal::Two,
                    3 => PixelColorVal::Three,
                    _ => panic!("pixel color val")
                };
              
                 
                screen_data[y as usize][((tile_x-1)*8+j) as usize] = value_color;
            }
            y+=1;

            i+=2;
        }
        screen_data
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
            _ => panic!("TODO: support others areas of the memory")
        }
    }

    pub fn write_byte(&mut self, address: u16, value : u8){
        let address = address as usize;
        match address {
            VRAM_START ..= VRAM_END => {
                self.gpu.write_vram(address - VRAM_START, value)
            },
            LCDC_ADDR => self.lcdc.write_byte(value),
            _ => panic!("TODO: support others areas of the memory")
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

        if self.display_enable {
            result |= 0x80;
        } else {
            result &= !0x80;
        }

        result |= (self.window_tile_map as u8) << 6;

        if self.window_display_enable {
            result |= 0x20;
        } else {
            result &= !0x20;
        }

        result |= (self.bg_and_window_tile_data as u8) << 4;
        result |= (self.bg_tile_map as u8) << 3;
        result |= (self.sprite_size as u8) << 2;

        if self.sprite_display_enable {
            result |= 0x02;
        } else {
            result &= !0x02;
        }

        if self.bg_display_enable {
            result |= 0x01;
        } else {
            result &= !0x01;
        }

        result
    }


    pub fn write_byte(&mut self, value: u8) {
        self.display_enable = (value & 0x80) != 0;
        self.window_tile_map = ((value >> 6) & 0x01) as usize;
        self.window_display_enable = (value & 0x20) != 0;
        self.bg_and_window_tile_data = ((value >> 4) & 0x01) as usize;
        self.bg_tile_map = ((value >> 3) & 0x01) as usize;
        self.sprite_size = ((value >> 2) & 0x01) as usize;
        self.sprite_display_enable = (value & 0x02) != 0;
        self.bg_display_enable = (value & 0x01) != 0;
    }
}
    