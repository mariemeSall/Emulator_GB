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
    BC, DE, HL, SP, A16

}
pub enum LoadWordSource {
    SP, D16, HL, SP8
}
pub enum LoadATarget {
    A, C, A8, A16

}
pub enum LoadASource {
    A, C, A8, A16
}
pub enum LoadType {
  Byte(LoadByteTarget, LoadByteSource),
  Word(LoadWordTarget, LoadWordSource),
  A(LoadATarget, LoadASource),
}

pub enum StackTarget {
    BC, DE, HL, AF
}
pub enum Target8{
    A,B,C,D,E,H,L,HL, D8
}
pub enum Target16{
    BC, DE, HL, SP, R8
}
pub enum TargetType {
    A(Target8), HL(Target16)
}

pub enum JumpTest {
    NotZero,
    Zero,
    NotCarry,
    Carry,
}

pub enum Condition {
    Yes(JumpTest),
    No(JumpValue),
}

pub enum JumpValue {
    A16, Hl
}

pub enum JumpCond {
    Jump(JumpTest),
    True,
}

pub enum Instructions {
    ADD(TargetType),
    LD(LoadType),
    PUSH(StackTarget),
    POP(StackTarget),
    CALL(JumpCond),
    RET(JumpCond),
    RETI(),
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
    RLCA(),
    RLA(), 
    DI(),
    EI(),
    RRA(), 
    RRCA(),
    DAA(),
    RLC(Target8),
    RL(Target8),
    RRC(Target8),
    RR(Target8),
    SLA(Target8),
    SWAP(Target8),
    SRA(Target8),
    SRL(Target8),
    BIT(u8, Target8),
    SET(u8, Target8),
    RES(u8, Target8),
    JP(Condition),
    JR(JumpCond),

    //TODO
    JPI(),
    STOP(),
    
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
            0x01 => Some(Instructions::LD(LoadType::Word(LoadWordTarget::BC, LoadWordSource::D16))),
            0x02 => Some(Instructions::LD(LoadType::Byte(LoadByteTarget::BC, LoadByteSource::A))),
            0x03 => Some(Instructions::INC(TargetType::HL(Target16::BC))),
            0x04 => Some(Instructions::INC(TargetType::A(Target8::B))),
            0x05 => Some(Instructions::DEC(TargetType::A(Target8::B))),
            0x06 => Some(Instructions::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::D8))),
            0x07 => Some(Instructions::RLCA()),
            0x08 => Some(Instructions::LD(LoadType::Word(LoadWordTarget::A16, LoadWordSource::SP))),
            0x09 => Some(Instructions::ADD(TargetType::HL(Target16::BC))),
            0x0A => Some(Instructions::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::BC))),
            0x0B => Some(Instructions::DEC(TargetType::HL(Target16::BC))),
            0x0C => Some(Instructions::INC(TargetType::A(Target8::C))),
            0x0D => Some(Instructions::DEC(TargetType::A(Target8::C))),
            0x0E => Some(Instructions::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::D8))),
            0x0F => Some(Instructions::RRCA()),
            0x10 => Some(Instructions::STOP()),
            0x11 => Some(Instructions::LD(LoadType::Word(LoadWordTarget::DE, LoadWordSource::D16))),
            0x12 => Some(Instructions::LD(LoadType::Byte(LoadByteTarget::DE, LoadByteSource::A))),
            0x13 => Some(Instructions::INC(TargetType::HL(Target16::DE))),
            0x14 => Some(Instructions::INC(TargetType::A(Target8::D))),
            0x15 => Some(Instructions::DEC(TargetType::A(Target8::D))),
            0x16 => Some(Instructions::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::D8))),
            0x17 => Some(Instructions::RLA()),
            0x18 => Some(Instructions::JR(JumpCond::True)),
            0x19 => Some(Instructions::ADD(TargetType::HL(Target16::DE))),
            0x1A => Some(Instructions::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::DE))),
            0x1B => Some(Instructions::DEC(TargetType::HL(Target16::DE))),
            0x1C => Some(Instructions::INC(TargetType::A(Target8::E))),
            0x1D => Some(Instructions::DEC(TargetType::A(Target8::E))),
            0x1E => Some(Instructions::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::D8))),
            0x1F => Some(Instructions::RRA()),
            0x20 => Some(Instructions::JR(JumpCond::Jump(JumpTest::NotZero))),
            0x21 => Some(Instructions::LD(LoadType::Word(LoadWordTarget::HL, LoadWordSource::D16))),
            0x22 => Some(Instructions::LD(LoadType::Byte(LoadByteTarget::HLI, LoadByteSource::A))),
            0x23 => Some(Instructions::INC(TargetType::HL(Target16::HL))),
            0x24 => Some(Instructions::INC(TargetType::A(Target8::H))),
            0x25 => Some(Instructions::DEC(TargetType::A(Target8::H))),
            0x26 => Some(Instructions::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::D8))),
            0x27 => Some(Instructions::DAA()),
            0x28 => Some(Instructions::JR(JumpCond::Jump(JumpTest::Zero))),
            0x29 => Some(Instructions::ADD(TargetType::HL(Target16::HL))),
            0x2A => Some(Instructions::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::HLI))),
            0x2B => Some(Instructions::DEC(TargetType::HL(Target16::HL))),
            0x2C => Some(Instructions::INC(TargetType::A(Target8::L))),
            0x2D => Some(Instructions::DEC(TargetType::A(Target8::L))),
            0x2E => Some(Instructions::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::D8))),
            0x2F => Some(Instructions::CPL()),
            0x30 => Some(Instructions::JR(JumpCond::Jump(JumpTest::NotCarry))),
            0x31 => Some(Instructions::LD(LoadType::Word(LoadWordTarget::SP, LoadWordSource::D16))),
            0x32 => Some(Instructions::LD(LoadType::Byte(LoadByteTarget::HLD, LoadByteSource::A))),
            0x33 => Some(Instructions::INC(TargetType::HL(Target16::SP))),
            0x34 => Some(Instructions::INC(TargetType::A(Target8::HL))),
            0x35 => Some(Instructions::DEC(TargetType::A(Target8::HL))),
            0x36 => Some(Instructions::LD(LoadType::Byte(LoadByteTarget::HL, LoadByteSource::D8))),
            0x37 => Some(Instructions::SCF()),
            0x38 => Some(Instructions::JR(JumpCond::Jump(JumpTest::Carry))),
            0x39 => Some(Instructions::ADD(TargetType::HL(Target16::SP))),
            0x3A => Some(Instructions::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::HLD))),
            0x3B => Some(Instructions::DEC(TargetType::HL(Target16::SP))),
            0x3C => Some(Instructions::INC(TargetType::A(Target8::A))),
            0x3D => Some(Instructions::DEC(TargetType::A(Target8::A))),
            0x3E => Some(Instructions::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::D8))),
            0x3F => Some(Instructions::CCF()),
            0x40 => Some(Instructions::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::B))),
            0x41 => Some(Instructions::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::C))),
            0x42 => Some(Instructions::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::D))),
            0x43 => Some(Instructions::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::E))),
            0x44 => Some(Instructions::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::H))),
            0x45 => Some(Instructions::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::L))),
            0x46 => Some(Instructions::LD(LoadType::Byte(LoadByteTarget::B,LoadByteSource::HLI))),
            0x47 => Some(Instructions::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::A))),
            0x48 => Some(Instructions::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::B))),
            0x49 => Some(Instructions::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::C))),
            0x4A => Some(Instructions::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::D))),
            0x4B => Some(Instructions::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::E))),
            0x4C => Some(Instructions::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::H))),
            0x4D => Some(Instructions::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::L))),
            0x4E => Some(Instructions::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::HLI))),
            0x4F => Some(Instructions::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::A))),
            0x50 => Some(Instructions::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::B))),
            0x51 => Some(Instructions::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::C))),
            0x52 => Some(Instructions::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::D))),
            0x53 => Some(Instructions::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::E))),
            0x54 => Some(Instructions::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::H))),
            0x55 => Some(Instructions::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::L))),
            0x56 => Some(Instructions::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::HLI))),
            0x57 => Some(Instructions::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::A))),
            0x58 => Some(Instructions::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::B))),
            0x59 => Some(Instructions::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::C))),
            0x5A => Some(Instructions::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::D))),
            0x5B => Some(Instructions::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::E))),
            0x5C => Some(Instructions::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::H))),
            0x5D => Some(Instructions::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::L))),
            0x5E => Some(Instructions::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::HLI))),
            0x5F => Some(Instructions::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::A))),
            0x60 => Some(Instructions::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::B))),
            0x61 => Some(Instructions::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::C))),
            0x62 => Some(Instructions::LD(LoadType::Byte(LoadByteTarget::H,LoadByteSource::D))),
            0x63 => Some(Instructions::LD(LoadType::Byte(LoadByteTarget::H,LoadByteSource::E))),
            0x64 => Some(Instructions::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::H))),
            0x65 => Some(Instructions::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::L))),
            0x66 => Some(Instructions::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::HLI))),
            0x67 => Some(Instructions::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::A))),
            0x68 => Some(Instructions::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::B))),
            0x69 => Some(Instructions::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::C))),
            0x6A => Some(Instructions::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::D))),
            0x6B => Some(Instructions::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::E))),
            0x6C => Some(Instructions::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::H))),
            0x6D => Some(Instructions::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::L))),
            0x6E => Some(Instructions::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::HLI))),
            0x6F => Some(Instructions::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::A))),
            0x70 => Some(Instructions::LD(LoadType::Byte(LoadByteTarget::HLI, LoadByteSource::B))),
            0x71 => Some(Instructions::LD(LoadType::Byte(LoadByteTarget::HLI, LoadByteSource::C))),
            0x72 => Some(Instructions::LD(LoadType::Byte(LoadByteTarget::HLI, LoadByteSource::D))),
            0x73 => Some(Instructions::LD(LoadType::Byte(LoadByteTarget::HLI, LoadByteSource::E))),
            0x74 => Some(Instructions::LD(LoadType::Byte(LoadByteTarget::HLI, LoadByteSource::H))),
            0x75 => Some(Instructions::LD(LoadType::Byte(LoadByteTarget::HLI, LoadByteSource::L))),
            0x76 => Some(Instructions::HALT()),
            0x77 => Some(Instructions::LD(LoadType::Byte(LoadByteTarget::HLI, LoadByteSource::A))),
            0x78 => Some(Instructions::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::B))),
            0x79 => Some(Instructions::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::C))),
            0x7A => Some(Instructions::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::D))),
            0x7B => Some(Instructions::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::E))),
            0x7C => Some(Instructions::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::H))),
            0x7D => Some(Instructions::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::L))),
            0x7E => Some(Instructions::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::HLI))),
            0x7F => Some(Instructions::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::A))),
            0x80 => Some(Instructions::ADD(TargetType::A(Target8::B))),
            0x81 => Some(Instructions::ADD(TargetType::A(Target8::C))),
            0x82 => Some(Instructions::ADD(TargetType::A(Target8::D))),
            0x83 => Some(Instructions::ADD(TargetType::A(Target8::E))),
            0x84 => Some(Instructions::ADD(TargetType::A(Target8::H))),
            0x85 => Some(Instructions::ADD(TargetType::A(Target8::L))),
            0x86 => Some(Instructions::ADD(TargetType::A(Target8::HL))),
            0x87 => Some(Instructions::ADD(TargetType::A(Target8::A))),
            0x88 => Some(Instructions::ADC(Target8::B)),
            0x89 => Some(Instructions::ADC(Target8::C)),
            0x8A => Some(Instructions::ADC(Target8::D)),
            0x8B => Some(Instructions::ADC(Target8::E)),
            0x8C => Some(Instructions::ADC(Target8::H)),
            0x8D => Some(Instructions::ADC(Target8::L)),
            0x8E => Some(Instructions::ADC(Target8::HL)),
            0x8F => Some(Instructions::ADC(Target8::A)),
            0x90 => Some(Instructions::SUB(Target8::B)),
            0x91 => Some(Instructions::SUB(Target8::C)),
            0x92 => Some(Instructions::SUB(Target8::D)),
            0x93 => Some(Instructions::SUB(Target8::E)),
            0x94 => Some(Instructions::SUB(Target8::H)),
            0x95 => Some(Instructions::SUB(Target8::L)),
            0x96 => Some(Instructions::SUB(Target8::HL)),
            0x97 => Some(Instructions::SUB(Target8::A)),
            0x98 => Some(Instructions::SBC(Target8::B)),
            0x99 => Some(Instructions::SBC(Target8::C)),
            0x9A => Some(Instructions::SBC(Target8::D)),
            0x9B => Some(Instructions::SBC(Target8::E)),
            0x9C => Some(Instructions::SBC(Target8::H)),
            0x9D => Some(Instructions::SBC(Target8::L)),
            0x9E => Some(Instructions::SBC(Target8::HL)),
            0x9F => Some(Instructions::SBC(Target8::A)),
            0xA0 => Some(Instructions::AND(Target8::B)),
            0xA1 => Some(Instructions::AND(Target8::C)),
            0xA2 => Some(Instructions::AND(Target8::D)),
            0xA3 => Some(Instructions::AND(Target8::E)),
            0xA4 => Some(Instructions::AND(Target8::H)),
            0xA5 => Some(Instructions::AND(Target8::L)),
            0xA6 => Some(Instructions::AND(Target8::HL)),
            0xA7 => Some(Instructions::AND(Target8::A)),
            0xA8 => Some(Instructions::XOR(Target8::B)),
            0xA9 => Some(Instructions::XOR(Target8::C)),
            0xAA => Some(Instructions::XOR(Target8::D)),
            0xAB => Some(Instructions::XOR(Target8::E)),
            0xAC => Some(Instructions::XOR(Target8::H)),
            0xAD => Some(Instructions::XOR(Target8::L)),
            0xAE => Some(Instructions::XOR(Target8::HL)),
            0xAF => Some(Instructions::XOR(Target8::A)),
            0xB0 => Some(Instructions::OR(Target8::B)),
            0xB1 => Some(Instructions::OR(Target8::C)),
            0xB2 => Some(Instructions::OR(Target8::D)),
            0xB3 => Some(Instructions::OR(Target8::E)),
            0xB4 => Some(Instructions::OR(Target8::H)),
            0xB5 => Some(Instructions::OR(Target8::L)),
            0xB6 => Some(Instructions::OR(Target8::HL)),
            0xB7 => Some(Instructions::OR(Target8::A)),
            0xB8 => Some(Instructions::CP(Target8::B)),
            0xB9 => Some(Instructions::CP(Target8::C)),
            0xBA => Some(Instructions::CP(Target8::D)),
            0xBB => Some(Instructions::CP(Target8::E)),
            0xBC => Some(Instructions::CP(Target8::H)),
            0xBD => Some(Instructions::CP(Target8::L)),
            0xBE => Some(Instructions::CP(Target8::HL)),
            0xBF => Some(Instructions::CP(Target8::A)),
            0xC0 => Some(Instructions::RET(JumpCond::Jump(JumpTest::NotZero))),
            0xC1 => Some(Instructions::POP(StackTarget::BC)),
            0xC2 => Some(Instructions::JP(Condition::Yes(JumpTest::NotZero))),
            0xC3 => Some(Instructions::JP(Condition::No(JumpValue::A16))),
            0xC4 => Some(Instructions::CALL(JumpCond::Jump(JumpTest::NotZero))),
            0xC5 => Some(Instructions::PUSH(StackTarget::BC)),
            0xC6 => Some(Instructions::ADD(TargetType::A(Target8::D8))),
            0xC8 => Some(Instructions::RET(JumpCond::Jump(JumpTest::Zero))),
            0xC9 => Some(Instructions::RET(JumpCond::True)),
            0xCA => Some(Instructions::JP(Condition::Yes(JumpTest::Zero))), 
            0xCC => Some(Instructions::CALL(JumpCond::Jump(JumpTest::Zero))),
            0xCD => Some(Instructions::CALL(JumpCond::True)),
            0xCE => Some(Instructions::ADC(Target8::D8)),
            0xD0 => Some(Instructions::RET(JumpCond::Jump(JumpTest::NotCarry))),
            0xD1 => Some(Instructions::POP(StackTarget::DE)),
            0xD2 => Some(Instructions::JP(Condition::Yes(JumpTest::NotCarry))),
            0xD4 => Some(Instructions::CALL(JumpCond::Jump(JumpTest::NotCarry))),
            0xD5 => Some(Instructions::PUSH(StackTarget::DE)),
            0xD6 => Some(Instructions::SUB(Target8::D8)),
            0xD8 => Some(Instructions::RET(JumpCond::Jump(JumpTest::Carry))),
            0xD9 => Some(Instructions::RETI()),
            0xDA => Some(Instructions::JP(Condition::Yes(JumpTest::Carry))),
            0xDC => Some(Instructions::CALL(JumpCond::Jump(JumpTest::Carry))),
            0xDE => Some(Instructions::SBC(Target8::D8)),
            0xE0 => Some(Instructions::LD(LoadType::A(LoadATarget::A8, LoadASource::A))), 
            0xE1 => Some(Instructions::POP(StackTarget::HL)),
            0xE2 => Some(Instructions::LD(LoadType::A(LoadATarget::C, LoadASource::A))), 
            0xE5 => Some(Instructions::PUSH(StackTarget::HL)),
            0xE6 => Some(Instructions::AND(Target8::D8)),
            0xE8 => Some(Instructions::ADD(TargetType::HL(Target16::R8))),
            0xE9 => Some(Instructions::JP(Condition::No(JumpValue::Hl))),
            0xEA => Some(Instructions::LD(LoadType::A(LoadATarget::A16, LoadASource::A))), 
            0xEE => Some(Instructions::XOR(Target8::D8)),
            0xF0 => Some(Instructions::LD(LoadType::A(LoadATarget::A, LoadASource::A8))), 
            0xF1 => Some(Instructions::POP(StackTarget::AF)),
            0xF2 => Some(Instructions::LD(LoadType::A(LoadATarget::A, LoadASource::C))), 
            0xF3 => Some(Instructions::DI()),
            0xF5 => Some(Instructions::PUSH(StackTarget::AF)),
            0xF6 => Some(Instructions::OR(Target8::D8)),
            0xF8 => Some(Instructions::LD(LoadType::Word(LoadWordTarget::HL, LoadWordSource::SP8))),
            0xF9 => Some(Instructions::LD(LoadType::Word(LoadWordTarget::SP, LoadWordSource::HL))),
            0xFA => Some(Instructions::LD(LoadType::A(LoadATarget::A, LoadASource::A16))), 
            0xFB => Some(Instructions::EI()),
            0xFE => Some(Instructions::CP(Target8::D8)),
            _ => None,
        }
    }

    //Fonction pour retourver l'instruction avec prefixe
    pub fn from_bytes_prefixed(byte: u8) -> Option<Instructions>{
        match byte {
            0x00 => Some(Instructions::RLC(Target8::B)),
            0x01 => Some(Instructions::RLC(Target8::C)),
            0x02 => Some(Instructions::RLC(Target8::D)),
            0x03 => Some(Instructions::RLC(Target8::E)),
            0x04 => Some(Instructions::RLC(Target8::H)),
            0x05 => Some(Instructions::RLC(Target8::L)),
            0x06 => Some(Instructions::RLC(Target8::HL)),
            0x07 => Some(Instructions::RLC(Target8::A)),
            0x08 => Some(Instructions::RRC(Target8::B)),
            0x09 => Some(Instructions::RRC(Target8::C)),
            0x0A => Some(Instructions::RRC(Target8::D)),
            0x0B => Some(Instructions::RRC(Target8::E)),
            0x0C => Some(Instructions::RRC(Target8::H)),
            0x0D => Some(Instructions::RRC(Target8::L)),
            0x0E => Some(Instructions::RRC(Target8::HL)),
            0x0F => Some(Instructions::RRC(Target8::A)),
            0x10 => Some(Instructions::RL(Target8::B)),
            0x11 => Some(Instructions::RL(Target8::C)),
            0x12 => Some(Instructions::RL(Target8::D)),
            0x13 => Some(Instructions::RL(Target8::E)),
            0x14 => Some(Instructions::RL(Target8::H)),
            0x15 => Some(Instructions::RL(Target8::L)),
            0x16 => Some(Instructions::RL(Target8::HL)),
            0x17 => Some(Instructions::RL(Target8::A)),
            0x18 => Some(Instructions::RR(Target8::B)),
            0x19 => Some(Instructions::RR(Target8::C)),
            0x1A => Some(Instructions::RR(Target8::D)),
            0x1B => Some(Instructions::RR(Target8::E)),
            0x1C => Some(Instructions::RR(Target8::H)),
            0x1D => Some(Instructions::RR(Target8::L)),
            0x1E => Some(Instructions::RR(Target8::HL)),
            0x1F => Some(Instructions::RR(Target8::A)),
            0x20 => Some(Instructions::SLA(Target8::B)),
            0x21 => Some(Instructions::SLA(Target8::C)),
            0x22 => Some(Instructions::SLA(Target8::D)),
            0x23 => Some(Instructions::SLA(Target8::E)),
            0x24 => Some(Instructions::SLA(Target8::H)),
            0x25 => Some(Instructions::SLA(Target8::L)),
            0x26 => Some(Instructions::SLA(Target8::HL)),
            0x27 => Some(Instructions::SLA(Target8::A)),
            0x28 => Some(Instructions::SRA(Target8::B)),
            0x29 => Some(Instructions::SRA(Target8::C)),
            0x2A => Some(Instructions::SRA(Target8::D)),
            0x2B => Some(Instructions::SRA(Target8::E)),
            0x2C => Some(Instructions::SRA(Target8::H)),
            0x2D => Some(Instructions::SRA(Target8::L)),
            0x2E => Some(Instructions::SRA(Target8::HL)),
            0x2F => Some(Instructions::SRA(Target8::A)),
            0x30 => Some(Instructions::SWAP(Target8::B)),
            0x31 => Some(Instructions::SWAP(Target8::C)),
            0x32 => Some(Instructions::SWAP(Target8::D)),
            0x33 => Some(Instructions::SWAP(Target8::E)),
            0x34 => Some(Instructions::SWAP(Target8::H)),
            0x35 => Some(Instructions::SWAP(Target8::L)),
            0x36 => Some(Instructions::SWAP(Target8::HL)),
            0x37 => Some(Instructions::SWAP(Target8::A)),
            0x38 => Some(Instructions::SRL(Target8::B)),
            0x39 => Some(Instructions::SRL(Target8::C)),
            0x3A => Some(Instructions::SRL(Target8::D)),
            0x3B => Some(Instructions::SRL(Target8::E)),
            0x3C => Some(Instructions::SRL(Target8::H)),
            0x3D => Some(Instructions::SRL(Target8::L)),
            0x3E => Some(Instructions::SRL(Target8::HL)),
            0x3F => Some(Instructions::SRL(Target8::A)),
            0x40 => Some(Instructions::BIT(0, Target8::B)),
            0x41 => Some(Instructions::BIT(0, Target8::C)),
            0x42 => Some(Instructions::BIT(0, Target8::D)),
            0x43 => Some(Instructions::BIT(0, Target8::E)),
            0x44 => Some(Instructions::BIT(0, Target8::H)),
            0x45 => Some(Instructions::BIT(0, Target8::L)),
            0x46 => Some(Instructions::BIT(0, Target8::HL)),
            0x47 => Some(Instructions::BIT(0, Target8::A)),
            0x48 => Some(Instructions::BIT(1, Target8::B)),
            0x49 => Some(Instructions::BIT(1, Target8::C)),
            0x4A => Some(Instructions::BIT(1, Target8::D)),
            0x4B => Some(Instructions::BIT(1, Target8::E)),
            0x4C => Some(Instructions::BIT(1, Target8::H)),
            0x4D => Some(Instructions::BIT(1, Target8::L)),
            0x4E => Some(Instructions::BIT(1, Target8::HL)),
            0x4F => Some(Instructions::BIT(1, Target8::A)),
            0x50 => Some(Instructions::BIT(2, Target8::B)),
            0x51 => Some(Instructions::BIT(2, Target8::C)),
            0x52 => Some(Instructions::BIT(2, Target8::D)),
            0x53 => Some(Instructions::BIT(2, Target8::E)),
            0x54 => Some(Instructions::BIT(2, Target8::H)),
            0x55 => Some(Instructions::BIT(2, Target8::L)),
            0x56 => Some(Instructions::BIT(2, Target8::HL)),
            0x57 => Some(Instructions::BIT(2, Target8::A)),
            0x58 => Some(Instructions::BIT(3, Target8::B)),
            0x59 => Some(Instructions::BIT(3, Target8::C)),
            0x5A => Some(Instructions::BIT(3, Target8::D)),
            0x5B => Some(Instructions::BIT(3, Target8::E)),
            0x5C => Some(Instructions::BIT(3, Target8::H)),
            0x5D => Some(Instructions::BIT(3, Target8::L)),
            0x5E => Some(Instructions::BIT(3, Target8::HL)),
            0x5F => Some(Instructions::BIT(3, Target8::A)),
            0x60 => Some(Instructions::BIT(4, Target8::B)),
            0x61 => Some(Instructions::BIT(4, Target8::C)),
            0x62 => Some(Instructions::BIT(4, Target8::D)),
            0x63 => Some(Instructions::BIT(4, Target8::E)),
            0x64 => Some(Instructions::BIT(4, Target8::H)),
            0x65 => Some(Instructions::BIT(4, Target8::L)),
            0x66 => Some(Instructions::BIT(4, Target8::HL)),
            0x67 => Some(Instructions::BIT(4, Target8::A)),
            0x68 => Some(Instructions::BIT(5, Target8::B)),
            0x69 => Some(Instructions::BIT(5, Target8::C)),
            0x6A => Some(Instructions::BIT(5, Target8::D)),
            0x6B => Some(Instructions::BIT(5, Target8::E)),
            0x6C => Some(Instructions::BIT(5, Target8::H)),
            0x6D => Some(Instructions::BIT(5, Target8::L)),
            0x6E => Some(Instructions::BIT(5, Target8::HL)),
            0x6F => Some(Instructions::BIT(5, Target8::A)),
            0x70 => Some(Instructions::BIT(6, Target8::B)),
            0x71 => Some(Instructions::BIT(6, Target8::C)),
            0x72 => Some(Instructions::BIT(6, Target8::D)),
            0x73 => Some(Instructions::BIT(6, Target8::E)),
            0x74 => Some(Instructions::BIT(6, Target8::H)),
            0x75 => Some(Instructions::BIT(6, Target8::L)),
            0x76 => Some(Instructions::BIT(6, Target8::HL)),
            0x77 => Some(Instructions::BIT(6, Target8::A)),
            0x78 => Some(Instructions::BIT(7, Target8::B)),
            0x79 => Some(Instructions::BIT(7, Target8::C)),
            0x7A => Some(Instructions::BIT(7, Target8::D)),
            0x7B => Some(Instructions::BIT(7, Target8::E)),
            0x7C => Some(Instructions::BIT(7, Target8::H)),
            0x7D => Some(Instructions::BIT(7, Target8::L)),
            0x7E => Some(Instructions::BIT(7, Target8::HL)),
            0x7F => Some(Instructions::BIT(7, Target8::A)),
            0x80 => Some(Instructions::RES(0, Target8::B)),
            0x81 => Some(Instructions::RES(0, Target8::C)),
            0x82 => Some(Instructions::RES(0, Target8::D)),
            0x83 => Some(Instructions::RES(0, Target8::E)),
            0x84 => Some(Instructions::RES(0, Target8::H)),
            0x85 => Some(Instructions::RES(0, Target8::L)),
            0x86 => Some(Instructions::RES(0, Target8::HL)),
            0x87 => Some(Instructions::RES(0, Target8::A)),
            0x88 => Some(Instructions::RES(1, Target8::B)),
            0x89 => Some(Instructions::RES(1, Target8::C)),
            0x8A => Some(Instructions::RES(1, Target8::D)),
            0x8B => Some(Instructions::RES(1, Target8::E)),
            0x8C => Some(Instructions::RES(1, Target8::H)),
            0x8D => Some(Instructions::RES(1, Target8::L)),
            0x8E => Some(Instructions::RES(1, Target8::HL)),
            0x8F => Some(Instructions::RES(1, Target8::A)),
            0x90 => Some(Instructions::RES(2, Target8::B)),
            0x91 => Some(Instructions::RES(2, Target8::C)),
            0x92 => Some(Instructions::RES(2, Target8::D)),
            0x93 => Some(Instructions::RES(2, Target8::E)),
            0x94 => Some(Instructions::RES(2, Target8::H)),
            0x95 => Some(Instructions::RES(2, Target8::L)),
            0x96 => Some(Instructions::RES(2, Target8::HL)),
            0x97 => Some(Instructions::RES(2, Target8::A)),
            0x98 => Some(Instructions::RES(3, Target8::B)),
            0x99 => Some(Instructions::RES(3, Target8::C)),
            0x9A => Some(Instructions::RES(3, Target8::D)),
            0x9B => Some(Instructions::RES(3, Target8::E)),
            0x9C => Some(Instructions::RES(3, Target8::H)),
            0x9D => Some(Instructions::RES(3, Target8::L)),
            0x9E => Some(Instructions::RES(3, Target8::HL)),
            0x9F => Some(Instructions::RES(3, Target8::A)),
            0xA0 => Some(Instructions::RES(4, Target8::B)),
            0xA1 => Some(Instructions::RES(4, Target8::C)),
            0xA2 => Some(Instructions::RES(4, Target8::D)),
            0xA3 => Some(Instructions::RES(4, Target8::E)),
            0xA4 => Some(Instructions::RES(4, Target8::H)),
            0xA5 => Some(Instructions::RES(4, Target8::L)),
            0xA6 => Some(Instructions::RES(4, Target8::HL)),
            0xA7 => Some(Instructions::RES(4, Target8::A)),
            0xA8 => Some(Instructions::RES(5, Target8::B)),
            0xA9 => Some(Instructions::RES(5, Target8::C)),
            0xAA => Some(Instructions::RES(5, Target8::D)),
            0xAB => Some(Instructions::RES(5, Target8::E)),
            0xAC => Some(Instructions::RES(5, Target8::H)),
            0xAD => Some(Instructions::RES(5, Target8::L)),
            0xAE => Some(Instructions::RES(5, Target8::HL)),
            0xAF => Some(Instructions::RES(5, Target8::A)),
            0xB0 => Some(Instructions::RES(6, Target8::B)),
            0xB1 => Some(Instructions::RES(6, Target8::C)),
            0xB2 => Some(Instructions::RES(6, Target8::D)),
            0xB3 => Some(Instructions::RES(6, Target8::E)),
            0xB4 => Some(Instructions::RES(6, Target8::H)),
            0xB5 => Some(Instructions::RES(6, Target8::L)),
            0xB6 => Some(Instructions::RES(6, Target8::HL)),
            0xB7 => Some(Instructions::RES(6, Target8::A)),
            0xB8 => Some(Instructions::RES(7, Target8::B)),
            0xB9 => Some(Instructions::RES(7, Target8::C)),
            0xBA => Some(Instructions::RES(7, Target8::D)),
            0xBB => Some(Instructions::RES(7, Target8::E)),
            0xBC => Some(Instructions::RES(7, Target8::H)),
            0xBD => Some(Instructions::RES(7, Target8::L)),
            0xBE => Some(Instructions::RES(7, Target8::HL)),
            0xBF => Some(Instructions::RES(7, Target8::A)),
            0xC0 => Some(Instructions::SET(0, Target8::B)),
            0xC1 => Some(Instructions::SET(0, Target8::C)),
            0xC2 => Some(Instructions::SET(0, Target8::D)),
            0xC3 => Some(Instructions::SET(0, Target8::E)),
            0xC4 => Some(Instructions::SET(0, Target8::H)),
            0xC5 => Some(Instructions::SET(0, Target8::L)),
            0xC6 => Some(Instructions::SET(0, Target8::HL)),
            0xC7 => Some(Instructions::SET(0, Target8::A)),
            0xC8 => Some(Instructions::SET(1, Target8::B)),
            0xC9 => Some(Instructions::SET(1, Target8::C)),
            0xCA => Some(Instructions::SET(1, Target8::D)),
            0xCB => Some(Instructions::SET(1, Target8::E)),
            0xCC => Some(Instructions::SET(1, Target8::H)),
            0xCD => Some(Instructions::SET(1, Target8::L)),
            0xCE => Some(Instructions::SET(1, Target8::HL)),
            0xCF => Some(Instructions::SET(1, Target8::A)),
            0xD0 => Some(Instructions::SET(2, Target8::B)),
            0xD1 => Some(Instructions::SET(2, Target8::C)),
            0xD2 => Some(Instructions::SET(2, Target8::D)),
            0xD3 => Some(Instructions::SET(2, Target8::E)),
            0xD4 => Some(Instructions::SET(2, Target8::H)),
            0xD5 => Some(Instructions::SET(2, Target8::L)),
            0xD6 => Some(Instructions::SET(2, Target8::HL)),
            0xD7 => Some(Instructions::SET(2, Target8::A)),
            0xD8 => Some(Instructions::SET(3, Target8::B)),
            0xD9 => Some(Instructions::SET(3, Target8::C)),
            0xDA => Some(Instructions::SET(3, Target8::D)),
            0xDB => Some(Instructions::SET(3, Target8::E)),
            0xDC => Some(Instructions::SET(3, Target8::H)),
            0xDD => Some(Instructions::SET(3, Target8::L)),
            0xDE => Some(Instructions::SET(3, Target8::HL)),
            0xDF => Some(Instructions::SET(3, Target8::A)),
            0xE0 => Some(Instructions::SET(4, Target8::B)),
            0xE1 => Some(Instructions::SET(4, Target8::C)),
            0xE2 => Some(Instructions::SET(4, Target8::D)),
            0xE3 => Some(Instructions::SET(4, Target8::E)),
            0xE4 => Some(Instructions::SET(4, Target8::H)),
            0xE5 => Some(Instructions::SET(4, Target8::L)),
            0xE6 => Some(Instructions::SET(4, Target8::HL)),
            0xE7 => Some(Instructions::SET(4, Target8::A)),
            0xE8 => Some(Instructions::SET(5, Target8::B)),
            0xE9 => Some(Instructions::SET(5, Target8::C)),
            0xEA => Some(Instructions::SET(5, Target8::D)),
            0xEB => Some(Instructions::SET(5, Target8::E)),
            0xEC => Some(Instructions::SET(5, Target8::H)),
            0xED => Some(Instructions::SET(5, Target8::L)),
            0xEE => Some(Instructions::SET(5, Target8::HL)),
            0xEF => Some(Instructions::SET(5, Target8::A)),
            0xF0 => Some(Instructions::SET(6, Target8::B)),
            0xF1 => Some(Instructions::SET(6, Target8::C)),
            0xF2 => Some(Instructions::SET(6, Target8::D)),
            0xF3 => Some(Instructions::SET(6, Target8::E)),
            0xF4 => Some(Instructions::SET(6, Target8::H)),
            0xF5 => Some(Instructions::SET(6, Target8::L)),
            0xF6 => Some(Instructions::SET(6, Target8::HL)),
            0xF7 => Some(Instructions::SET(6, Target8::A)),
            0xF8 => Some(Instructions::SET(7, Target8::B)),
            0xF9 => Some(Instructions::SET(7, Target8::C)),
            0xFA => Some(Instructions::SET(7, Target8::D)),
            0xFB => Some(Instructions::SET(7, Target8::E)),
            0xFC => Some(Instructions::SET(7, Target8::H)),
            0xFD => Some(Instructions::SET(7, Target8::L)),
            0xFE => Some(Instructions::SET(7, Target8::HL)),
            0xFF => Some(Instructions::SET(7, Target8::A)),
        }
    }
}


impl CPU {

    pub fn execute(&mut self, instruction: Instructions) -> u16{
        if !self.is_halted{
            match instruction {
                Instructions::ADD(add)=>{
                    match add {
                        TargetType::A(target) =>{
                            match target {
                                Target8::A=> {
                                    self.resgiters.a = self.add(self.resgiters.a); 
                                    self.pc.wrapping_add(1)
                                },
                                Target8::B=> {
                                    let value = self.resgiters.b;
                                    self.resgiters.a = self.add(value);
                                    self.pc.wrapping_add(1)
                                },
                                Target8::C=> {
                                    let value = self.resgiters.c;
                                    self.resgiters.a = self.add(value);
                                    self.pc.wrapping_add(1)
                                },
                                Target8::D=> {
                                    let value = self.resgiters.d;
                                    self.resgiters.a = self.add(value);
                                    self.pc.wrapping_add(1)
                                },
                                Target8::E=> {
                                    let value = self.resgiters.e;
                                    self.resgiters.a = self.add(value);                    
                                    self.pc.wrapping_add(1)
                                },
                                Target8::H=> {
                                    let value = self.resgiters.h;
                                    self.resgiters.a = self.add(value);                    
                                    self.pc.wrapping_add(1)
                                },
                                Target8::L=> {
                                    let value = self.resgiters.l;
                                    self.resgiters.a = self.add(value);
                                    self.pc.wrapping_add(1)
                                },
                                Target8::HL => {
                                    let value = self.bus.read_byte(self.resgiters.get_hl());
                                    self.resgiters.a = self.add(value);
                                    self.pc.wrapping_add(1)
                                },
                                Target8::D8 => {
                                    let value = self.bus.read_byte(self.pc + 1);
                                    self.resgiters.a = self.add(value);
                                    self.pc.wrapping_add(2)
                                }
                            }
                        },
                        TargetType::HL(target) => {
                            match target {
                                Target16::BC => {
                                    let value = self.resgiters.get_bc();
                                    self.add_hl(value);
                                    self.pc.wrapping_add(1)
                                },
                                Target16::DE => {
                                    let value = self.resgiters.get_de();
                                    self.add_hl(value);
                                    self.pc.wrapping_add(1)
                                },
                                Target16::HL => {
                                    let value = self.resgiters.get_hl();
                                    self.add_hl(value);
                                    self.pc.wrapping_add(1)
                                },
                                Target16::SP => {
                                    let value = self.sp;
                                    self.add_hl(value);
                                    self.pc.wrapping_add(1)
                                },
                                Target16::R8=> {
                                    let n = self.bus.read_byte(self.pc + 1 ) as i8;
                                    let (new, overflow) = self.resgiters.get_hl().overflowing_add_signed(n as i16);
                                    self.resgiters.f.zero = new==0;
                                    self.resgiters.f.carry = overflow;
                                    let (_, overflow8) = ((self.resgiters.get_hl() & 0xFF) as u8).overflowing_add_signed(n);
                                    self.resgiters.f.half_carry= overflow8;
                            
                                    self.resgiters.set_hl(new);

                                    self.pc.wrapping_add(2)
                                },
                            }
                          

                        }
                    }
                },
                Instructions::ADC(target) => {
                    match target {
                        Target8::A=> {
                            self.resgiters.a = self.adc(self.resgiters.a); 
                            self.pc.wrapping_add(1)
                        },
                        Target8::B=> {
                            let value = self.resgiters.b;
                            self.resgiters.a = self.adc(value);
                            self.pc.wrapping_add(1)
                        },
                        Target8::C=> {
                            let value = self.resgiters.c;
                            self.resgiters.a = self.adc(value);
                            self.pc.wrapping_add(1)
                        },
                        Target8::D=> {
                            let value = self.resgiters.d;
                            self.resgiters.a = self.adc(value);
                            self.pc.wrapping_add(1)
                        },
                        Target8::E=> {
                            let value = self.resgiters.e;
                            self.resgiters.a = self.adc(value);                    
                            self.pc.wrapping_add(1)
                        },
                        Target8::H=> {
                            let value = self.resgiters.h;
                            self.resgiters.a = self.adc(value);                    
                            self.pc.wrapping_add(1)
                        },
                        Target8::L=> {
                            let value = self.resgiters.l;
                            self.resgiters.a = self.adc(value);
                            self.pc.wrapping_add(1)
                        },
                        Target8::HL => {
                            let value = self.bus.read_byte(self.resgiters.get_hl());
                            self.resgiters.a = self.adc(value);
                            self.pc.wrapping_add(1)
                        },
                        Target8::D8 => {
                            let value = self.bus.read_byte(self.pc + 1);
                            self.resgiters.a = self.add(value);
                            self.pc.wrapping_add(2)
                        }
                    }
                },
                Instructions::SUB(target)=> {
                    match target {
                        Target8::A => {
                            self.resgiters.a = self.sub(self.resgiters.a);
                            self.pc.wrapping_add(1)
                        }
                        Target8::B => {
                            self.resgiters.a = self.sub(self.resgiters.b);
                            self.pc.wrapping_add(1)
                        }
                        Target8::C => {
                            self.resgiters.a = self.sub(self.resgiters.c);
                            self.pc.wrapping_add(1)
                        }
                        Target8::D => {
                            self.resgiters.a = self.sub(self.resgiters.d);
                            self.pc.wrapping_add(1)
                        }
                        Target8::E => {
                            self.resgiters.a = self.sub(self.resgiters.e);
                            self.pc.wrapping_add(1)
                        }
                        Target8::H => {
                            self.resgiters.a = self.sub(self.resgiters.h);
                            self.pc.wrapping_add(1)
                        }
                        Target8::L => {
                            self.resgiters.a = self.sub(self.resgiters.l);
                            self.pc.wrapping_add(1)
                        }
                        Target8::HL => {
                            self.resgiters.a = self.sub(self.bus.read_byte(self.resgiters.get_hl()));
                            self.pc.wrapping_add(1)
                        }
                        Target8::D8 => {
                            self.resgiters.a = self.sub(self.bus.read_byte(self.pc + 1));
                            self.pc.wrapping_add(2)
                        }
                    }
                },
                Instructions::SBC(target)=> {
                    match target {
                        Target8::A => {
                            self.resgiters.a = self.sbc(self.resgiters.a);
                            self.pc.wrapping_add(1)
                        }
                        Target8::B => {
                            self.resgiters.a = self.sbc(self.resgiters.b);
                            self.pc.wrapping_add(1)
                        }
                        Target8::C => {
                            self.resgiters.a = self.sbc(self.resgiters.c);
                            self.pc.wrapping_add(1)
                        }
                        Target8::D => {
                            self.resgiters.a = self.sbc(self.resgiters.d);
                            self.pc.wrapping_add(1)
                        }
                        Target8::E => {
                            self.resgiters.a = self.sbc(self.resgiters.e);
                            self.pc.wrapping_add(1)
                        }
                        Target8::H => {
                            self.resgiters.a = self.sbc(self.resgiters.h);
                            self.pc.wrapping_add(1)
                        }
                        Target8::L => {
                            self.resgiters.a = self.sbc(self.resgiters.l);
                            self.pc.wrapping_add(1)
                        }
                        Target8::HL => {
                            self.resgiters.a = self.sbc(self.bus.read_byte(self.resgiters.get_hl()));
                            self.pc.wrapping_add(1)
                        }
                        Target8::D8 => {
                            self.resgiters.a = self.sbc(self.bus.read_byte(self.pc + 1));
                            self.pc.wrapping_add(2)
                        }
                    }
                },
                Instructions::CP(target)=> {
                    match target {
                        Target8::A => {
                            self.sub(self.resgiters.a);
                            self.pc.wrapping_add(1)
                        }
                        Target8::B => {
                            self.sub(self.resgiters.b);
                            self.pc.wrapping_add(1)
                        }
                        Target8::C => {
                            self.sub(self.resgiters.c);
                            self.pc.wrapping_add(1)
                        }
                        Target8::D => {
                            self.sub(self.resgiters.d);
                            self.pc.wrapping_add(1)
                        }
                        Target8::E => {
                            self.sub(self.resgiters.e);
                            self.pc.wrapping_add(1)
                        }
                        Target8::H => {
                            self.sub(self.resgiters.h);
                            self.pc.wrapping_add(1)
                        }
                        Target8::L => {
                            self.sub(self.resgiters.l);
                            self.pc.wrapping_add(1)
                        }
                        Target8::HL => {
                            self.sub(self.bus.read_byte(self.resgiters.get_hl()));
                            self.pc.wrapping_add(1)
                        }
                        Target8::D8 => {
                           self.sub(self.bus.read_byte(self.pc + 1));
                           self.pc.wrapping_add(2)

                        }
                    }
                },
                Instructions::INC(target)=> {
                    match target {
                        TargetType::A(t8) => {
                            match t8 {
                                Target8::A => {
                                    let (new, _overflow) = self.resgiters.a.overflowing_add(1);
                                    self.resgiters.f.zero = new==0;
                                    self.resgiters.f.subtract = false;
                                    self.resgiters.f.half_carry = (new >> 4) != (self.resgiters.a >>4);
                                    self.resgiters.a = new
                                },
                                Target8::B => {
                                    let (new, _overflow) = self.resgiters.b.overflowing_add(1);
                                    self.resgiters.f.zero = new==0;
                                    self.resgiters.f.subtract = false;
                                    self.resgiters.f.half_carry = (new >> 4) != (self.resgiters.b >>4);
                                    self.resgiters.b = new
                                },
                                Target8::C => {
                                    let (new, _overflow) = self.resgiters.c.overflowing_add(1);
                                    self.resgiters.f.zero = new==0;
                                    self.resgiters.f.subtract = false;
                                    self.resgiters.f.half_carry = (new >> 4) != (self.resgiters.c >>4);
                                    self.resgiters.c = new
                                },
                                Target8::D => {
                                    let (new, _overflow) = self.resgiters.d.overflowing_add(1);
                                    self.resgiters.f.zero = new==0;
                                    self.resgiters.f.subtract = false;
                                    self.resgiters.f.half_carry = (new >> 4) != (self.resgiters.d >>4);
                                    self.resgiters.d = new
                                },
                                Target8::E => {
                                    let (new, _overflow) = self.resgiters.e.overflowing_add(1);
                                    self.resgiters.f.zero = new==0;
                                    self.resgiters.f.subtract = false;
                                    self.resgiters.f.half_carry = (new >> 4) != (self.resgiters.e >>4);
                                    self.resgiters.e = new
                                },
                                Target8::H => {
                                    let (new, _overflow) = self.resgiters.h.overflowing_add(1);
                                    self.resgiters.f.zero = new==0;
                                    self.resgiters.f.subtract = false;
                                    self.resgiters.f.half_carry = (new >> 4) != (self.resgiters.h >>4);
                                    self.resgiters.h = new
                                },
                                Target8::L => {
                                    let (new, _overflow) = self.resgiters.l.overflowing_add(1);
                                    self.resgiters.f.zero = new==0;
                                    self.resgiters.f.subtract = false;
                                    self.resgiters.f.half_carry = (new >> 4) != (self.resgiters.l >>4);
                                    self.resgiters.l = new
                                },
                                Target8::HL =>{
                                    let value = self.bus.read_byte(self.resgiters.get_hl());
                                    let (new, _overflow) = value.overflowing_add(1);
                                    self.resgiters.f.zero = new==0;
                                    self.resgiters.f.subtract = false;
                                    self.resgiters.f.half_carry = (new >> 4) != (value >>4);
                                    self.bus.write_byte(self.resgiters.get_hl(), new)
                                },
                                _=> panic!("Incrémentation target")

                            }
                        },
                        TargetType::HL(t16)=>{
                            match t16 {
                                Target16::BC => self.resgiters.set_bc(self.resgiters.get_bc() + 1) ,
                                Target16::DE => self.resgiters.set_de(self.resgiters.get_de() + 1),
                                Target16::HL => self.resgiters.set_hl(self.resgiters.get_hl() + 1),
                                Target16::SP => self.sp +=1,
                                _=> panic!("INC target 16")
                            }
                        },                        
                    };
                    self.pc.wrapping_add(1)
                },
                Instructions::DEC(target)=> {
                    match target {
                        TargetType::A(t8) => {
                            match t8 {
                                Target8::A => {
                                    let (new, _overflow) = self.resgiters.a.overflowing_sub(1);
                                    self.resgiters.f.zero = new==0;
                                    self.resgiters.f.subtract = true;
                                    self.resgiters.f.half_carry = (new >> 4) != (self.resgiters.a >>4);
                                    self.resgiters.a = new
                                },
                                Target8::B => {
                                    let (new, _overflow) = self.resgiters.b.overflowing_sub(1);
                                    self.resgiters.f.zero = new==0;
                                    self.resgiters.f.subtract = true;
                                    self.resgiters.f.half_carry = (new >> 4) != (self.resgiters.b >>4);
                                    self.resgiters.b = new
                                },
                                Target8::C => {
                                    let (new, _overflow) = self.resgiters.c.overflowing_sub(1);
                                    self.resgiters.f.zero = new==0;
                                    self.resgiters.f.subtract = true;
                                    self.resgiters.f.half_carry = (new >> 4) != (self.resgiters.c >>4);
                                    self.resgiters.c = new
                                },
                                Target8::D => {
                                    let (new, _overflow) = self.resgiters.d.overflowing_sub(1);
                                    self.resgiters.f.zero = new==0;
                                    self.resgiters.f.subtract = true;
                                    self.resgiters.f.half_carry = (new >> 4) != (self.resgiters.d >>4);
                                    self.resgiters.d = new
                                },
                                Target8::E => {
                                    let (new, _overflow) = self.resgiters.e.overflowing_sub(1);
                                    self.resgiters.f.zero = new==0;
                                    self.resgiters.f.subtract = true;
                                    self.resgiters.f.half_carry = (new >> 4) != (self.resgiters.e >>4);
                                    self.resgiters.e = new
                                },
                                Target8::H => {
                                    let (new, _overflow) = self.resgiters.h.overflowing_sub(1);
                                    self.resgiters.f.zero = new==0;
                                    self.resgiters.f.subtract = true;
                                    self.resgiters.f.half_carry = (new >> 4) != (self.resgiters.h >>4);
                                    self.resgiters.h = new
                                },
                                Target8::L => {
                                    let (new, _overflow) = self.resgiters.l.overflowing_sub(1);
                                    self.resgiters.f.zero = new==0;
                                    self.resgiters.f.subtract = true;
                                    self.resgiters.f.half_carry = (new >> 4) != (self.resgiters.l >>4);
                                    self.resgiters.l = new
                                },
                                Target8::HL =>{
                                    let value = self.bus.read_byte(self.resgiters.get_hl());
                                    let (new, _overflow) = value.overflowing_sub(1);
                                    self.resgiters.f.zero = new==0;
                                    self.resgiters.f.subtract = true;
                                    self.resgiters.f.half_carry = (new >> 4) != (value >>4);
                                    self.bus.write_byte(self.resgiters.get_hl(), new)
                                },
                                _=> panic!("Decrémentation target")

                            }
                        },
                        TargetType::HL(t16)=>{
                            match t16 {
                                Target16::BC => self.resgiters.set_bc(self.resgiters.get_bc() - 1) ,
                                Target16::DE => self.resgiters.set_de(self.resgiters.get_de() - 1),
                                Target16::HL => self.resgiters.set_hl(self.resgiters.get_hl() - 1),
                                Target16::SP => self.sp -=1,
                                _ => panic!("DEC target 16")
                            }
                        },                        
                    };
                    self.pc.wrapping_add(1)
                },
                Instructions::AND(target) => {
                    match target {
                        Target8::A => self.and(self.resgiters.a),
                        Target8::B => self.and(self.resgiters.b),
                        Target8::C => self.and(self.resgiters.c),
                        Target8::D => self.and(self.resgiters.d),
                        Target8::E => self.and(self.resgiters.e),
                        Target8::H => self.and(self.resgiters.h),
                        Target8::HL => self.and(self.bus.read_byte(self.resgiters.get_hl())),
                        Target8::D8 => {
                            self.and(self.bus.read_byte(self.pc + 1)); 
                            self.pc = self.pc.wrapping_add(1)},
                        _=> panic!("AND target")
                    }
                    self.pc.wrapping_add(1)

                },
                Instructions::OR(target) => {
                    match target {
                        Target8::A => self.or(self.resgiters.a),
                        Target8::B => self.or(self.resgiters.b),
                        Target8::C => self.or(self.resgiters.c),
                        Target8::D => self.or(self.resgiters.d),
                        Target8::E => self.or(self.resgiters.e),
                        Target8::H => self.or(self.resgiters.h),
                        Target8::HL => self.or(self.bus.read_byte(self.resgiters.get_hl())),
                        Target8::D8 => {
                            self.or(self.bus.read_byte(self.pc + 1)); 
                            self.pc = self.pc.wrapping_add(1)},
                        _=> panic!("OR target")
                    }
                    self.pc.wrapping_add(1)

                },
                Instructions::XOR(target) => {
                    match target {
                        Target8::A => self.xor(self.resgiters.a),
                        Target8::B => self.xor(self.resgiters.b),
                        Target8::C => self.xor(self.resgiters.c),
                        Target8::D => self.xor(self.resgiters.d),
                        Target8::E => self.xor(self.resgiters.e),
                        Target8::H => self.xor(self.resgiters.h),
                        Target8::HL => self.xor(self.bus.read_byte(self.resgiters.get_hl())),
                        Target8::D8 => {
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
                Instructions::JP(condition) => {
                    match condition {
                        Condition::Yes(jump)=> {
                            let jump_condition = match jump {
                                JumpTest::Carry => self.resgiters.f.carry,
                                JumpTest::NotCarry => !self.resgiters.f.carry,
                                JumpTest::Zero => self.resgiters.f.zero,
                                JumpTest::NotZero => !self.resgiters.f.zero
                            };
                            self.jump(jump_condition)

                        },
                        Condition::No(a16) => {
                            match a16 {
                                JumpValue::A16 => self.jump(true) ,
                                JumpValue::Hl => self.resgiters.get_hl(),
                            }
                        },

                    }

                    
                },
                Instructions::JR(jmp)=> {
                    match jmp {
                        JumpCond::True => {
                            self.jr(true)
                        },
                        JumpCond::Jump(jump) => {
                            let jump_condition = match jump {
                                JumpTest::Carry => self.resgiters.f.carry,
                                JumpTest::NotCarry => !self.resgiters.f.carry,
                                JumpTest::Zero => self.resgiters.f.zero,
                                JumpTest::NotZero => !self.resgiters.f.zero
                            };
                            self.jr(jump_condition)
                        },
                    }

                },
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
                        },
                        LoadType::Word(target,source ) => {
                            let source_value = match source {
                                LoadWordSource::SP => self.sp,
                                LoadWordSource::D16 => (self.bus.read_byte(self.pc + 1) as u16) & ((self.bus.read_byte(self.pc + 2) as u16 )<< 8),
                                LoadWordSource::HL => self.resgiters.get_hl(),
                                LoadWordSource::SP8 => {
                                    let r = self.bus.read_byte(self.pc + 1) as i8;
                                    let (new, overflow) = self.sp.overflowing_add_signed(r as i16);
                                    self.resgiters.f.zero = false;
                                    self.resgiters.f.subtract = false;
                                    self.resgiters.f.carry = overflow;
                                    self.resgiters.f.half_carry = ((new>>8) as u8) != ((self.sp >>8) as u8);

                                    new
                                },
                            };

                            match target {
                                LoadWordTarget::BC => self.resgiters.set_bc(source_value) ,
                                LoadWordTarget::DE => self.resgiters.set_de(source_value),
                                LoadWordTarget::HL => self.resgiters.set_hl(source_value),
                                LoadWordTarget::SP => self.sp = source_value,
                                LoadWordTarget::A16 => {
                                    let nn = (self.bus.read_byte(self.pc + 1) as u16) & ((self.bus.read_byte(self.pc + 2) as u16 )<< 8);
                                     self.bus.write_byte(nn, (source_value & 0xFF) as u8);
                                     self.bus.write_byte(nn + 1, ((source_value & 0xFF00)>>8) as u8);
                                },
                            };

                            match source {
                                LoadWordSource::D16 => self.pc.wrapping_add(3),
                                _=> self.pc.wrapping_add(1)
                            }
                        },
                        LoadType::A(target,source ) => {
                            let source_value = match source {
                                LoadASource::C => self.bus.read_byte((self.resgiters.c as u16) & 0xFF00),
                                LoadASource::A => self.resgiters.a,
                                LoadASource::A8 => {let n = self.bus.read_byte(self.pc +1);
                                    self.bus.read_byte((n as u16)&0xFF00)
                                },
                                LoadASource::A16 => {
                                    let lower = self.bus.read_byte(self.pc + 1);
                                    let higher = self.bus.read_byte(self.pc + 2);
                                    self.bus.read_byte(((higher as u16)<<8)|(lower as u16))

                                },
                            };

                            match target {
                                LoadATarget::A => self.resgiters.a = source_value,
                                LoadATarget::C => self.bus.write_byte((self.resgiters.c as u16) & 0xFF00, source_value),
                                LoadATarget::A8 => self.bus.write_byte((self.bus.read_byte(self.pc +1) as u16)&0xFF00, source_value),
                                LoadATarget::A16 => {
                                    let lower = self.bus.read_byte(self.pc + 1);
                                    let higher = self.bus.read_byte(self.pc + 2);
                                    self.bus.write_byte(((higher as u16)<<8)|(lower as u16), source_value)

                                },
                            };

                            match source {
                                LoadASource::A16 => self.pc.wrapping_add(3),
                                _=> self.pc.wrapping_add(2)
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
                Instructions::CALL(jmp)=>{
                    match jmp {
                        JumpCond::True => self.call(true),
                        JumpCond::Jump(jump)=> {
                            let jump_condition = match jump {
                                JumpTest::Carry => self.resgiters.f.carry,
                                JumpTest::NotCarry => !self.resgiters.f.carry,
                                JumpTest::Zero => self.resgiters.f.zero,
                                JumpTest::NotZero => !self.resgiters.f.zero
                            };
                            self.call(jump_condition)
                        }
                    }
                    
                },
                Instructions::RET(jmp)=>{
                    match jmp {
                        JumpCond::True => self.ret(true),
                        JumpCond::Jump(jump) => {
                            let jump_condition = match jump {
                                JumpTest::Carry => self.resgiters.f.carry,
                                JumpTest::NotCarry => !self.resgiters.f.carry,
                                JumpTest::Zero => self.resgiters.f.zero,
                                JumpTest::NotZero => !self.resgiters.f.zero
                            };
                            self.ret(jump_condition)
                        }
                    }
                    
                },
                Instructions::RETI() => {
                    self.ime = true;
                    self.ret(true)
                },
                Instructions::NOP()=> self.pc.wrapping_add(1),
                Instructions::HALT() => {self.is_halted = true; self.pc.wrapping_add(1)},
                Instructions::DI() => {self.ime =false; self.pc.wrapping_add(1)},
                Instructions::EI() => {self.ime =true; self.pc.wrapping_add(1)},
                Instructions::RLCA() => {
                    let n = self.resgiters.a & 0x80;
                    self.resgiters.a = (self.resgiters.a <<1) | n;
                    self.resgiters.f.zero = self.resgiters.a ==0;
                    self.resgiters.f.carry = if n ==0 {false} else {true};
                    self.resgiters.f.half_carry = false;
                    self.resgiters.f.subtract =false;
                    self.pc.wrapping_add(1)
                },
                Instructions::RLA() => {
                    let n = self.resgiters.a & 0x80;
                    self.resgiters.a = (self.resgiters.a <<1) | (if self.resgiters.f.carry {1} else {0});
                    self.resgiters.f.carry = if n ==0 {false} else {true};
                    self.resgiters.f.half_carry = false;
                    self.resgiters.f.subtract =false;
                    self.resgiters.f.zero = self.resgiters.a ==0;
                    self.pc.wrapping_add(1)
                },
                Instructions::RRCA() => {
                    let n = self.resgiters.a & 0x01;
                    self.resgiters.a = (self.resgiters.a >>1) | n <<7;
                    self.resgiters.f.zero = self.resgiters.a ==0;
                    self.resgiters.f.carry = if n ==0 {false} else {true};
                    self.resgiters.f.half_carry = false;
                    self.resgiters.f.subtract =false;
                    self.pc.wrapping_add(1)
                },
                Instructions::RRA() => {
                    let n = self.resgiters.a & 0x01;
                    self.resgiters.a = (self.resgiters.a >>1) | ((if self.resgiters.f.carry {1} else {0}) as u8)<<7;
                    self.resgiters.f.carry = if n ==0 {false} else {true};
                    self.resgiters.f.half_carry = false;
                    self.resgiters.f.subtract =false;
                    self.resgiters.f.zero = self.resgiters.a ==0;
                    self.pc.wrapping_add(1)
                },
                Instructions::DAA() => {
                    let mut a = self.resgiters.a;
                    if self.resgiters.f.subtract {
                        if self.resgiters.f.carry || a>0x99 { a+= 0x60; self.resgiters.f.carry =true;}
                        if self.resgiters.f.half_carry || (a & 0x0f) > 0x09 { a+=0x06; }

                    } else {
                        if self.resgiters.f.carry  { a-= 0x60;}
                        if self.resgiters.f.half_carry  { a-=0x06; }
                    };

                    self.resgiters.f.half_carry = false;
                    self.resgiters.f.zero = a==0;

                    self.resgiters.a = a;


                    self.pc.wrapping_add(1)
                }
                
                //Instructions avec prefixe
                Instructions::RLC(target) => {
                    match target {
                        Target8::A => {
                            let n = self.resgiters.a & 0x80;
                            self.resgiters.a = (self.resgiters.a <<1 )| n;
                            self.resgiters.f.zero = self.resgiters.a ==0;
                            self.resgiters.f.carry = if n ==0 {false} else {true};
                            self.resgiters.f.half_carry = false;
                            self.resgiters.f.subtract =false;
                        },
                        Target8::B => {
                            let n = self.resgiters.b & 0x80;
                            self.resgiters.b = (self.resgiters.b <<1) | n;
                            self.resgiters.f.zero = self.resgiters.b ==0;
                            self.resgiters.f.carry = if n ==0 {false} else {true};
                            self.resgiters.f.half_carry = false;
                            self.resgiters.f.subtract =false; 
                        },
                        Target8::C => {
                            let n = self.resgiters.c & 0x80;
                            self.resgiters.c = (self.resgiters.c <<1) | n;
                            self.resgiters.f.zero = self.resgiters.c ==0;
                            self.resgiters.f.carry = if n ==0 {false} else {true};
                            self.resgiters.f.half_carry = false;
                            self.resgiters.f.subtract =false;
                        },
                        Target8::D => {
                            let n = self.resgiters.d & 0x80;
                            self.resgiters.d = (self.resgiters.d <<1) | n;
                            self.resgiters.f.zero = self.resgiters.d ==0;
                            self.resgiters.f.carry = if n ==0 {false} else {true};
                            self.resgiters.f.half_carry = false;
                            self.resgiters.f.subtract =false; 
                        }, 
                        Target8::H => {
                            let n = self.resgiters.h & 0x80;
                            self.resgiters.h = (self.resgiters.h <<1 )| n;
                            self.resgiters.f.zero = self.resgiters.h ==0;
                            self.resgiters.f.carry = if n ==0 {false} else {true};
                            self.resgiters.f.half_carry = false;
                            self.resgiters.f.subtract =false;
                        },
                        Target8::L => {
                            let n = self.resgiters.l & 0x80;
                            self.resgiters.l = (self.resgiters.l <<1 )| n;
                            self.resgiters.f.zero = self.resgiters.l ==0;
                            self.resgiters.f.carry = if n ==0 {false} else {true};
                            self.resgiters.f.half_carry = false;
                            self.resgiters.f.subtract =false; 
                        }, 
                        Target8::E => {
                            let n = self.resgiters.e & 0x80;
                            self.resgiters.e = (self.resgiters.e <<1 )| n;
                            self.resgiters.f.zero = self.resgiters.e ==0;
                            self.resgiters.f.carry = if n ==0 {false} else {true};
                            self.resgiters.f.half_carry = false;
                            self.resgiters.f.subtract =false;
                        },
                        Target8::HL => {
                            let value = self.bus.read_byte(self.resgiters.get_hl());
                            let n = value & 0x80;
                            self.bus.write_byte(self.resgiters.get_hl(), value <<1 | n);
                            self.resgiters.f.zero = self.bus.read_byte(self.resgiters.get_hl()) ==0;
                            self.resgiters.f.carry = if n ==0 {false} else {true};
                            self.resgiters.f.half_carry = false;
                            self.resgiters.f.subtract =false; 
                        },
                        _=> panic!("RLC target")
                    };

                    self.pc.wrapping_add(1)
                },
                Instructions::RL(target) => {
                    match target {
                        Target8::A => {
                            let n = self.resgiters.a & 0x80;
                            self.resgiters.a = (self.resgiters.a <<1) | (if self.resgiters.f.carry {1} else {0});
                            self.resgiters.f.zero = self.resgiters.a ==0;
                            self.resgiters.f.carry = if n ==0 {false} else {true};
                            self.resgiters.f.half_carry = false;
                            self.resgiters.f.subtract =false;
                        },
                        Target8::B => {
                            let n = self.resgiters.b & 0x80;
                            self.resgiters.b = (self.resgiters.b <<1) | (if self.resgiters.f.carry {1} else {0});
                            self.resgiters.f.zero = self.resgiters.b ==0;
                            self.resgiters.f.carry = if n ==0 {false} else {true};
                            self.resgiters.f.half_carry = false;
                            self.resgiters.f.subtract =false; 
                        },
                        Target8::C => {
                            let n = self.resgiters.c & 0x80;
                            self.resgiters.c = (self.resgiters.c <<1 )| (if self.resgiters.f.carry {1} else {0});
                            self.resgiters.f.zero = self.resgiters.c ==0;
                            self.resgiters.f.carry = if n ==0 {false} else {true};
                            self.resgiters.f.half_carry = false;
                            self.resgiters.f.subtract =false;
                        },
                        Target8::D => {
                            let n = self.resgiters.d & 0x80;
                            self.resgiters.d = (self.resgiters.d <<1 )| (if self.resgiters.f.carry {1} else {0});
                            self.resgiters.f.zero = self.resgiters.d ==0;
                            self.resgiters.f.carry = if n ==0 {false} else {true};
                            self.resgiters.f.half_carry = false;
                            self.resgiters.f.subtract =false; 
                        }, 
                        Target8::H => {
                            let n = self.resgiters.h & 0x80;
                            self.resgiters.h = (self.resgiters.h <<1) | (if self.resgiters.f.carry {1} else {0});
                            self.resgiters.f.zero = self.resgiters.h ==0;
                            self.resgiters.f.carry = if n ==0 {false} else {true};
                            self.resgiters.f.half_carry = false;
                            self.resgiters.f.subtract =false;
                        },
                        Target8::L => {
                            let n = self.resgiters.l & 0x80;
                            self.resgiters.l = (self.resgiters.l <<1) | (if self.resgiters.f.carry {1} else {0});
                            self.resgiters.f.zero = self.resgiters.l ==0;
                            self.resgiters.f.carry = if n ==0 {false} else {true};
                            self.resgiters.f.half_carry = false;
                            self.resgiters.f.subtract =false; 
                        }, 
                        Target8::E => {
                            let n = self.resgiters.e & 0x80;
                            self.resgiters.e = (self.resgiters.e <<1) | (if self.resgiters.f.carry {1} else {0});
                            self.resgiters.f.zero = self.resgiters.e ==0;
                            self.resgiters.f.carry = if n ==0 {false} else {true};
                            self.resgiters.f.half_carry = false;
                            self.resgiters.f.subtract =false;
                        },
                        Target8::HL => {
                            let value = self.bus.read_byte(self.resgiters.get_hl());
                            let n = value & 0x80;
                            self.bus.write_byte(self.resgiters.get_hl(), value <<1 | (if self.resgiters.f.carry {1} else {0}));
                            self.resgiters.f.zero = self.bus.read_byte(self.resgiters.get_hl()) ==0;
                            self.resgiters.f.carry = if n ==0 {false} else {true};
                            self.resgiters.f.half_carry = false;
                            self.resgiters.f.subtract =false; 
                        },
                        _=> panic!("RL target")
                        
                    };

                    self.pc.wrapping_add(1)

                },
                Instructions::RRC(target) => {
                    
                    match target {
                        Target8::A => {
                            let n = self.resgiters.a & 0x01;
                            self.resgiters.a = (self.resgiters.a >>1) | n <<7;
                            self.resgiters.f.zero = self.resgiters.a ==0;
                            self.resgiters.f.carry = if n ==0 {false} else {true};
                            self.resgiters.f.half_carry = false;
                            self.resgiters.f.subtract =false;

                        },
                        Target8::B => {
                            let n = self.resgiters.b & 0x01;
                            self.resgiters.b = (self.resgiters.b >>1 )| n<<7;
                            self.resgiters.f.zero = self.resgiters.b ==0;
                            self.resgiters.f.carry = if n ==0 {false} else {true};
                            self.resgiters.f.half_carry = false;
                            self.resgiters.f.subtract =false; 
                        },
                        Target8::C => {
                            let n = self.resgiters.c & 0x01;
                            self.resgiters.c = (self.resgiters.c >>1 )| n<<7;
                            self.resgiters.f.zero = self.resgiters.c ==0;
                            self.resgiters.f.carry = if n ==0 {false} else {true};
                            self.resgiters.f.half_carry = false;
                            self.resgiters.f.subtract =false;
                        },
                        Target8::D => {
                            let n = self.resgiters.d & 0x01;
                            self.resgiters.d = (self.resgiters.d >>1) | n<<7;
                            self.resgiters.f.zero = self.resgiters.d ==0;
                            self.resgiters.f.carry = if n ==0 {false} else {true};
                            self.resgiters.f.half_carry = false;
                            self.resgiters.f.subtract =false; 
                        }, 
                        Target8::H => {
                            let n = self.resgiters.h & 0x01;
                            self.resgiters.h = (self.resgiters.h >>1 )| n<<7;
                            self.resgiters.f.zero = self.resgiters.h ==0;
                            self.resgiters.f.carry = if n ==0 {false} else {true};
                            self.resgiters.f.half_carry = false;
                            self.resgiters.f.subtract =false;
                        },
                        Target8::L => {
                            let n = self.resgiters.l & 0x01;
                            self.resgiters.l = (self.resgiters.l >>1) | n<<7;
                            self.resgiters.f.zero = self.resgiters.l ==0;
                            self.resgiters.f.carry = if n ==0 {false} else {true};
                            self.resgiters.f.half_carry = false;
                            self.resgiters.f.subtract =false; 
                        }, 
                        Target8::E => {
                            let n = self.resgiters.e & 0x01;
                            self.resgiters.e = (self.resgiters.e >>1) | n<<7;
                            self.resgiters.f.zero = self.resgiters.e ==0;
                            self.resgiters.f.carry = if n ==0 {false} else {true};
                            self.resgiters.f.half_carry = false;
                            self.resgiters.f.subtract =false;
                        },
                        Target8::HL => {
                            let value = self.bus.read_byte(self.resgiters.get_hl());
                            let n = value & 0x01;
                            self.bus.write_byte(self.resgiters.get_hl(), value >>1 | n<<7);
                            self.resgiters.f.zero = self.bus.read_byte(self.resgiters.get_hl()) ==0;
                            self.resgiters.f.carry = if n ==0 {false} else {true};
                            self.resgiters.f.half_carry = false;
                            self.resgiters.f.subtract =false; 
                        },
                        _=> panic!("RRC target")
                    };
                    self.pc.wrapping_add(1)
                },
                Instructions::RR(target) => {
                    match target {
                        Target8::A => {
                            let n = self.resgiters.a & 0x01;
                            self.resgiters.a = (self.resgiters.a >>1) | ((if self.resgiters.f.carry {1} else {0}) as u8)<<7;
                            self.resgiters.f.zero = self.resgiters.a ==0;
                            self.resgiters.f.carry = if n ==0 {false} else {true};
                            self.resgiters.f.half_carry = false;
                            self.resgiters.f.subtract =false;

                        },
                        Target8::B => {
                            let n = self.resgiters.b & 0x01;
                            self.resgiters.b = (self.resgiters.b >>1 )| ((if self.resgiters.f.carry {1} else {0}) as u8)<<7;
                            self.resgiters.f.zero = self.resgiters.b ==0;
                            self.resgiters.f.carry = if n ==0 {false} else {true};
                            self.resgiters.f.half_carry = false;
                            self.resgiters.f.subtract =false; 
                        },
                        Target8::C => {
                            let n = self.resgiters.c & 0x01;
                            self.resgiters.c = (self.resgiters.c >>1 )| ((if self.resgiters.f.carry {1} else {0}) as u8)<<7;
                            self.resgiters.f.zero = self.resgiters.c ==0;
                            self.resgiters.f.carry = if n ==0 {false} else {true};
                            self.resgiters.f.half_carry = false;
                            self.resgiters.f.subtract =false;
                        },
                        Target8::D => {
                            let n = self.resgiters.d & 0x01;
                            self.resgiters.d = (self.resgiters.d >>1) | ((if self.resgiters.f.carry {1} else {0}) as u8)<<7;
                            self.resgiters.f.zero = self.resgiters.d ==0;
                            self.resgiters.f.carry = if n ==0 {false} else {true};
                            self.resgiters.f.half_carry = false;
                            self.resgiters.f.subtract =false; 
                        }, 
                        Target8::H => {
                            let n = self.resgiters.h & 0x01;
                            self.resgiters.h = (self.resgiters.h >>1 )| ((if self.resgiters.f.carry {1} else {0}) as u8)<<7;
                            self.resgiters.f.zero = self.resgiters.h ==0;
                            self.resgiters.f.carry = if n ==0 {false} else {true};
                            self.resgiters.f.half_carry = false;
                            self.resgiters.f.subtract =false;
                        },
                        Target8::L => {
                            let n = self.resgiters.l & 0x01;
                            self.resgiters.l = (self.resgiters.l >>1) | ((if self.resgiters.f.carry {1} else {0}) as u8)<<7;
                            self.resgiters.f.zero = self.resgiters.l ==0;
                            self.resgiters.f.carry = if n ==0 {false} else {true};
                            self.resgiters.f.half_carry = false;
                            self.resgiters.f.subtract =false; 
                        }, 
                        Target8::E => {
                            let n = self.resgiters.e & 0x01;
                            self.resgiters.e = (self.resgiters.e >>1) | ((if self.resgiters.f.carry {1} else {0}) as u8)<<7;
                            self.resgiters.f.zero = self.resgiters.e ==0;
                            self.resgiters.f.carry = if n ==0 {false} else {true};
                            self.resgiters.f.half_carry = false;
                            self.resgiters.f.subtract =false;
                        },
                        Target8::HL => {
                            let value = self.bus.read_byte(self.resgiters.get_hl());
                            let n = value & 0x01;
                            self.bus.write_byte(self.resgiters.get_hl(), value >>1 | ((if self.resgiters.f.carry {1} else {0}) as u8)<<7);
                            self.resgiters.f.zero = self.bus.read_byte(self.resgiters.get_hl()) ==0;
                            self.resgiters.f.carry = if n ==0 {false} else {true};
                            self.resgiters.f.half_carry = false;
                            self.resgiters.f.subtract =false; 
                        },
                        _=> panic!("RR target")
                    };
                    self.pc.wrapping_add(1)
                },
                Instructions::SLA(target) => {

                    match target {
                        Target8::A => {
                            let n = self.resgiters.a & 0x80;
                            self.resgiters.a = self.resgiters.a <<1 ;
                            self.resgiters.f.zero = self.resgiters.a ==0;
                            self.resgiters.f.carry = if n ==0 {false} else {true};
                            self.resgiters.f.half_carry = false;
                            self.resgiters.f.subtract =false;
                        },
                        Target8::B => {
                            let n = self.resgiters.b & 0x80;
                            self.resgiters.b = self.resgiters.b <<1;
                            self.resgiters.f.zero = self.resgiters.b ==0;
                            self.resgiters.f.carry = if n ==0 {false} else {true};
                            self.resgiters.f.half_carry = false;
                            self.resgiters.f.subtract =false; 
                        },
                        Target8::C => {
                            let n = self.resgiters.c & 0x80;
                            self.resgiters.c = self.resgiters.c <<1 ;
                            self.resgiters.f.zero = self.resgiters.c ==0;
                            self.resgiters.f.carry = if n ==0 {false} else {true};
                            self.resgiters.f.half_carry = false;
                            self.resgiters.f.subtract =false;
                        },
                        Target8::D => {
                            let n = self.resgiters.d & 0x80;
                            self.resgiters.d = self.resgiters.d <<1 ;
                            self.resgiters.f.zero = self.resgiters.d ==0;
                            self.resgiters.f.carry = if n ==0 {false} else {true};
                            self.resgiters.f.half_carry = false;
                            self.resgiters.f.subtract =false; 
                        }, 
                        Target8::H => {
                            let n = self.resgiters.h & 0x80;
                            self.resgiters.h = self.resgiters.h <<1 ;
                            self.resgiters.f.zero = self.resgiters.h ==0;
                            self.resgiters.f.carry = if n ==0 {false} else {true};
                            self.resgiters.f.half_carry = false;
                            self.resgiters.f.subtract =false;
                        },
                        Target8::L => {
                            let n = self.resgiters.l & 0x80;
                            self.resgiters.l = self.resgiters.l <<1 ;
                            self.resgiters.f.zero = self.resgiters.l ==0;
                            self.resgiters.f.carry = if n ==0 {false} else {true};
                            self.resgiters.f.half_carry = false;
                            self.resgiters.f.subtract =false; 
                        }, 
                        Target8::E => {
                            let n = self.resgiters.e & 0x80;
                            self.resgiters.e = self.resgiters.e <<1 ;
                            self.resgiters.f.zero = self.resgiters.e ==0;
                            self.resgiters.f.carry = if n ==0 {false} else {true};
                            self.resgiters.f.half_carry = false;
                            self.resgiters.f.subtract =false;
                        },
                        Target8::HL => {
                            let value = self.bus.read_byte(self.resgiters.get_hl());
                            let n = value & 0x80;
                            self.bus.write_byte(self.resgiters.get_hl(), value <<1 );
                            self.resgiters.f.zero = self.bus.read_byte(self.resgiters.get_hl()) ==0;
                            self.resgiters.f.carry = if n ==0 {false} else {true};
                            self.resgiters.f.half_carry = false;
                            self.resgiters.f.subtract =false; 
                        },
                        _=> panic!("SLA target")
                    };
                    self.pc.wrapping_add(1)
                },
                Instructions::SRA(target) => {
                       
                    match target {
                        Target8::A => {
                            let n = self.resgiters.a & 0x80;
                            self.resgiters.a = (self.resgiters.a >>1) | n <<7;
                            self.resgiters.f.zero = self.resgiters.a ==0;
                            self.resgiters.f.carry = if n ==0 {false} else {true};
                            self.resgiters.f.half_carry = false;
                            self.resgiters.f.subtract =false;

                        },
                        Target8::B => {
                            let n = self.resgiters.b & 0x80;
                            self.resgiters.b = (self.resgiters.b >>1 )| n<<7;
                            self.resgiters.f.zero = self.resgiters.b ==0;
                            self.resgiters.f.carry = if n ==0 {false} else {true};
                            self.resgiters.f.half_carry = false;
                            self.resgiters.f.subtract =false; 
                        },
                        Target8::C => {
                            let n = self.resgiters.c & 0x80;
                            self.resgiters.c = (self.resgiters.c >>1 )| n<<7;
                            self.resgiters.f.zero = self.resgiters.c ==0;
                            self.resgiters.f.carry = if n ==0 {false} else {true};
                            self.resgiters.f.half_carry = false;
                            self.resgiters.f.subtract =false;
                        },
                        Target8::D => {
                            let n = self.resgiters.d & 0x80;
                            self.resgiters.d = (self.resgiters.d >>1) | n<<7;
                            self.resgiters.f.zero = self.resgiters.d ==0;
                            self.resgiters.f.carry = if n ==0 {false} else {true};
                            self.resgiters.f.half_carry = false;
                            self.resgiters.f.subtract =false; 
                        }, 
                        Target8::H => {
                            let n = self.resgiters.h & 0x80;
                            self.resgiters.h = (self.resgiters.h >>1 )| n<<7;
                            self.resgiters.f.zero = self.resgiters.h ==0;
                            self.resgiters.f.carry = if n ==0 {false} else {true};
                            self.resgiters.f.half_carry = false;
                            self.resgiters.f.subtract =false;
                        },
                        Target8::L => {
                            let n = self.resgiters.l & 0x80;
                            self.resgiters.l = (self.resgiters.l >>1) | n<<7;
                            self.resgiters.f.zero = self.resgiters.l ==0;
                            self.resgiters.f.carry = if n ==0 {false} else {true};
                            self.resgiters.f.half_carry = false;
                            self.resgiters.f.subtract =false; 
                        }, 
                        Target8::E => {
                            let n = self.resgiters.e & 0x80;
                            self.resgiters.e = (self.resgiters.e >>1) | n<<7;
                            self.resgiters.f.zero = self.resgiters.e ==0;
                            self.resgiters.f.carry = if n ==0 {false} else {true};
                            self.resgiters.f.half_carry = false;
                            self.resgiters.f.subtract =false;
                        },
                        Target8::HL => {
                            let value = self.bus.read_byte(self.resgiters.get_hl());
                            let n = value & 0x80;
                            self.bus.write_byte(self.resgiters.get_hl(), value >>1 | n<<7);
                            self.resgiters.f.zero = self.bus.read_byte(self.resgiters.get_hl()) ==0;
                            self.resgiters.f.carry = if n ==0 {false} else {true};
                            self.resgiters.f.half_carry = false;
                            self.resgiters.f.subtract =false; 
                        },
                        _=> panic!("RRC target")
                    };
                    self.pc.wrapping_add(1)
                }
                Instructions::SRL(target) => {
                       
                    match target {
                        Target8::A => {
                            let n = self.resgiters.a & 0x80;
                            self.resgiters.a = self.resgiters.a >>1 ;
                            self.resgiters.f.zero = self.resgiters.a ==0;
                            self.resgiters.f.carry = if n ==0 {false} else {true};
                            self.resgiters.f.half_carry = false;
                            self.resgiters.f.subtract =false;

                        },
                        Target8::B => {
                            let n = self.resgiters.b & 0x80;
                            self.resgiters.b = self.resgiters.b >>1;
                            self.resgiters.f.zero = self.resgiters.b ==0;
                            self.resgiters.f.carry = if n ==0 {false} else {true};
                            self.resgiters.f.half_carry = false;
                            self.resgiters.f.subtract =false; 
                        },
                        Target8::C => {
                            let n = self.resgiters.c & 0x80;
                            self.resgiters.c = self.resgiters.c >>1 ;
                            self.resgiters.f.zero = self.resgiters.c ==0;
                            self.resgiters.f.carry = if n ==0 {false} else {true};
                            self.resgiters.f.half_carry = false;
                            self.resgiters.f.subtract =false;
                        },
                        Target8::D => {
                            let n = self.resgiters.d & 0x80;
                            self.resgiters.d = self.resgiters.d >>1;
                            self.resgiters.f.zero = self.resgiters.d ==0;
                            self.resgiters.f.carry = if n ==0 {false} else {true};
                            self.resgiters.f.half_carry = false;
                            self.resgiters.f.subtract =false; 
                        }, 
                        Target8::H => {
                            let n = self.resgiters.h & 0x80;
                            self.resgiters.h = self.resgiters.h >>1 ;
                            self.resgiters.f.zero = self.resgiters.h ==0;
                            self.resgiters.f.carry = if n ==0 {false} else {true};
                            self.resgiters.f.half_carry = false;
                            self.resgiters.f.subtract =false;
                        },
                        Target8::L => {
                            let n = self.resgiters.l & 0x80;
                            self.resgiters.l = self.resgiters.l >>1;
                            self.resgiters.f.zero = self.resgiters.l ==0;
                            self.resgiters.f.carry = if n ==0 {false} else {true};
                            self.resgiters.f.half_carry = false;
                            self.resgiters.f.subtract =false; 
                        }, 
                        Target8::E => {
                            let n = self.resgiters.e & 0x80;
                            self.resgiters.e = self.resgiters.e >>1;
                            self.resgiters.f.zero = self.resgiters.e ==0;
                            self.resgiters.f.carry = if n ==0 {false} else {true};
                            self.resgiters.f.half_carry = false;
                            self.resgiters.f.subtract =false;
                        },
                        Target8::HL => {
                            let value = self.bus.read_byte(self.resgiters.get_hl());
                            let n = value & 0x80;
                            self.bus.write_byte(self.resgiters.get_hl(), value >>1 );
                            self.resgiters.f.zero = self.bus.read_byte(self.resgiters.get_hl()) ==0;
                            self.resgiters.f.carry = if n ==0 {false} else {true};
                            self.resgiters.f.half_carry = false;
                            self.resgiters.f.subtract =false; 
                        },
                        _=> panic!("RRC target")
                    };
                    self.pc.wrapping_add(1)
                }
                Instructions::SWAP(target) => {
                    match target {
                        Target8::A => self.resgiters.a = self.swap(self.resgiters.a),
                        Target8::B => self.resgiters.a = self.swap(self.resgiters.b),
                        Target8::C => self.resgiters.a = self.swap(self.resgiters.c),
                        Target8::D => self.resgiters.a = self.swap(self.resgiters.d),
                        Target8::E => self.resgiters.a = self.swap(self.resgiters.e),
                        Target8::H => self.resgiters.a = self.swap(self.resgiters.h),
                        Target8::L => self.resgiters.a = self.swap(self.resgiters.l),
                        Target8::HL => {
                            let value = self.bus.read_byte(self.resgiters.get_hl());
                            self.bus.write_byte(self.resgiters.get_hl(),value);
                        },
                        _=> panic!("SWAP target"),
                    };

                    self.pc.wrapping_add(1)
                }
                Instructions::BIT(n,target ) => {
                    if n<8 {
                        match target {
                            Target8::A => self.bit(self.resgiters.a, n),
                            Target8::B => self.bit(self.resgiters.b, n),
                            Target8::C => self.bit(self.resgiters.c, n),
                            Target8::D => self.bit(self.resgiters.d, n),
                            Target8::E => self.bit(self.resgiters.e, n),
                            Target8::H => self.bit(self.resgiters.h, n),
                            Target8::L => self.bit(self.resgiters.l, n),
                            Target8::HL => self.bit(self.bus.read_byte(self.resgiters.get_hl()), n),
                            _=> panic!("BIT target"),
                        };

                    } else {
                        panic!("BIT n")
                    }

                    self.pc.wrapping_add(1)
                },   
                Instructions::SET(n,target ) => {
                    if n<8 {
                        match target {
                            Target8::A => self.set(self.resgiters.a, n),
                            Target8::B => self.set(self.resgiters.b, n),
                            Target8::C => self.set(self.resgiters.c, n),
                            Target8::D => self.set(self.resgiters.d, n),
                            Target8::E => self.set(self.resgiters.e, n),
                            Target8::H => self.set(self.resgiters.h, n),
                            Target8::L => self.set(self.resgiters.l, n),
                            Target8::HL => self.set(self.bus.read_byte(self.resgiters.get_hl()), n),
                            _=> panic!("BIT target"),
                        };

                    } else {
                        panic!("BIT n")
                    }

                    self.pc.wrapping_add(1)
                },   
                Instructions::RES(n,target ) => {
                    if n<8 {
                        match target {
                            Target8::A => self.res(self.resgiters.a, n),
                            Target8::B => self.res(self.resgiters.b, n),
                            Target8::C => self.res(self.resgiters.c, n),
                            Target8::D => self.res(self.resgiters.d, n),
                            Target8::E => self.res(self.resgiters.e, n),
                            Target8::H => self.res(self.resgiters.h, n),
                            Target8::L => self.res(self.resgiters.l, n),
                            Target8::HL => self.res(self.bus.read_byte(self.resgiters.get_hl()), n),
                            _=> panic!("BIT target"),
                        };

                    } else {
                        panic!("BIT n")
                    }

                    self.pc.wrapping_add(1)
                },
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

    pub fn add_hl(&mut self, value:u16) {
        let (new, overflow) = self.resgiters.get_hl().overflowing_add(value);

        self.resgiters.f.zero = new==0;
        self.resgiters.f.carry = overflow;
        let (_, overflow8) = ((self.resgiters.get_hl() & 0xFF) as u8).overflowing_add((value & 0xFF)as u8);
        self.resgiters.f.half_carry= overflow8;

        self.resgiters.set_hl(new);
        
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

    pub fn jr(&self, condition: bool) ->u16{
        if condition {
            let n = self.bus.read_byte(self.pc + 1) as i8;

            self.pc.wrapping_add_signed(n as i16)
        } else {
            self.pc.wrapping_add(2)
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
        let lower = self.bus.read_byte(self.pc + 2);
        let higher = self.bus.read_byte(self.pc + 3);
        let next_program = (lower as u16) & ((higher as u16)<<8);
        if jump {
            self.push(self.pc + 1);
            next_program
        } else {
            self.pc.wrapping_add(3)
        }

    }

    pub fn ret(&mut self, jump: bool) -> u16{
        if jump {
            self.pop() 
        } else {
            self.pc.wrapping_add(1)           
        }

    }

    pub fn swap(&self, value:u8)-> u8 {
        ((value & 0xF0)>>4)|((value & 0x0F)<<4)
    }

    pub fn bit(&mut self, reg:u8, n:u8) {
        let nth = (reg >>n) & 0x01;
        self.resgiters.f.zero = nth==0;
        self.resgiters.f.half_carry = true;
        self.resgiters.f.subtract = false;
    }
    
    pub fn set(&mut self, reg:u8, n:u8)-> u8 {
        reg | (0x01<<n)
        
    } 
    
    pub fn res(&mut self, reg:u8, n:u8)-> u8 {
        reg & !(0x01<<n)
        
    }
    
   

}

