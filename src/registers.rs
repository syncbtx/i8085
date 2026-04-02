pub struct Reg8{
    pub val: u8,
    pub is_dirty: bool,
}

impl Reg8{
    pub fn new() -> Self{
        Reg8{
            val: 0x00,
            is_dirty: false,
        }
    }

    pub fn reset(&mut self){
        self.val = 0x00;
        self.is_dirty = false
    }
}

pub struct Reg16{
    pub val: u16,
    pub is_dirty: bool,
}

impl Reg16{
    pub fn new() -> Self{
        Reg16{
            val: 0x00,
            is_dirty: false,
        }
    }

    pub fn reset(&mut self){
        self.val = 0x00;
        self.is_dirty = false
    }
}

pub struct Registers{
    pub a: Reg8,
    pub b: Reg8,
    pub c: Reg8,
    pub d: Reg8,
    pub e: Reg8,
    pub h: Reg8,
    pub l: Reg8,
    pub w: Reg8,
    pub z: Reg8,
    pub pc: Reg16,
    pub sp: Reg16,
}

impl Registers{
    pub fn new() -> Self{
        Registers{
            a: Reg8::new(),
            b: Reg8::new(),
            c: Reg8::new(),
            d: Reg8::new(),
            e: Reg8::new(),
            h: Reg8::new(),
            l: Reg8::new(),
            w: Reg8::new(),
            z: Reg8::new(),
            pc: Reg16::new(),
            sp: Reg16::new(),
        }
    }

    pub fn reset(&mut self){
        *self = Self::new();
    }

    #[inline]
    pub fn get_bc(&self) -> u16{
        ((self.b.val as u16) << 8) | self.c.val as u16
    }

    #[inline]
    pub fn get_de(&self) -> u16{
        ((self.d.val as u16) << 8) | self.e.val as u16
    }

    #[inline]
    pub fn get_hl(&self) -> u16{
        ((self.h.val as u16) << 8) | self.l.val as u16
    }

    #[inline]
    pub fn set_bc(&mut self, val: u16){
        self.b.val = (val >> 8) as u8;
        self.c.val = (val & 0x00FF) as u8;
    }

    #[inline]
    pub fn set_de(&mut self, val: u16){
        self.d.val = (val >> 8) as u8;
        self.e.val = (val & 0x00FF) as u8;
    }

    #[inline]
    pub fn set_hl(&mut self, val: u16){
        self.h.val = (val >> 8) as u8;
        self.l.val = (val & 0x00FF) as u8;
    }
}