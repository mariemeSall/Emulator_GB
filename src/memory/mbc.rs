use std::{fs::File, io::Read};



pub trait MBC  {
    fn read_byte(&mut self, address: usize) ->u8;
    fn write_byte(&mut self, address: usize, value:u8);
    fn load_game(&mut self, game_file: &mut File)->usize;
    
}


pub struct MBC1 {
    pub rom : Vec<u8>,
    pub ram : Vec<u8>,
    pub rom_bank : u8,
    pub ram_bank : u8,
    pub use_ram : bool,
    pub mode : bool,

}

impl MBC for MBC1 {
    fn read_byte(&mut self, address: usize) ->u8 {

        match address {
            0x0000..=0x3FFF => {
                self.rom[address]
            },
            0x4000..=0x7FFF => {
                let new_address = ((self.rom_bank as usize) - 1)*0x4000 +address;
                self.rom[new_address]

            },
            0xA000..=0xBFFF => {
                if self.use_ram {
                    let new_address = (self.rom_bank as usize)*0x2000 + address%0x2000;
                    self.ram[new_address]
                } else {
                    0
                }
            },
            _=> panic!("Invalid memory to read")
        }

    }

    fn write_byte(&mut self, address:usize, value:u8){
        match address {
            0x0000 ..=0x1FFF =>{
                //ecrire dans cette partie active l'utilisation de la ram si le dernier octet est A
                self.use_ram = (value&0xF)==0xA;
            },  
            0x2000 ..=0x3FFF =>{
                //ecrire dans cette partie choisi une bonne valeur pour la rom bank
                let mut new_rom_bank = (self.rom_bank&0xE0)| (value & 0x1F);
                if new_rom_bank&0x1F==0 {
                    new_rom_bank =self.rom_bank+1;
                }
                self.rom_bank = new_rom_bank;

            },  
            0x4000 ..=0x5FFF =>{
                //ecrire dans cette partie, modifie les valeur des banl en fonction du mode

                let new_val = value&0x03;

                if self.mode {
                    //si le mode est 4/32, on change la valeur de la bank ram
                    self.ram_bank = new_val;

                } else {
                    //si le mode est 16/8, on set les deux bits significatif de la rom bank

                    self.rom_bank |= new_val<<5;
                }

            },  
            0x6000 ..=0x7FFF =>{
                //ecrire dans cette partie modifie le mode a utiliser
                self.mode = value%2==1;

                if self.mode {
                    self.rom_bank &= !0x60;
                } else {
                    self.ram_bank=0;
                }
            },  
            0xA000 ..=0xBFFF =>{
                //ecrire dans cette partie se fait si la ram est utilise et met la valeur dans l'adresse modifier
                if self.use_ram {
                    let new_address = (self.rom_bank as usize)*0x2000 + address%0x2000;
                    self.ram[new_address] = value;
                }

            },
            _=> panic!("Invalid memory to write")
            
        }
    }
    
    fn load_game(&mut self, game_file: &mut File) -> usize {
		game_file.read(&mut self.rom).unwrap()
	}
}

impl MBC1 {
    pub fn new()-> MBC1 {
        MBC1{
            rom: vec![0; 0x200000], 
            ram: vec![0; 0x008000], 
            rom_bank: 1, 
            ram_bank: 0, 
            use_ram: false,
            mode: false
        }
    }

}

pub struct ROM_ONLY {
    pub rom : [u8; 0x8000],
    pub ram : [u8; 0x2000],

}

impl ROM_ONLY {
    pub fn new() -> ROM_ONLY {
        ROM_ONLY { rom: [0; 0x8000], ram: [0;0x2000] }
    }
}

impl MBC for ROM_ONLY {
    
    fn load_game(&mut self, game_file: &mut File)->usize {
        game_file.read(&mut self.rom).unwrap()
    }

    fn read_byte(&mut self, address: usize) ->u8 {
        if address>=0xA000{
            self.ram[address - 0xA000]

        } else {
            self.rom[address] 
        } 
    }

    fn write_byte(&mut self, address: usize, value:u8) {
        if address>=0xA000{
            self.ram[address - 0xA000] = value;

        } else {
            self.rom[address] = value;
        }
    }

}