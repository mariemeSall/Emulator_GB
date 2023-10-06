use super::register::Resgisters;

pub struct CPU {
    resgiters: Resgisters,
    pc: u16,
    bus: MemoryBus,
}

pub struct MemoryBus {
    memory: [u8; 0xFFFF]

}

impl MemoryBus {
    pub fn read_byte(&self, address: u16) -> u8 {
      self.memory[address as usize]
    }
}

pub enum Instructions {
    ADD(TargetResgister),
}

impl Instructions {
    //Fonction pour retrouver l'instruction à faire en fonction du byte passé
    pub fn from_bytes(byte: u8) -> Option<Instructions> {
        match byte {

            //ADD bytes
            0x80 => Some(Instructions::ADD(TargetResgister::B)),
            0x81 => Some(Instructions::ADD(TargetResgister::C)),
            0x82 => Some(Instructions::ADD(TargetResgister::D)),
            0x83 => Some(Instructions::ADD(TargetResgister::E)),
            0x84 => Some(Instructions::ADD(TargetResgister::H)),
            0x85 => Some(Instructions::ADD(TargetResgister::L)),
            0x87 => Some(Instructions::ADD(TargetResgister::A)),
            _ => None,
        }
    }
}
pub enum TargetResgister{
    A,B,C,D,E,H,L,
}

impl CPU {

    pub fn execute(&mut self, instruction: Instructions) -> u16{
        match instruction {
            Instructions::ADD(target)=>{
                match target {
                    TargetResgister::A=> {
                        self.resgiters.a = self.add(self.resgiters.a); 
                        self.pc.wrapping_add(1)
                    }
                    TargetResgister::B=> {
                        let value = self.resgiters.b;
                        self.resgiters.a = self.add(value);
                        self.pc.wrapping_add(1)
                    }
                    TargetResgister::C=> {
                        let value = self.resgiters.c;
                        self.resgiters.a = self.add(value);
                        self.pc.wrapping_add(1)
                    }
                    TargetResgister::D=> {
                        let value = self.resgiters.d;
                        self.resgiters.a = self.add(value);
                        self.pc.wrapping_add(1)
                    }
                    TargetResgister::E=> {
                        let value = self.resgiters.e;
                        self.resgiters.a = self.add(value);                    
                        self.pc.wrapping_add(1)
                    }
                    TargetResgister::H=> {
                        let value = self.resgiters.h;
                        self.resgiters.a = self.add(value);                    
                        self.pc.wrapping_add(1)
                    }
                    TargetResgister::L=> {
                        let value = self.resgiters.l;
                        self.resgiters.a = self.add(value);
                        self.pc.wrapping_add(1)
                    }

                }
            }


        }
    }

    pub fn step(&mut self){
        //On récupère l'instruction à faire depuis le bus.
        let instruction_byte = self.bus.read_byte(self.pc);

        //On vérifie que l'insturction existe.
        let next_pc = if let Some(instruction) = Instructions::from_bytes(instruction_byte) {
            self.execute(instruction)
        } else {
            panic!("Pas d'instuction trouvée depuis l'adresse 0x{:x}", instruction_byte);
        };

        //On change le pointeur pour l'execution suivante.
        self.pc = next_pc;

    }  

    pub fn add(&mut self, value: u8) -> u8{
        //Ajoute value à a en vérifiant si overflow 
        let (new, overflow) = self.resgiters.a.overflowing_add(value);

        //Modifictaions des flags si nécessaire
        self.resgiters.f.zero = new==0;
        self.resgiters.f.carry = overflow;
        self.resgiters.f.subtract = false;

        //Pour trouver le half carry, on additionne les deux parties inférieurs 
        //et vérifie si le résultat est supérieur à 15 (dernière valeur en 4 bits)
        let lower_a = self.resgiters.a & 15;
        let lower_value = value & 15;
        let half_carry = lower_a & lower_value;
        self.resgiters.f.half_carry = half_carry>15 ;
        new 

    }

}

