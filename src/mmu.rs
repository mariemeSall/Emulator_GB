pub trait MemoryUnit {
    const VRAM_START: u16 ;
    const VRAM_END: u16 ;
    const WRAM_START: u16;
    const WRAM_END: u16 ;
    const ECHO_RAM_START: u16;
    const ECHO_RAM_END: u16 ;
    const OAM_START: u16 ;
    const OAM_END: u16 ;
    const NU_START: u16 ;
    const NU_END: u16 ;
    const IO_REGISTER_START: u16 ;
    const IO_REGISTER_END: u16 ;
    const HRAM_START: u16 ;
    const HRAM_END: u16;
    const IE: u16;
}

pub struct MemoryBus {
    pub memory: [u8; 0xFFFF],
}

impl MemoryUnit for MemoryBus {
    const VRAM_START: u16 = 0x8000 ;
    const VRAM_END: u16 = 0x9FFF;
    const WRAM_START: u16 = 0xC000;
    const WRAM_END: u16 = 0xDFFF;
    const ECHO_RAM_START: u16 = 0xE000;
    const ECHO_RAM_END: u16 = 0xFDFF;
    const OAM_START: u16 = 0xFE00;
    const OAM_END: u16 = 0xFE9F;
    const NU_START: u16 = 0xFEA0;
    const NU_END: u16 = 0xFEFF;
    const IO_REGISTER_START: u16 = 0xFF00;
    const IO_REGISTER_END: u16 = 0xFF7F;
    const HRAM_START: u16  = 0xFF80;
    const HRAM_END: u16 = 0xFFFE;
    const IE: u16 = 0xFFFF;
}


impl MemoryBus {
    pub fn read_byte(&self, address: u16) -> u8 {
      self.memory[address as usize]
    }

    pub fn write_byte(&mut self, address: u16, source: u8){
        self.memory[address as usize] = source;

    }
}
