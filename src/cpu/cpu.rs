use super::register::Resgisters;

pub struct CPU {
    resgiters: Resgisters,
    pc: u16,
    sp: u16,
    bus: MemoryBus,
    is_halted: bool,
    ime: bool,
}

pub struct MemoryBus {
    memory: [u8; 0xFFFF]

}

impl MemoryBus {
    pub fn read_byte(&self, address: u16) -> u8 {
      self.memory[address as usize]
    }

    pub fn write_byte(&mut self, address: u16, source: u8){
        self.memory[address as usize] = source;

    }
}

pub enum LoadByteTarget {
    A, B, C, D, E, H, L, HLI,BC,DE,HLD,HL
}
pub enum LoadByteSource {
    A, B, C, D, E, H, L, D8,HLI,BC,DE,HLD,HL
}

pub enum LoadWordTarget {
    BC, DE, HL, SP, a16

}
pub enum LoadWordSource {
    SP, d16,
}

pub enum LoadType {
  Byte(LoadByteTarget, LoadByteSource),
  Word(LoadWordTarget, LoadWordSource),
}

pub enum StackTarget {
    BC, DE, HL, AF
}
pub enum Target8{
    A,B,C,D,E,H,L,HL, d8
}
pub enum Target16{
    BC, DE, HL, SP
}
pub enum TargetType {
    A(Target8), HL(Target16)
}

pub enum JumpTest {
    NotZero,
    Zero,
    NotCarry,
    Carry,
    Always
}

pub enum Instructions {
    ADD(TargetType),
    JP(JumpTest),
    LD(LoadType),
    PUSH(StackTarget),
    POP(StackTarget),
    CALL(JumpTest),
    RET(JumpTest),
    NOP(),
    HALT(),
    SUB(Target8),
    ADC(Target8),
    SBC(Target8),
    CP(Target8),
    AND(Target8),
    OR (Target8),
    XOR(Target8),
    INC(TargetType), 
    DEC(TargetType), 
    CCF(), 
    CPL(), 
    SCF(), 
    DI(),

    //TODO
    JR(),
    JPI(),
    STOP(),
    RRA(), 
    RLA(), 
    RRCA(),
    RLCA(),
    SRA(),
    SLA(),
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
            0x00 => Some(Instructions::NOP()),
            0x01 => Some(Instructions::LD(LoadType::Word(LoadWordTarget::BC, LoadWordSource::d16))),
            0x02 => Some(Instructions::LD(LoadType::Byte(LoadByteTarget::BC, LoadByteSource::A))),
            0x03 => Some(Instructions::INC(TargetType::HL(Target16::BC))),
            0x04 => Some(Instructions::INC(TargetType::A(Target8::B))),
            0x05 => Some(Instructions::DEC(TargetType::A(Target8::B))),
            0x06 => Some(Instructions::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::D8))),
            0x07 => Some(Instructions::RLCA()),
            0x08 => Some(Instructions::LD(LoadType::Word(LoadWordTarget::a16, LoadWordSource::SP))),
            0x09 => Some(Instructions::ADD(TargetType::HL(Target16::BC))),
            0x0A => Some(Instructions::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::BC))),
            0x0B => Some(Instructions::DEC(TargetType::HL(Target16::BC))),
            0x0C => Some(Instructions::INC(TargetType::A(Target8::C))),
            0x0D => Some(Instructions::DEC(TargetType::A(Target8::C))),
            0x0E => Some(Instructions::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::D8))),
            0x0F => Some(Instructions::RRCA()),
            0x10 => Some(Instructions::STOP()),
            0x11 => Some(Instructions::LD(LoadType::Word(LoadWordTarget::DE, LoadWordSource::d16))),
            0x12 => Some(Instructions::LD(LoadType::Byte(LoadByteTarget::DE, LoadByteSource::A))),
            0x13 => Some(Instructions::INC(TargetResgister::DE)),
            0x14 => Some(Instructions::INC(TargetResgister::D)),
            0x15 => Some(Instructions::DEC(TargetResgister::D)),
            0x16 => Some(Instructions::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::D8))),
            0x17 => Some(Instructions::RLA()),
            0x18 => Some(Instructions::JR()),
            0x19 => Some(Instructions::ADD(AddResgister::HL, TargetResgister::DE)),
            0x1A => Some(Instructions::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::DE))),
            0x1B => Some(Instructions::DEC(TargetResgister::DE)),
            0x1C => Some(Instructions::INC(TargetResgister::E)),
            0x1D => Some(Instructions::DEC(TargetResgister::E)),
            0x1E => Some(Instructions::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::D8))),
            0x1F => Some(Instructions::RRA()),
            0x20 => Some(Instructions::JR()),
            0x21 => Some(Instructions::LD(LoadType::Word(LoadWordTarget::HL, LoadWordSource::d16))),
            0x22 => Some(Instructions::LD(LoadType::Byte(LoadByteTarget::HLI, LoadByteSource::A))),
            0x23 => Some(Instructions::INC(TargetResgister::HL)),
            0x24 => Some(Instructions::INC(TargetResgister::H)),
            0x25 => Some(Instructions::DEC(TargetResgister::H)),
            0x26 => Some(Instructions::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::D8))),
            0x27 => Some(Instructions::DAA()),
            0x28 => Some(Instructions::JR()),
            0x29 => Some(Instructions::ADD(AddResgister::HL, TargetResgister::HL)),
            0x2A => Some(Instructions::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::HLI))),
            0x2B => Some(Instructions::DEC(TargetResgister::HL)),
            0x2C => Some(Instructions::INC(TargetResgister::L)),
            0x2D => Some(Instructions::DEC(TargetResgister::L)),
            0x2E => Some(Instructions::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::D8))),
            0x2F => Some(Instructions::CPL()),
            0x30 => Some(Instructions::JR()),
            0x31 => Some(Instructions::LD(LoadType::Word(LoadWordTarget::SP, LoadWordSource::d16))),
            0x32 => Some(Instructions::LD(LoadType::Byte(LoadByteTarget::HLD, LoadByteSource::A))),
            0x33 => Some(Instructions::INC(TargetResgister::SP)),
            0x34 => Some(Instructions::INC(TargetResgister::HL)),
            0x35 => Some(Instructions::DEC(TargetResgister::HL)),
            0x36 => Some(Instructions::LD(LoadType::Byte(LoadByteTarget::HL, LoadByteSource::D8))),
            0x37 => Some(Instructions::SCF()),
            0x38 => Some(Instructions::JR()),
            0x39 => Some(Instructions::ADD(AddResgister::HL, TargetResgister::SP)),
            0x3A => Some(Instructions::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::HLD))),
            0x3B => Some(Instructions::DEC(TargetResgister::SP)),
            0x3C => Some(Instructions::INC(TargetResgister::A)),
            0x3D => Some(Instructions::DEC(TargetResgister::A)),
            0x3E => Some(Instructions::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::D8))),
            0x3F => Some(Instructions::CCF()),
            0x40 => Some(Instructions::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::B))),
            0x41 => Some(Instructions::LD(LoadType::Byte((LoadByteTarget::B), LoadByteSource::C))),
            0x42 => Some(Instructions::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::D))),
            0x43 => Some(Instructions::LD(LoadType::Byte(LoadByteTarget::B, (LoadByteSource::E)))),
            0x44 => Some(Instructions::LD(LoadType::Byte(LoadByteTarget::B, (LoadByteSource::H)))),
            0x45 => Some(Instructions::LD(LoadType::Byte((LoadByteTarget::B), (LoadByteSource::L)))),
            0x46 => Some(Instructions::LD(LoadType::Byte((LoadByteTarget::B), (LoadByteSource::HLI)))),
            0x47 => Some(Instructions::LD(LoadType::Byte((LoadByteTarget::B), (LoadByteSource::A)))),
            0x48 => Some(Instructions::LD(LoadType::Byte((LoadByteTarget::C), (LoadByteSource::B)))),
            0x49 => Some(Instructions::LD(LoadType::Byte((LoadByteTarget::C), (LoadByteSource::C)))),
            0x4A => Some(Instructions::LD(LoadType::Byte((LoadByteTarget::C), (LoadByteSource::D)))),
            0x4B => Some(Instructions::LD(LoadType::Byte(LoadByteTarget::C, (LoadByteSource::E)))),
            0x4C => Some(Instructions::LD(LoadType::Byte(LoadByteTarget::C, (LoadByteSource::H)))),
            0x4D => Some(Instructions::LD(LoadType::Byte(LoadByteTarget::C, (LoadByteSource::L)))),
            0x4E => Some(Instructions::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::HLI))),
            0x4F => Some(Instructions::LD(LoadType::Byte((LoadByteTarget::C), (LoadByteSource::A)))),
            0x50 => Some(Instructions::LD(LoadType::Byte((LoadByteTarget::D), (LoadByteSource::B)))),
            0x51 => Some(Instructions::LD(LoadType::Byte((LoadByteTarget::D), (LoadByteSource::C)))),
            0x52 => Some(Instructions::LD(LoadType::Byte((LoadByteTarget::D), (LoadByteSource::D)))),
            0x53 => Some(Instructions::LD(LoadType::Byte((LoadByteTarget::D), (LoadByteSource::E)))),
            0x54 => Some(Instructions::LD(LoadType::Byte((LoadByteTarget::D), (LoadByteSource::H)))),
            0x55 => Some(Instructions::LD(LoadType::Byte((LoadByteTarget::D), (LoadByteSource::L)))),
            0x56 => Some(Instructions::LD(LoadType::Byte((LoadByteTarget::D), (LoadByteSource::HLI)))),
            0x57 => Some(Instructions::LD(LoadType::Byte((LoadByteTarget::D), (LoadByteSource::A)))),
            0x58 => Some(Instructions::LD(LoadType::Byte((LoadByteTarget::E), (LoadByteSource::B)))),
            0x59 => Some(Instructions::LD(LoadType::Byte((LoadByteTarget::E), (LoadByteSource::C)))),
            0x5A => Some(Instructions::LD(LoadType::Byte((LoadByteTarget::E), (LoadByteSource::D)))),
            0x5B => Some(Instructions::LD(LoadType::Byte((LoadByteTarget::E), (LoadByteSource::E)))),
            0x5C => Some(Instructions::LD(LoadType::Byte((LoadByteTarget::E), (LoadByteSource::H)))),
            0x5D => Some(Instructions::LD(LoadType::Byte((LoadByteTarget::E), (LoadByteSource::L)))),
            0x5E => Some(Instructions::LD(LoadType::Byte((LoadByteTarget::E), (LoadByteSource::HLI)))),
            0x5F => Some(Instructions::LD(LoadType::Byte((LoadByteTarget::E), (LoadByteSource::A)))),
            0x60 => Some(Instructions::LD(LoadType::Byte((LoadByteTarget::H), (LoadByteSource::B)))),
            0x61 => Some(Instructions::LD(LoadType::Byte((LoadByteTarget::H), (LoadByteSource::C)))),
            0x62 => Some(Instructions::LD(LoadType::Byte((LoadByteTarget::H), (LoadByteSource::D)))),
            0x63 => Some(Instructions::LD(LoadType::Byte((LoadByteTarget::H), (LoadByteSource::E)))),
            0x64 => Some(Instructions::LD(LoadType::Byte((LoadByteTarget::H), (LoadByteSource::H)))),
            0x65 => Some(Instructions::LD(LoadType::Byte((LoadByteTarget::H), (LoadByteSource::L)))),
            0x66 => Some(Instructions::LD(LoadType::Byte((LoadByteTarget::H), (LoadByteSource::HLI)))),
            0x67 => Some(Instructions::LD(LoadType::Byte((LoadByteTarget::H), (LoadByteSource::A)))),
            0x68 => Some(Instructions::LD(LoadType::Byte((LoadByteTarget::L), (LoadByteSource::B)))),
            0x69 => Some(Instructions::LD(LoadType::Byte((LoadByteTarget::L), (LoadByteSource::C)))),
            0x6A => Some(Instructions::LD(LoadType::Byte((LoadByteTarget::L), (LoadByteSource::D)))),
            0x6B => Some(Instructions::LD(LoadType::Byte((LoadByteTarget::L), (LoadByteSource::E)))),
            0x6C => Some(Instructions::LD(LoadType::Byte((LoadByteTarget::L), (LoadByteSource::H)))),
            0x6D => Some(Instructions::LD(LoadType::Byte((LoadByteTarget::L), (LoadByteSource::L)))),
            0x6E => Some(Instructions::LD(LoadType::Byte((LoadByteTarget::L), (LoadByteSource::HLI)))),
            0x6F => Some(Instructions::LD(LoadType::Byte((LoadByteTarget::L), (LoadByteSource::A)))),
            0x70 => Some(Instructions::LD(LoadType::Byte((LoadByteTarget::HLI), (LoadByteSource::B)))),
            0x71 => Some(Instructions::LD(LoadType::Byte((LoadByteTarget::HLI), (LoadByteSource::C)))),
            0x72 => Some(Instructions::LD(LoadType::Byte((LoadByteTarget::HLI), (LoadByteSource::D)))),
            0x73 => Some(Instructions::LD(LoadType::Byte((LoadByteTarget::HLI), (LoadByteSource::E)))),
            0x74 => Some(Instructions::LD(LoadType::Byte((LoadByteTarget::HLI), (LoadByteSource::H)))),
            0x75 => Some(Instructions::LD(LoadType::Byte((LoadByteTarget::HLI), (LoadByteSource::L)))),
            0x76 => Some(Instructions::HALT()),
            0x77 => Some(Instructions::LD(LoadType::Byte((LoadByteTarget::HLI), (LoadByteSource::A)))),
            0x78 => Some(Instructions::LD(LoadType::Byte((LoadByteTarget::A), (LoadByteSource::B)))),
            0x79 => Some(Instructions::LD(LoadType::Byte((LoadByteTarget::A), (LoadByteSource::C)))),
            0x7A => Some(Instructions::LD(LoadType::Byte((LoadByteTarget::A), (LoadByteSource::D)))),
            0x7B => Some(Instructions::LD(LoadType::Byte((LoadByteTarget::A), (LoadByteSource::E)))),
            0x7C => Some(Instructions::LD(LoadType::Byte((LoadByteTarget::A), (LoadByteSource::H)))),
            0x7D => Some(Instructions::LD(LoadType::Byte((LoadByteTarget::A), (LoadByteSource::L)))),
            0x7E => Some(Instructions::LD(LoadType::Byte((LoadByteTarget::A), (LoadByteSource::HLI)))),
            0x7F => Some(Instructions::LD(LoadType::Byte((LoadByteTarget::A), (LoadByteSource::A)))),
            0x80 => Some(Instructions::ADD(AddResgister::A,TargetResgister::B)),
            0x81 => Some(Instructions::ADD(AddResgister::A,TargetResgister::C)),
            0x82 => Some(Instructions::ADD(AddResgister::A,TargetResgister::D)),
            0x83 => Some(Instructions::ADD(AddResgister::A,TargetResgister::E)),
            0x84 => Some(Instructions::ADD(AddResgister::A,TargetResgister::H)),
            0x85 => Some(Instructions::ADD(AddResgister::A,TargetResgister::L)),
            0x86 => Some(Instructions::ADD(AddResgister::A,TargetResgister::HL)),
            0x87 => Some(Instructions::ADD(AddResgister::A,TargetResgister::A)),
            0x88 => Some(Instructions::ADC(Target8::B)),
            0x89 => Some(Instructions::ADC(TargetResgister::C)),
            0x8A => Some(Instructions::ADC(TargetResgister::D)),
            0x8B => Some(Instructions::ADC(TargetResgister::E)),
            0x8C => Some(Instructions::ADC(TargetResgister::H)),
            0x8D => Some(Instructions::ADC(TargetResgister::L)),
            0x8E => Some(Instructions::ADC(TargetResgister::HL)),
            0x8F => Some(Instructions::ADC(TargetResgister::A)),
            0x90 => Some(Instructions::SUB(TargetResgister::B)),
            0x91 => Some(Instructions::SUB(TargetResgister::C)),
            0x92 => Some(Instructions::SUB(TargetResgister::D)),
            0x93 => Some(Instructions::SUB(TargetResgister::E)),
            0x94 => Some(Instructions::SUB(TargetResgister::H)),
            0x95 => Some(Instructions::SUB(TargetResgister::L)),
            0x96 => Some(Instructions::SUB(TargetResgister::HL)),
            0x97 => Some(Instructions::SUB(TargetResgister::A)),
            0x98 => Some(Instructions::SBC(TargetResgister::B)),
            0x99 => Some(Instructions::SBC(TargetResgister::C)),
            0x9A => Some(Instructions::SBC(TargetResgister::D)),
            0x9B => Some(Instructions::SBC(TargetResgister::E)),
            0x9C => Some(Instructions::SBC(TargetResgister::H)),
            0x9D => Some(Instructions::SBC(TargetResgister::L)),
            0x9E => Some(Instructions::SBC(TargetResgister::HL)),
            0x9F => Some(Instructions::SBC(TargetResgister::A)),
            0xA0 => Some(Instructions::AND(TargetResgister::B)),
            0xA1 => Some(Instructions::AND(TargetResgister::C)),
            0xA2 => Some(Instructions::AND(TargetResgister::D)),
            0xA3 => Some(Instructions::AND(TargetResgister::E)),
            0xA4 => Some(Instructions::AND(TargetResgister::H)),
            0xA5 => Some(Instructions::AND(TargetResgister::L)),
            0xA6 => Some(Instructions::AND(TargetResgister::HL)),
            0xA7 => Some(Instructions::AND(TargetResgister::A)),
            0xA8 => Some(Instructions::XOR(TargetResgister::B)),
            0xA9 => Some(Instructions::XOR(TargetResgister::C)),
            0xAA => Some(Instructions::XOR(TargetResgister::D)),
            0xAB => Some(Instructions::XOR(TargetResgister::E)),
            0xAC => Some(Instructions::XOR(TargetResgister::H)),
            0xAD => Some(Instructions::XOR(TargetResgister::L)),
            0xAE => Some(Instructions::XOR(TargetResgister::HL)),
            0xAF => Some(Instructions::XOR(TargetResgister::A)),
            0x80 => Some(Instructions::OR(TargetResgister::B)),
            0x81 => Some(Instructions::OR(TargetResgister::C)),
            0x82 => Some(Instructions::OR(TargetResgister::D)),
            0x83 => Some(Instructions::OR(TargetResgister::E)),
            0x84 => Some(Instructions::OR(TargetResgister::H)),
            0x85 => Some(Instructions::OR(TargetResgister::L)),
            0x86 => Some(Instructions::OR(TargetResgister::HL)),
            0x87 => Some(Instructions::OR(TargetResgister::A)),
            0x88 => Some(Instructions::CP(TargetResgister::B)),
            0x89 => Some(Instructions::CP(TargetResgister::C)),
            0x8A => Some(Instructions::CP(TargetResgister::D)),
            0x8B => Some(Instructions::CP(TargetResgister::E)),
            0x8C => Some(Instructions::CP(TargetResgister::H)),
            0x8D => Some(Instructions::CP(TargetResgister::L)),
            0x8E => Some(Instructions::CP(TargetResgister::HL)),
            0x8F => Some(Instructions::CP(TargetResgister::A)),
            0xF3 => Some(Instructions::DI()),
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


impl CPU {

    pub fn execute(&mut self, instruction: Instructions) -> u16{
        if !self.is_halted{
            match instruction {
                Instructions::ADD(add,target)=>{
                    match add {
                        AddResgister::A =>{
                            match target {
                                TargetResgister::A=> {
                                    self.resgiters.a = self.add(self.resgiters.a); 
                                    self.pc.wrapping_add(1)
                                },
                                TargetResgister::B=> {
                                    let value = self.resgiters.b;
                                    self.resgiters.a = self.add(value);
                                    self.pc.wrapping_add(1)
                                },
                                TargetResgister::C=> {
                                    let value = self.resgiters.c;
                                    self.resgiters.a = self.add(value);
                                    self.pc.wrapping_add(1)
                                },
                                TargetResgister::D=> {
                                    let value = self.resgiters.d;
                                    self.resgiters.a = self.add(value);
                                    self.pc.wrapping_add(1)
                                },
                                TargetResgister::E=> {
                                    let value = self.resgiters.e;
                                    self.resgiters.a = self.add(value);                    
                                    self.pc.wrapping_add(1)
                                },
                                TargetResgister::H=> {
                                    let value = self.resgiters.h;
                                    self.resgiters.a = self.add(value);                    
                                    self.pc.wrapping_add(1)
                                },
                                TargetResgister::L=> {
                                    let value = self.resgiters.l;
                                    self.resgiters.a = self.add(value);
                                    self.pc.wrapping_add(1)
                                },
                                TargetResgister::HL => {
                                    let value = self.bus.read_byte(self.resgiters.get_hl());
                                    self.resgiters.a = self.add(value);
                                    self.pc.wrapping_add(1)
                                },
                                TargetResgister::d8 => {
                                    let value = self.bus.read_byte(self.pc + 1);
                                    self.resgiters.a = self.add(value);
                                    self.pc.wrapping_add(2)
                                }
                                _=> self.pc
                            }
                        },
                        AddResgister::HL => {

                        }
                    }
                },
                Instructions::ADC(target) => {
                    match target {
                        TargetResgister::A=> {
                            self.resgiters.a = self.adc(self.resgiters.a); 
                            self.pc.wrapping_add(1)
                        },
                        TargetResgister::B=> {
                            let value = self.resgiters.b;
                            self.resgiters.a = self.adc(value);
                            self.pc.wrapping_add(1)
                        },
                        TargetResgister::C=> {
                            let value = self.resgiters.c;
                            self.resgiters.a = self.adc(value);
                            self.pc.wrapping_add(1)
                        },
                        TargetResgister::D=> {
                            let value = self.resgiters.d;
                            self.resgiters.a = self.adc(value);
                            self.pc.wrapping_add(1)
                        },
                        TargetResgister::E=> {
                            let value = self.resgiters.e;
                            self.resgiters.a = self.adc(value);                    
                            self.pc.wrapping_add(1)
                        },
                        TargetResgister::H=> {
                            let value = self.resgiters.h;
                            self.resgiters.a = self.adc(value);                    
                            self.pc.wrapping_add(1)
                        },
                        TargetResgister::L=> {
                            let value = self.resgiters.l;
                            self.resgiters.a = self.adc(value);
                            self.pc.wrapping_add(1)
                        },
                        TargetResgister::HL => {
                            let value = self.bus.read_byte(self.resgiters.get_hl());
                            self.resgiters.a = self.adc(value);
                            self.pc.wrapping_add(1)
                        },
                        TargetResgister::d8 => {
                            let value = self.bus.read_byte(self.pc + 1);
                            self.resgiters.a = self.add(value);
                            self.pc.wrapping_add(2)
                        }
                        _=> self.pc
                    }
                },
                Instructions::SUB(target)=> {
                    match target {
                        TargetResgister::A => {
                            self.resgiters.a = self.sub(self.resgiters.a);
                            self.pc.wrapping_add(1)
                        }
                        TargetResgister::B => {
                            self.resgiters.a = self.sub(self.resgiters.b);
                            self.pc.wrapping_add(1)
                        }
                        TargetResgister::C => {
                            self.resgiters.a = self.sub(self.resgiters.c);
                            self.pc.wrapping_add(1)
                        }
                        TargetResgister::D => {
                            self.resgiters.a = self.sub(self.resgiters.d);
                            self.pc.wrapping_add(1)
                        }
                        TargetResgister::E => {
                            self.resgiters.a = self.sub(self.resgiters.e);
                            self.pc.wrapping_add(1)
                        }
                        TargetResgister::H => {
                            self.resgiters.a = self.sub(self.resgiters.h);
                            self.pc.wrapping_add(1)
                        }
                        TargetResgister::L => {
                            self.resgiters.a = self.sub(self.resgiters.l);
                            self.pc.wrapping_add(1)
                        }
                        TargetResgister::HL => {
                            self.resgiters.a = self.sub(self.bus.read_byte(self.resgiters.get_hl()));
                            self.pc.wrapping_add(1)
                        }
                        TargetResgister::d8 => {
                            self.resgiters.a = self.sub(self.bus.read_byte(self.pc + 1));
                            self.pc.wrapping_add(2)
                        }
                        _=> self.pc
                    }
                },
                Instructions::SBC(target)=> {
                    match target {
                        TargetResgister::A => {
                            self.resgiters.a = self.sbc(self.resgiters.a);
                            self.pc.wrapping_add(1)
                        }
                        TargetResgister::B => {
                            self.resgiters.a = self.sbc(self.resgiters.b);
                            self.pc.wrapping_add(1)
                        }
                        TargetResgister::C => {
                            self.resgiters.a = self.sbc(self.resgiters.c);
                            self.pc.wrapping_add(1)
                        }
                        TargetResgister::D => {
                            self.resgiters.a = self.sbc(self.resgiters.d);
                            self.pc.wrapping_add(1)
                        }
                        TargetResgister::E => {
                            self.resgiters.a = self.sbc(self.resgiters.e);
                            self.pc.wrapping_add(1)
                        }
                        TargetResgister::H => {
                            self.resgiters.a = self.sbc(self.resgiters.h);
                            self.pc.wrapping_add(1)
                        }
                        TargetResgister::L => {
                            self.resgiters.a = self.sbc(self.resgiters.l);
                            self.pc.wrapping_add(1)
                        }
                        TargetResgister::HL => {
                            self.resgiters.a = self.sbc(self.bus.read_byte(self.resgiters.get_hl()));
                            self.pc.wrapping_add(1)
                        }
                        TargetResgister::d8 => {
                            self.resgiters.a = self.sbc(self.bus.read_byte(self.pc + 1));
                            self.pc.wrapping_add(2)
                        }
                        _=> self.pc
                    }
                },
                Instructions::CP(target)=> {
                    match target {
                        TargetResgister::A => {
                            self.sub(self.resgiters.a);
                            self.pc.wrapping_add(1)
                        }
                        TargetResgister::B => {
                            self.sub(self.resgiters.b);
                            self.pc.wrapping_add(1)
                        }
                        TargetResgister::C => {
                            self.sub(self.resgiters.c);
                            self.pc.wrapping_add(1)
                        }
                        TargetResgister::D => {
                            self.sub(self.resgiters.d);
                            self.pc.wrapping_add(1)
                        }
                        TargetResgister::E => {
                            self.sub(self.resgiters.e);
                            self.pc.wrapping_add(1)
                        }
                        TargetResgister::H => {
                            self.sub(self.resgiters.h);
                            self.pc.wrapping_add(1)
                        }
                        TargetResgister::L => {
                            self.sub(self.resgiters.l);
                            self.pc.wrapping_add(1)
                        }
                        TargetResgister::HL => {
                            self.sub(self.bus.read_byte(self.resgiters.get_hl()));
                            self.pc.wrapping_add(1)
                        }
                        TargetResgister::d8 => {
                           self.sub(self.bus.read_byte(self.pc + 1));
                           self.pc.wrapping_add(2)

                        }
                        _=> self.pc
                    }
                },
                Instructions::INC(target)=> {
                    match target {
                        TargetResgister::A => {
                            let (new, overflow) = self.resgiters.a.overflowing_add(1);
                            self.resgiters.f.zero = new==0;
                            self.resgiters.f.subtract = false;
                            self.resgiters.f.half_carry = (new >> 4) != (self.resgiters.a >>4);
                            self.resgiters.a = new
                        },
                        TargetResgister::B => {
                            let (new, overflow) = self.resgiters.b.overflowing_add(1);
                            self.resgiters.f.zero = new==0;
                            self.resgiters.f.subtract = false;
                            self.resgiters.f.half_carry = (new >> 4) != (self.resgiters.b >>4);
                            self.resgiters.b = new
                        },
                        TargetResgister::C => {
                            let (new, overflow) = self.resgiters.c.overflowing_add(1);
                            self.resgiters.f.zero = new==0;
                            self.resgiters.f.subtract = false;
                            self.resgiters.f.half_carry = (new >> 4) != (self.resgiters.c >>4);
                            self.resgiters.c = new
                        },
                        TargetResgister::D => {
                            let (new, overflow) = self.resgiters.d.overflowing_add(1);
                            self.resgiters.f.zero = new==0;
                            self.resgiters.f.subtract = false;
                            self.resgiters.f.half_carry = (new >> 4) != (self.resgiters.d >>4);
                            self.resgiters.d = new
                        },
                        TargetResgister::E => {
                            let (new, overflow) = self.resgiters.e.overflowing_add(1);
                            self.resgiters.f.zero = new==0;
                            self.resgiters.f.subtract = false;
                            self.resgiters.f.half_carry = (new >> 4) != (self.resgiters.e >>4);
                            self.resgiters.e = new
                        },
                        TargetResgister::H => {
                            let (new, overflow) = self.resgiters.h.overflowing_add(1);
                            self.resgiters.f.zero = new==0;
                            self.resgiters.f.subtract = false;
                            self.resgiters.f.half_carry = (new >> 4) != (self.resgiters.h >>4);
                            self.resgiters.h = new
                        },
                        TargetResgister::L => {
                            let (new, overflow) = self.resgiters.l.overflowing_add(1);
                            self.resgiters.f.zero = new==0;
                            self.resgiters.f.subtract = false;
                            self.resgiters.f.half_carry = (new >> 4) != (self.resgiters.l >>4);
                            self.resgiters.l = new
                        },
                        TargetResgister::HL =>{
                            let value = self.bus.read_byte(self.resgiters.get_hl());
                            let (new, overflow) = value.overflowing_add(1);
                            self.resgiters.f.zero = new==0;
                            self.resgiters.f.subtract = false;
                            self.resgiters.f.half_carry = (new >> 4) != (value >>4);
                            self.bus.write_byte(self.resgiters.get_hl(), new)
                        },
                        _=> panic!("Incrémentation target")
                    };
                    self.pc.wrapping_add(1)
                },
                Instructions::DEC(target)=> {
                    match target {
                        TargetResgister::A => {
                            let (new, overflow) = self.resgiters.a.overflowing_sub(1);
                            self.resgiters.f.zero = new==0;
                            self.resgiters.f.subtract = false;
                            self.resgiters.f.half_carry = (new >> 4) != (self.resgiters.a >>4);
                            self.resgiters.a = new
                        },
                        TargetResgister::B => {
                            let (new, overflow) = self.resgiters.b.overflowing_sub(1);
                            self.resgiters.f.zero = new==0;
                            self.resgiters.f.subtract = false;
                            self.resgiters.f.half_carry = (new >> 4) != (self.resgiters.b >>4);
                            self.resgiters.b = new
                        },
                        TargetResgister::C => {
                            let (new, overflow) = self.resgiters.c.overflowing_sub(1);
                            self.resgiters.f.zero = new==0;
                            self.resgiters.f.subtract = false;
                            self.resgiters.f.half_carry = (new >> 4) != (self.resgiters.c >>4);
                            self.resgiters.c = new
                        },
                        TargetResgister::D => {
                            let (new, overflow) = self.resgiters.d.overflowing_sub(1);
                            self.resgiters.f.zero = new==0;
                            self.resgiters.f.subtract = false;
                            self.resgiters.f.half_carry = (new >> 4) != (self.resgiters.d >>4);
                            self.resgiters.d = new
                        },
                        TargetResgister::E => {
                            let (new, overflow) = self.resgiters.e.overflowing_sub(1);
                            self.resgiters.f.zero = new==0;
                            self.resgiters.f.subtract = false;
                            self.resgiters.f.half_carry = (new >> 4) != (self.resgiters.e >>4);
                            self.resgiters.e = new
                        },
                        TargetResgister::H => {
                            let (new, overflow) = self.resgiters.h.overflowing_sub(1);
                            self.resgiters.f.zero = new==0;
                            self.resgiters.f.subtract = false;
                            self.resgiters.f.half_carry = (new >> 4) != (self.resgiters.h >>4);
                            self.resgiters.h = new
                        },
                        TargetResgister::L => {
                            let (new, overflow) = self.resgiters.l.overflowing_sub(1);
                            self.resgiters.f.zero = new==0;
                            self.resgiters.f.subtract = false;
                            self.resgiters.f.half_carry = (new >> 4) != (self.resgiters.l >>4);
                            self.resgiters.l = new
                        },
                        TargetResgister::HL =>{
                            let value = self.bus.read_byte(self.resgiters.get_hl());
                            let (new, overflow) = value.overflowing_sub(1);
                            self.resgiters.f.zero = new==0;
                            self.resgiters.f.subtract = false;
                            self.resgiters.f.half_carry = (new >> 4) != (value >>4);
                            self.bus.write_byte(self.resgiters.get_hl(), new)
                        },
                        _=> panic!("Incrémentation target")
                    };
                    self.pc.wrapping_add(1)
                },
                Instructions::AND(target) => {
                    match target {
                        TargetResgister::A => self.and(self.resgiters.a),
                        TargetResgister::B => self.and(self.resgiters.b),
                        TargetResgister::C => self.and(self.resgiters.c),
                        TargetResgister::D => self.and(self.resgiters.d),
                        TargetResgister::E => self.and(self.resgiters.e),
                        TargetResgister::H => self.and(self.resgiters.h),
                        TargetResgister::HL => self.and(self.bus.read_byte(self.resgiters.get_hl())),
                        TargetResgister::d8 => {
                            self.and(self.bus.read_byte(self.pc + 1)); 
                            self.pc = self.pc.wrapping_add(1)},
                        _=> panic!("AND target")
                    }
                    self.pc.wrapping_add(1)

                },
                Instructions::OR(target) => {
                    match target {
                        TargetResgister::A => self.or(self.resgiters.a),
                        TargetResgister::B => self.or(self.resgiters.b),
                        TargetResgister::C => self.or(self.resgiters.c),
                        TargetResgister::D => self.or(self.resgiters.d),
                        TargetResgister::E => self.or(self.resgiters.e),
                        TargetResgister::H => self.or(self.resgiters.h),
                        TargetResgister::HL => self.or(self.bus.read_byte(self.resgiters.get_hl())),
                        TargetResgister::d8 => {
                            self.or(self.bus.read_byte(self.pc + 1)); 
                            self.pc = self.pc.wrapping_add(1)},
                        _=> panic!("OR target")
                    }
                    self.pc.wrapping_add(1)

                },
                Instructions::XOR(target) => {
                    match target {
                        TargetResgister::A => self.xor(self.resgiters.a),
                        TargetResgister::B => self.xor(self.resgiters.b),
                        TargetResgister::C => self.xor(self.resgiters.c),
                        TargetResgister::D => self.xor(self.resgiters.d),
                        TargetResgister::E => self.xor(self.resgiters.e),
                        TargetResgister::H => self.xor(self.resgiters.h),
                        TargetResgister::HL => self.xor(self.bus.read_byte(self.resgiters.get_hl())),
                        TargetResgister::d8 => {
                            self.xor(self.bus.read_byte(self.pc + 1)); 
                            self.pc = self.pc.wrapping_add(1)},
                        _=> panic!("XOR target")
                    }
                    self.pc.wrapping_add(1)

                },
                Instructions::CCF() => {
                    self.ccf();
                    self.pc.wrapping_add(1)
                },
                Instructions::SCF() => {
                    self.scf();
                    self.pc.wrapping_add(1)
                },
                Instructions::CPL()=> {
                    self.cpl();
                    self.pc.wrapping_add(1)
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
                Instructions::LD(load_type) => {
                    match load_type {
                        LoadType::Byte(target,source)=>{
                            let source_value = match source {
                                LoadByteSource::A => self.resgiters.a,
                                LoadByteSource::B => self.resgiters.b,
                                LoadByteSource::C => self.resgiters.c,
                                LoadByteSource::D => self.resgiters.d,
                                LoadByteSource::E => self.resgiters.e,
                                LoadByteSource::H => self.resgiters.h,
                                LoadByteSource::L => self.resgiters.l,
                                LoadByteSource::D8 => self.read_next_byte(),
                                LoadByteSource::HL => self.bus.read_byte(self.resgiters.get_hl()),
                                LoadByteSource::BC => self.bus.read_byte(self.resgiters.get_bc()),
                                LoadByteSource::DE => self.bus.read_byte(self.resgiters.get_de()),
                                LoadByteSource::HLI => self.bus.read_byte(self.resgiters.get_hl() + 1),
                                LoadByteSource::HLD => self.bus.read_byte(self.resgiters.get_hl() - 1),
                            };

                            match target {
                                LoadByteTarget::A => self.resgiters.a = source_value,
                                LoadByteTarget::B => self.resgiters.b = source_value,
                                LoadByteTarget::C => self.resgiters.c = source_value,
                                LoadByteTarget::D => self.resgiters.d = source_value,
                                LoadByteTarget::E => self.resgiters.e = source_value,
                                LoadByteTarget::H => self.resgiters.h = source_value,
                                LoadByteTarget::L => self.resgiters.l = source_value,
                                LoadByteTarget::HL => self.bus.write_byte(self.resgiters.get_hl(), source_value),
                                LoadByteTarget::HLI => self.bus.write_byte(self.resgiters.get_hl() + 1, source_value),
                                LoadByteTarget::HLD => self.bus.write_byte(self.resgiters.get_hl() - 1, source_value),
                                LoadByteTarget::BC => self.bus.write_byte(self.resgiters.get_bc(), source_value),
                                LoadByteTarget::DE => self.bus.write_byte(self.resgiters.get_de(), source_value),
                            };

                            match source {
                                LoadByteSource::D8 => self.pc.wrapping_add(2),
                                _=> self.pc.wrapping_add(1),
                            }
                        }
                    }
                }
                Instructions::PUSH(target) => {
                    let value = match target {
                        StackTarget::BC => self.resgiters.get_bc(),
                        StackTarget::DE => self.resgiters.get_de(),
                        StackTarget::HL => self.resgiters.get_hl(),
                        StackTarget::AF => self.resgiters.get_af(),
                    };
                    self.push(value);
                    self.pc.wrapping_add(1)
                },
                Instructions::POP(target) => {
                    let pop = self.pop();
                    match target {
                        StackTarget::BC => self.resgiters.set_bc(pop),
                        StackTarget::DE => self.resgiters.set_de(pop),
                        StackTarget::HL => self.resgiters.set_hl(pop),
                        StackTarget::AF => self.resgiters.set_af(pop),
                    };
                    self.pc.wrapping_add(1)
                },
                Instructions::CALL(jump)=>{
                    let jump_condition = match jump {
                        JumpTest::Always => true,
                        JumpTest::Carry => self.resgiters.f.carry,
                        JumpTest::NotCarry => !self.resgiters.f.carry,
                        JumpTest::Zero => self.resgiters.f.zero,
                        JumpTest::NotZero => !self.resgiters.f.zero
                    };
                    self.call(jump_condition)
                },
                Instructions::RET(jump)=>{
                    let jump_condition = match jump {
                        JumpTest::Always => true,
                        JumpTest::Carry => self.resgiters.f.carry,
                        JumpTest::NotCarry => !self.resgiters.f.carry,
                        JumpTest::Zero => self.resgiters.f.zero,
                        JumpTest::NotZero => !self.resgiters.f.zero
                    };
                    self.ret(jump_condition)
                },
                Instructions::NOP()=> self.pc.wrapping_add(1),
                Instructions::HALT() => {self.is_halted = true; self.pc.wrapping_add(1)},
                Instructions::DI() => {self.ime =false; self.pc.wrapping_add(1)},
                _=> self.pc,
            }
        } else {
            self.pc
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
        let lower_a = self.resgiters.a & 0xF;
        let lower_value = value & 0xF;
        let half_carry = lower_a & lower_value;
        self.resgiters.f.half_carry = half_carry>15 ;
        new 

    }

    pub fn adc(&mut self, value: u8) -> u8 {
        let (value_c, overflow_c) = value.overflowing_add(u8::from(self.resgiters.f) & 0x10);
        let(new, overflow) = self.resgiters.a.overflowing_add(value_c);

        self.resgiters.f.zero = new==0;
        self.resgiters.f.carry = overflow || overflow_c;
        self.resgiters.f.subtract = false;

        let lower_a = self.resgiters.a & 0xF;
        let lower_value = value & 0xF;
        let half_carry = lower_a & lower_value;
        self.resgiters.f.half_carry = half_carry>15 ;
        new 

    }

    pub fn sub(&mut self, value: u8) -> u8 {
        let (new, overflow) = self.resgiters.a.overflowing_sub(value);
        self.resgiters.f.zero = new==0;
        self.resgiters.f.carry = overflow;
        self.resgiters.f.subtract = true;
        let lower_a = self.resgiters.a & 0xF;
        let lower_value = value & 0xF;
        let half_carry = lower_a - lower_value;
        self.resgiters.f.half_carry = half_carry>15 ;

        new
    }

    pub fn sbc(&mut self, value: u8) -> u8 {
        let (value_c, overflow_c) = value.overflowing_sub(u8::from(self.resgiters.f) & 0x10);
        let (new, overflow) = self.resgiters.a.overflowing_sub(value_c);
        self.resgiters.f.zero = new==0;
        self.resgiters.f.carry = overflow||overflow_c;
        self.resgiters.f.subtract = true;
        let lower_a = self.resgiters.a & 0xF;
        let lower_value = value & 0xF;
        let half_carry = lower_a - lower_value;
        self.resgiters.f.half_carry = half_carry>15 ;

        new
    }

    pub fn and(&mut self, value: u8) {
        let new = self.resgiters.a & value;
        self.resgiters.f.zero = new == 0;
        self.resgiters.f.subtract = false;
        self.resgiters.f.half_carry = true;
        self.resgiters.f.carry = false;
        self.resgiters.a = new;
    }
    
    pub fn or(&mut self, value: u8) {
        let new = self.resgiters.a | value;
        self.resgiters.f.zero = new == 0;
        self.resgiters.f.subtract = false;
        self.resgiters.f.half_carry = true;
        self.resgiters.f.carry = false;
        self.resgiters.a = new;
    } 
    
    pub fn xor(&mut self, value: u8) {
        let new = self.resgiters.a ^ value;
        self.resgiters.f.zero = new == 0;
        self.resgiters.f.subtract = false;
        self.resgiters.f.half_carry = true;
        self.resgiters.f.carry = false;
        self.resgiters.a = new;
    }

    pub fn ccf(&mut self){
        self.resgiters.f.subtract = false;
        self.resgiters.f.half_carry = false;
        self.resgiters.f.carry = !self.resgiters.f.carry;
    }

    pub fn scf(&mut self){
        self.resgiters.f.subtract = false;
        self.resgiters.f.half_carry = false;
        self.resgiters.f.carry = true;
    }

    pub fn cpl(&mut self){
        self.resgiters.f.subtract = true;
        self.resgiters.f.half_carry = true;
        self.resgiters.a = !self.resgiters.a;
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

    pub fn read_next_byte(&self) -> u8 {
        self.bus.read_byte(self.pc + 1)

    }

    pub fn push(&mut self, value: u16){
        let lower = (value & 0xFF) as u8;
        let higher = ((value & 0xFF00)>>8) as u8;

        self.bus.write_byte(self.sp -1 , higher);
        self.bus.write_byte(self.sp -2, lower);
        self.sp = self.sp.wrapping_sub(2);
    }

    pub fn pop(&mut self)-> u16{
        let higher = self.bus.read_byte(self.sp + 1);
        let lower = self.bus.read_byte(self.sp);
        self.sp = self.sp.wrapping_add(2);

        (higher as u16) <<8 | lower as u16 
    }

    pub fn call(&mut self, jump: bool) -> u16{
        let next_program = self.pc.wrapping_add(3);
        if jump {
        self.push(next_program);
        self.jump(jump)
        } else {
            next_program
        }

    }

    pub fn ret(&mut self, jump: bool) -> u16{
        if jump {
            self.pop() 
        } else {
            self.pc.wrapping_add(1)           
        }

    }

}

