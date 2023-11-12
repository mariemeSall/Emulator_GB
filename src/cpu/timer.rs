use crate::memory::memory::MemoryBus;

use super::cpu::CPU;

pub struct Timer {
    div: i16,        //Registre DIV (Divider Register)
    tima: i16,       //Registre TIMA (Timer Counter)
    tma: u8,        //Registre TMA (Timer Modulo)
    tac: u8,        //Registre TAC (Timer Control)
    last_cycle: u64, //Dernier cycle traité
}

impl Timer {
    pub fn new() -> Self {
        Timer {
            div: 0,
            tima: 0,
            tma: 0,
            tac: 0,
            last_cycle: 0,
        }
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        match address {
            0xFF04 => self.div as u8,
            0xFF05 => self.tima as u8,
            0xFF06 => self.tma,
            0xFF07 => self.tac,
            _ => 0,
        }
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            0xFF04 => self.div = 0, //Écrire n'importe quelle valeur réinitialise DIV
            0xFF05 => self.tima = value as i16,
            0xFF06 => self.tma = value,
            0xFF07 => self.tac = value,
            _ => {}
        }
    }

    pub fn update(&mut self, cycles: u64) {
        let delta_cycles = cycles - self.last_cycle;
        self.last_cycle = cycles;

        if self.is_timer_enabled() {
            let tac_bit = 2u8.pow(self.tac as u32 & 0x03);

            if delta_cycles >= u64::from(tac_bit) {
                self.tima = self.tima.wrapping_add(1);

                if self.tima == 0 {
                    self.tima = self.tma as i16;
                }
            }
        }

        //Met à jour le registre DIV en fonction des cycles écoulés
        self.div = self.div.wrapping_add((delta_cycles / 256) as i16);
    }

    pub fn step(&mut self, cycles: i16, memory: &mut MemoryBus, cpu: &mut CPU){
        self.div -= cycles;

        if self.div<=0 {
            self.div = 255;
            let div = memory.read_byte(0xFF04);
            memory.write_byte(0xFF04, div.wrapping_add(1));
        }

        let tac = memory.read_byte(0xFF07);

        if tac &4 >0 {
            self.tima -= cycles;
        }

        if self.tima <=0 {
            self.tima = match tac & 0x3 {
				0 => (41494304/4096) as i16,
				1 => (41494304/262144) as i16,
				2 => (41494304/65536) as i16,
				3 => (41494304/16384) as i16,
				_ => panic!("Invalid lower 2 bits for TAC")
			};

            let tima= memory.read_byte(0xFF05);

            if tima ==255 {
                let tma = memory.read_byte(0xFF06);
                memory.write_byte(0xFF06,tma);
                cpu.request(memory, 2);
            } else {
                memory.write_byte(0xFF05, tima + 1)
            }
        }
    }

    fn is_timer_enabled(&self) -> bool {
        (self.tac & 0x04) != 0
    }
}