use super::register::Resgisters;

pub struct CPU {
    resgiters: Resgisters,
}

pub enum Instructions {
    ADD(TargetResgister),
}

pub enum TargetResgister{
    A,B,C,D,E,H,L,
}

impl CPU {

    pub fn execute(&mut self, instruction: Instructions){
        match instruction {
            Instructions::ADD(target)=>{
                match target {
                    TargetResgister::A=> {
                        self.resgiters.a = self.add(self.resgiters.a); 
                    }
                    TargetResgister::B=> {
                        let value = self.resgiters.b;
                        self.resgiters.a = self.add(value);
                    }
                    TargetResgister::C=> {
                        let value = self.resgiters.c;
                        self.resgiters.a = self.add(value);
                    }
                    TargetResgister::D=> {
                        let value = self.resgiters.d;
                        self.resgiters.a = self.add(value);
                    }
                    TargetResgister::E=> {
                        let value = self.resgiters.e;
                        self.resgiters.a = self.add(value);                    
                    }
                    TargetResgister::H=> {
                        let value = self.resgiters.h;
                        self.resgiters.a = self.add(value);                    
                    }
                    TargetResgister::L=> {
                        let value = self.resgiters.l;
                        self.resgiters.a = self.add(value);
                    }

                }
            }


        }
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