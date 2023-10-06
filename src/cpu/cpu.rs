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
    JP(JumpTest),
    JR(),
    JPI(),
}

impl Instructions {
    pub fn from_bytes(byte: u8, prefixed: bool) -> Option<Instructions>{
        if prefixed {
            Instructions::from_bytes_prefixed(byte)
        } else  {
            Instructions::from_bytes_not_prefixed(byte)
        }

    }

    //Fonction pour retrouver l'instruction à faire en fonction du byte passé
    pub fn from_bytes_not_prefixed(byte: u8) -> Option<Instructions> {
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

    //Fonction pour retourver l'instruction avec prefixe
    pub fn from_bytes_prefixed(byte: u8) -> Option<Instructions>{
        match byte {
            _ => None,
        }
    }
}

pub enum TargetResgister{
    A,B,C,D,E,H,L,
}

pub enum JumpTest {
    NotZero,
    Zero,
    NotCarry,
    Carry,
    Always
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
            Instructions::JP(jump) => {
                let jump_condition = match jump {
                    JumpTest::Always => true,
                    JumpTest::Carry => self.resgiters.f.carry,
                    JumpTest::NotCarry => !self.resgiters.f.carry,
                    JumpTest::Zero => self.resgiters.f.zero,
                    JumpTest::NotZero => !self.resgiters.f.zero
                };
                self.jump(jump_condition)
            }
            _=> self.pc,
        }
    }

    pub fn step(&mut self){
        //On récupère l'instruction à faire depuis le bus.
        let mut instruction_byte = self.bus.read_byte(self.pc);
        //On vérifie si l'instruction est un préfixe.
        let prefixed = instruction_byte== 0xCB;

        //S'il y a un préfix, l'instruction passe à celle suivante dans le bus
        if prefixed {
            instruction_byte = self.bus.read_byte(self.pc + 1);
        }

        //On vérifie que l'insturction existe.
        let next_pc = if let Some(instruction) = Instructions::from_bytes(instruction_byte, prefixed) {
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

    pub fn jump(&self, condition: bool)-> u16{
        if condition {
            let lower = self.bus.read_byte(self.pc + 1);
            let higher = self.bus.read_byte(self.pc +2);
            lower as u16 | (higher as u16)<<8
        } else {
            self.pc.wrapping_add(3)
        }
    }

}

