use std::fmt;

pub struct Flag {
    pub val: bool,
}

impl fmt::Display for Flags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "S:{} Z:{} K:{} A:{} 0:{} P:{} V:{} C:{}", 
               self.s.val as u8, 
               self.z.val as u8, 
               self.k.val as u8,
               self.ac.val as u8, 
               0,
               self.p.val as u8, 
               self.v.val as u8,
               self.c.val as u8)
    }
}

pub struct Flags {
    pub s: Flag,
    pub z: Flag,
    pub k: Flag,
    pub ac: Flag,
    pub p: Flag,
    pub v: Flag,
    pub c: Flag,
}

impl Flags {
    pub fn new() -> Self {
        Flags{
            s: Flag{val: false},
            z: Flag { val: false },
            k: Flag { val: false },
            ac: Flag { val: false },
            p: Flag { val: false },
            v: Flag { val: false },
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
        f |= (self.k.val as u8) << 5;
        f |= (self.ac.val as u8) << 4;
        f |= 0 << 3;
        f |= (self.p.val as u8) << 2;
        f |= (self.v.val as u8) << 1;
        f |= self.c.val as u8;
        f
    }

    pub fn unpack(&mut self, f: u8) {
        self.s.val = (f & 0x80) != 0;
        self.z.val = (f & 0x40) != 0;
        self.k.val = (f & 0x20) != 0;
        self.ac.val = (f & 0x10) != 0;
        self.p.val = (f & 0x04) != 0;
        self.v.val = (f & 0x02) != 0;
        self.c.val = (f & 0x01) != 0;
    }
}