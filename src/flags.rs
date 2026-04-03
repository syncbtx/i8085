pub struct Flag {
    pub val: bool,
}

pub struct Flags {
    pub s: Flag,
    pub z: Flag,
    pub ac: Flag,
    pub p: Flag,
    pub c: Flag,
}

impl Flags {
    pub fn new() -> Self {
        Flags{
            s: Flag{val: false},
            z: Flag { val: false },
            ac: Flag { val: false },
            p: Flag { val: false },
            c: Flag { val: false },
        }
    }
    
    pub fn reset(&mut self){
        *self = Self::new();
    }

    pub fn pack(&self) -> u8 {
        let mut f = 0;
        f |= (self.s.val as u8) << 7;
        f |= (self.z.val as u8) << 6;
        f |= (self.ac.val as u8) << 4;
        f |= (self.p.val as u8) << 2;
        f |= 1 << 1;
        f |= self.c.val as u8;
        f
    }

    pub fn unpack(&mut self, f: u8) {
        self.s.val = (f & 0x80) != 0;
        self.z.val = (f & 0x40) != 0;
        self.ac.val = (f & 0x10) != 0;
        self.p.val = (f & 0x04) != 0;
        self.c.val = (f & 0x01) != 0;
    }
}