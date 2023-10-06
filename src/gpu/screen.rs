//première adresse consacrée aux tuiles
const VRAM_START: usize = 0x8000;
//dernière adresse consacrée aux tuiles
const VRAM_END: usize = 0x9FFF;
//taille utilisée par les données des tuiles
const VRAM_SIZE: usize = VRAM_END - VRAM_START + 1;

#[derive(Copy, Clone)]
enum PixelColorVal{
    Zero,
    One,
    Two,
    Three,
}

//tableau de 8x8 pour une tuile
type Tile = [[TilePixelValue; 8]; 8 ];

//Met les valeurs des pixels à zero : noir
fn black_tile() -> Tile{
    [[TilePixelValue::Zero; 8]; 8]
}

struct GPU{
    //video ram
    vram: [u8; VRAM_SIZE],
    tile_set: [Tile; 384],
}

impl GPU{
    fn new() -> GPU {

    }
}