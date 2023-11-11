#[derive(Debug)]
pub enum JoypadKey {
    Right,
    Left,
    Up,
    Down,
    A,
    B,
    Select,
    Start,
}

impl JoypadKey {
    //Associe les valeurs avec les touches
    pub fn correspondance(&self) -> u8 {
        match *self {
            JoypadKey::Right | JoypadKey::A => 0x01,
            JoypadKey::Left | JoypadKey::B => 0x02,
            JoypadKey::Up | JoypadKey::Select => 0x04,
            JoypadKey::Down | JoypadKey::Start => 0x08,
        }
    }
}

pub struct Keypad {
    pub p14: u8,
    pub p15: u8,
    pub mask: u8,
    pub interrupted: bool,
}

impl Keypad {
    pub fn new() -> Self {
        Keypad { 
            p14: 0x0F,      //registre p14, état des boutons de direction 
            p15: 0x0F,      //regirstre p15, état des boutons A et B
            mask: 0x30,     //contrôle quelles interruptions du contrôleur de jeu sont autorisées ou désactivées
            interrupted: false,     //interruptions lorsqu'un bouton est enfoncé ou relâché
        }
    }

    pub fn key_down(&mut self, key: JoypadKey) {
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

    pub fn key_up(&mut self, key: JoypadKey) {
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

    //Lit l'état des interruptions
    pub fn read_interrupt(&mut self) -> u8 {
        if self.interrupted {
            self.interrupted = false;
            return 0x10; //Le bit 4 indique une interruption du joypad
        }
        0
    }
}