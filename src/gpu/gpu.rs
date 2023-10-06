#![allow(unused_variables)]
fn main() {
    //première adresse consacrée aux tuiles
    pub const VRAM_START: usize = 0x8000;
    //dernière adresse consacrée aux tuiles
    pub const VRAM_END: usize = 0x9FFF;
    //taille utilisée par les données des tuiles
    pub const VRAM_SIZE: usize = VRAM_END - VRAM_START + 1;

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
        pub fn read_vram(){
            
        }
    }
}