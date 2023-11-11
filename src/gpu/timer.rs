pub struct Timer {
    div: u8,        //Registre DIV (Divider Register)
    tima: u8,       //Registre TIMA (Timer Counter)
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
            0xFF04 => self.div,
            0xFF05 => self.tima,
            0xFF06 => self.tma,
            0xFF07 => self.tac,
            _ => 0,
        }
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            0xFF04 => self.div = 0, //Écrire n'importe quelle valeur réinitialise DIV
            0xFF05 => self.tima = value,
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
                    self.tima = self.tma;
                }
            }
        }

        //Met à jour le registre DIV en fonction des cycles écoulés
        self.div = self.div.wrapping_add((delta_cycles / 256) as u8);
    }

    fn is_timer_enabled(&self) -> bool {
        (self.tac & 0x04) != 0
    }
}
