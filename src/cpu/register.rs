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
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    //Le register f est utilisé pour les flags.
    /*
    Les 4 derniers bits sont toujours à zéro.
    Les 4 premiers sont utilisés comme des flags.
    Bit 7 : "zero"
    Bit 6 : "substraction"
    Bit 5 : "half carry"
    Bit 4 : "carry"
     */
    pub f: FlagsRegister,
    pub h: u8,
    pub l: u8,
}

impl Resgisters {
    pub fn new()-> Resgisters {
        Resgisters { 
            a: 0x01, 
            b: 0x00, 
            c: 0x13, 
            d: 0x00, 
            e: 0xD8, 
            f: FlagsRegister { 
                zero: true, 
                subtract: false, 
                half_carry: true, 
                carry: true 
            }, 
            h: 0x01, 
            l: 0x4D, 
        }
    }

    pub fn get_af(&self) -> u16 {
        //On récupère les bits du register a en u16.
        //Comme ils sont sur les 8 derniers bits, on les décale sur les 8 premiers.
        //On ajoute les bits du register f à la fin du u16.
        (self.a as u16) << 8
        | u8::from(self.f) as u16

    }

    pub fn set_af(&mut self, value: u16) {
        //On récupère les 8 premiers bits de value.
        //Comme ils doivent être stockés dans le register a,
        //il faut les décaler sur les 8 derniers bits pour la convertion en u8.
        self.a = ((value & 0xFF00) >> 8) as u8;

        //On récupère les 8 derniers bits de value et on les stocke dans f
        self.f = FlagsRegister::from((value & 0xFF) as u8);

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
#[derive(Copy, Clone)]
pub struct FlagsRegister {
    pub zero: bool,
    pub subtract: bool,
    pub half_carry: bool,
    pub carry: bool
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