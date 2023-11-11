pub enum JoypadKey{
    Left,
    Right,
    Up,
    Down,
    A,
    B,
    Select,
    Start,
}

pub struct Keypad{
    pub p14: u8,
    pub p15: u8,
    pub mask: u8,
    pub interrupted: bool,
}

impl Keypad{
    pub fn new() -> Self{
        Keypad { 
            p14: 0x0F,      //registre p14, état des boutons de direction 
            p15: 0x0F,      //regirstre p15, état des boutons A et B
            mask: 0x30,     //contrôle quelles interruptions du contrôleur de jeu sont autorisées ou désactivées
            interrupted: false,     //interruptions lorsqu'un bouton est enfoncé ou relâché
        }
    }

    pub fn keyDown(&mut self, key: JoypadKey){
        match key {
            JoypadKey::Right => self.p14 &= !(1 << 0), //permet de mettre le bit à la position 0 à 0
            JoypadKey::Left => self.p14 &= !(1 << 1),
            JoypadKey::Up => self.p14 &= !(1 << 2),
            JoypadKey::Down => self.p14 &= !(1 << 3),
            JoypadKey::A => self.p15 &= !(1 << 4),
            JoypadKey::B => self.p15 &= !(1 << 5),
            JoypadKey::Select => self.p15 &= !(1 << 6),
            JoypadKey::Start => self.p15 &= !(1 << 7),
        }
        self.interrupted = true;
    }

    pub fn keyUp(&mut self, key: JoypadKey){
        match key {
            JoypadKey::Right => self.p14 |= 1 << 0, //met un 1 à la position choisie
            JoypadKey::Left => self.p14 |= 1 << 1,
            JoypadKey::Up => self.p14 |= 1 << 2,
            JoypadKey::Down => self.p14 |= 1 << 3,
            JoypadKey::A => self.p15 |= 1 << 4,
            JoypadKey::B => self.p15 |= 1 << 5,
            JoypadKey::Select => self.p15 |= 1 << 6,
            JoypadKey::Start => self.p15 |= 1 << 7,
        }
        self.interrupted = true;
    }

    /*pub fn readKey(&self) -> u8 {
        //Définir les masques pour chaque touche
        const RIGHT_MASK: u8 = 0x01;    //le bit 0 du registre p14
        const LEFT_MASK: u8 = 0x02;     //le bit 1 du registre p14
        const UP_MASK: u8 = 0x04;       //le bit 2 du registre p14
        const DOWN_MASK: u8 = 0x08;     //le bit 3 du registre p14
        const A_MASK: u8 = 0x10;        //le bit 4 du registre p15
        const B_MASK: u8 = 0x20;        //le bit 5 du registre p15
        const SELECT_MASK: u8 = 0x40;   //le bit 6 du registre p15
        const START_MASK: u8 = 0x80;    //le bit 7 du registre p15

        //Lire l'état actuel des touches en utilisant les registres p14 et p15
        let mut keys = 0xFF; //Initialise les touches à l'état relâché -> 1

        //Si p14 est enfoncé met le bit correspondant à 0
        if (self.p14 & RIGHT_MASK) == 0 {
            keys &= !0x01; // Met le bit Right à 0
        }
        if (self.p14 & LEFT_MASK) == 0 {
            keys &= !0x02; // Met le bit Left à 0
        }
        if (self.p14 & UP_MASK) == 0 {
            keys &= !0x04; // Met le bit Up à 0
        }
        if (self.p14 & DOWN_MASK) == 0 {
            keys &= !0x08; // Met le bit Down à 0
        }

        //Si p15 est enfoncé met le bit correspondant à 0
        if (self.p15 & A_MASK) == 0 {
            keys &= !0x10; // Met le bit A à 0
        }
        if (self.p15 & B_MASK) == 0 {
            keys &= !0x20; // Met le bit B à 0
        }
        if (self.p15 & SELECT_MASK) == 0 {
            keys &= !0x40; // Met le bit Select à 0
        }
        if (self.p15 & START_MASK) == 0 {
            keys &= !0x80; // Met le bit Start à 0
        }

        //Retourne l'état des touches
        keys
    }*/
}

