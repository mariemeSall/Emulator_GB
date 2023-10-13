use std::fs::File;

use super::mbc::{MBC1, MBC, ROM_ONLY};

pub trait MemoryUnit {
    const VRAM_START: usize ;
    const VRAM_END: usize ;
    const WRAM_START: usize;
    const WRAM_END: usize ;
    const ECHO_RAM_START: usize;
    const ECHO_RAM_END: usize ;
    const OAM_START: usize ;
    const OAM_END: usize ;
    const NU_START: usize ;
    const NU_END: usize ;
    const IO_REGISTER_START: usize ;
    const IO_REGISTER_END: usize ;
    const HRAM_START: usize ;
    const HRAM_END: usize;
    const IE: usize;
    const LCDC_ADDR: usize; 
}

pub struct MemoryBus {
    pub memory: [u8; 0x10000],
    pub mbc : ROM_ONLY,
    pub vram : [u8; 0x4000],
    pub wram : [u8; 0x8000],
    pub wram_bank: u8,
    pub bios : Vec<u8>,
    pub bios_run : bool,
    pub key_state : u8,
    pub background_palette : [u8; 64],
    pub sprite_palette : [u8; 64],
}

impl MemoryUnit for MemoryBus {
    const VRAM_START: usize = 0x8000 ;
    const VRAM_END: usize = 0x9FFF;
    const WRAM_START: usize = 0xC000;
    const WRAM_END: usize = 0xDFFF;
    const ECHO_RAM_START: usize = 0xE000;
    const ECHO_RAM_END: usize = 0xFDFF;
    const OAM_START: usize = 0xFE00;
    const OAM_END: usize = 0xFE9F;
    const NU_START: usize = 0xFEA0;
    const NU_END: usize = 0xFEFF;
    const IO_REGISTER_START: usize = 0xFF00;
    const IO_REGISTER_END: usize = 0xFF7F;
    const HRAM_START: usize  = 0xFF80;
    const HRAM_END: usize = 0xFFFE;
    const IE: usize = 0xFFFF;
    const LCDC_ADDR: usize = 0xFF40; 
}



impl MemoryBus {
    pub fn new()-> MemoryBus {

        MemoryBus { 
            memory: [0; 0x10000], 
            mbc : ROM_ONLY::new() ,
            vram : [0; 0x4000],
            wram : [0; 0x8000],
            wram_bank : 1,
            bios : Vec::<u8>::new(),
            bios_run : true,
            key_state : 0xFF,
            background_palette : [0; 64],
            sprite_palette : [0; 64],
        }

    }

    pub fn read_byte(&mut self, address: usize) -> u8 {
        let bios_len = self.bios.len();
        if address <bios_len {
            if address<0x0200 {
                self.mbc.read_byte(address)

            } else {
                if self.bios_run {
                    self.bios[address]
                } else {
                    self.mbc.read_byte(address)
                }
            }
        } else {
            match address {
                0x0..=0x7FFF|0xA000 ..=0xBFFF => {
                    self.mbc.read_byte(address)
                },
                0x8000..=0x9FFF => {
                    self.vram[address%0x8000]
                },
                 0xC000..=0xCFFF => {
                    self.wram[address - 0xC000]
                }, 
                0xE000..=0xEFFF => {
                    self.wram[self.wram_bank as usize*0x1000 + address%0xD000]
                },
                0xFF00 => {
                    match self.memory[0xFF00] & 0x30 {
                        0x10 => 0x10 | (self.key_state >> 4),
                        0x20 => 0x20 | (self.key_state & 0xF),
                        _ => 0
                    }
                },
                0xFF55 => {
                    if self.memory[0xFF55] == 0xFF {0xFF} else {self.memory[0xFF55] & 0x7F}
                },
                0xFF69 => {
                    self.background_palette[(self.read_byte(0xFF68) & 0x3F) as usize]
                },
                0xFF6B => {
                    self.sprite_palette[(self.read_byte(0xFF6A) & 0x3F) as usize]
                },
                _ => {
                    self.memory[address]
                },



            }
        }
       
    }

    pub fn write_byte(&mut self, address: usize, value: u8){
        match address {
            0x0000..=0x7FFF |0xA000..=0xBFFF=> {
                self.mbc.write_byte(address, value);

            },
            0x8000..=0x9FFF => {
                self.vram[address%0x8000] = value;
            },
            0xC000..=0xCFFF => {
                self.wram[address - 0xC000] = value;
                self.memory[address + 0x2000] = value;
            },
            0xD000..= 0xDFFF => {
                self.wram[self.wram_bank as usize*0x1000 + address%0xD000] = value;
                if address<0xDE00 {
                    self.memory[address + 0x2000] = value;
                }
            },
            0xEFFF..=0xFDFF => {
                self.memory[address - 0x2000] = value;
            },
            0xFF04 => { 
                self.memory[0xFF04] = 0
            },   
            0xFF44  => { 
                 self.memory[0xFF44] = 0
            },
            0xFF46  => { 
                let start = (value as u16) << 8;
                for i in 0..0xA0 {
                    let copy_val = self.read_byte((start + i)as usize);
                    self.write_byte((0xFE00 + i) as usize, copy_val);
                }
                return;
            },   
            0xFF4D  => { // Prepare speed switch
                let curr_speed = self.memory[0xFF4D] & 0x80;
                return self.memory[0xFF4D] = curr_speed | (value & 0x7F);
            },   
            0xFF4F  => { //VRAM bank
                return self.memory[0xFF4F] = value & 1;
            },          
            0xFF69  => { //Background Palette Data
                self.background_palette[(self.read_byte(0xFF68) & 0x3F) as usize] = value;
                if (self.read_byte(0xFF68) >> 7) > 0 {
                    let old_val = self.read_byte(0xFF68);
                    self.write_byte(0xFF68, (old_val + 1) | (1 << 7));
                }
            },   
            0xFF6B  => { //Sprite Palette Data 
                self.sprite_palette[(self.read_byte(0xFF6A) & 0x3F) as usize] = value;
                if (self.read_byte(0xFF6A) >> 7) > 0 {
                    let old_val = self.read_byte(0xFF6A);
                    self.write_byte(0xFF6A, (old_val + 1) | (1 << 7));
                }
            },
            0xFF70  => { //select wram bank
                self.wram_bank = if (value & 7) == 0 || true {1} else {value & 7};
            }
            _=> self.memory [address] = value, //a voir si c'est a mettre en dehors aussi
        }

    }

    pub fn load_data(&mut self, data: &mut File){
        self.mbc.load_game(data);
    }

    pub fn read_vram(&self, address: usize, bank: bool) -> u8 {
		self.vram[bank as usize*0x2000 + address %0x8000]
	}
}