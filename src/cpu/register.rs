//Constantes pour les positions binaires des flags
//1000 0000
pub const F_ZERO_BYTE_POSITION : u8 = 7;
//0100 0000
pub const F_SUBSTRACT_BYTE_POSITION : u8 = 6;
//0010 0000
pub const F_HALF_CARRY_BYTE_POSITION : u8 = 5;
//0001 000
pub const F_CARRY_BYTE_POSITION : u8 = 4;


pub struct Resgisters {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    //Le register f est utilisé pour les flags.
    /*
    Les 4 derniers bits sont toujours à zéro.
    Les 4 premiers sont utilisés comme des flags.
    Bit 7 : "zero"
    Bit 6 : "substraction"
    Bit 5 : "half carry"
    Bit 4 : "carry"
     */
    f: u8,
    h: u8,
    l: u8,
}

impl Resgisters {
    pub fn get_af(&self) -> u16 {
        //On récupère les bits du register a en u16.
        //Comme ils sont sur les 8 derniers bits, on les décale sur les 8 premiers.
        //On ajoute les bits du register f à la fin du u16.
        (self.a as u16) << 8
        | self.f as u16

    }

    pub fn set_af(&mut self, value: u16) {
        //On récupère les 8 premiers bits de value.
        //Comme ils doivent être stockés dans le register a,
        //il faut les décaler sur les 8 derniers bits pour la convertion en u8.
        self.a = ((value & 0xFF00) >> 8) as u8;

        //On récupère les 8 derniers bits de value et on les stocke dans f
        self.f = (value & 0xFF) as u8;

    }

    pub fn get_bc(&self)-> u16 {
        (self.b as u16) << 8
        | self.c as u16
    }
    
    pub fn set_bc(&mut self, value: u16) {
        self.b = ((value & 0xFF00) >> 8) as u8;
        self.c = (value & 0xFF) as u8;
    }

    pub fn get_de(&self) -> u16 {
        (self.d as u16) << 8 
        | self.e as u16

    }

    pub fn set_de(&mut self, value: u16) {
        self.d = ((value & 0xFF00) >> 8) as u8;
        self.e = (value & 0xFF) as u8;
    }

    pub fn get_hl(&self) -> u16 {
        (self.h as u16) << 8 
        | self.e as u16

    }

    pub fn set_hl(&mut self, value: u16) {
        self.h = ((value & 0xFF00) >> 8) as u8;
        self.l = (value & 0xFF) as u8;
    }

}

//Structure pour le register f
pub struct FlagsRegister {
    zero: bool,
    subtract: bool,
    half_carry: bool,
    carry: bool
}

impl std::convert::From<FlagsRegister> for u8 {
    fn from(flags: FlagsRegister)-> u8 {
        (if flags.zero {1} else {0}) << F_ZERO_BYTE_POSITION
        | (if flags.subtract {1} else {0}) << F_SUBSTRACT_BYTE_POSITION
        | (if flags.half_carry {1} else {0}) << F_HALF_CARRY_BYTE_POSITION
        | (if flags.carry {1} else {0}) << F_CARRY_BYTE_POSITION
    }

}

impl std::convert::From<u8> for FlagsRegister {
    fn from(bits: u8) -> FlagsRegister {
        let zero = (bits & 8) != 0;
        let subtract = (bits & 4) != 0;
        let half_carry = (bits & 2) != 0;
        let carry = (bits & 1) != 0;

        FlagsRegister {
            zero,
            subtract,
            half_carry,
            carry
        }

    }
    
}