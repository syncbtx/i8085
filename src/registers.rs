use std::fmt;
use crate::utils::{DataFormat, ToFormatted};

pub struct Reg8 {
    pub val: u8,
    pub is_dirty: bool,
}

impl fmt::Debug for Reg8 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#04X}", self.val)
    }
}

impl ToFormatted for Reg8 {
    fn format(&self, format: DataFormat) -> String {
        self.val.format(format)
    }
}

impl Reg8 {
    pub fn new() -> Self {
        Reg8 {
            val: 0x00,
            is_dirty: false,
        }
    }

    pub fn reset(&mut self) {
        self.val = 0x00;
        self.is_dirty = false
    }
}

pub struct Reg16 {
    pub val: u16,
    pub is_dirty: bool,
}

impl fmt::Debug for Reg16 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#06X}", self.val)
    }
}

impl ToFormatted for Reg16 {
    fn format(&self, format: DataFormat) -> String {
        self.val.format(format)
    }
}

impl Reg16 {
    pub fn new() -> Self {
        Reg16 {
            val: 0x00,
            is_dirty: false,
        }
    }

    pub fn reset(&mut self) {
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

impl fmt::Display for Registers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "┌──────────────────────── CPU Registers ─────────────────────────┐")?;
        writeln!(f, "│  A: {:?}    B: {:?}    C: {:?}    D: {:?}    E: {:?}           │", self.a, self.b, self.c, self.d, self.e)?;
        writeln!(f, "│  H: {:?}    L: {:?}    W: {:?}    Z: {:?}                      │", self.h, self.l, self.w, self.z)?;
        writeln!(f, "├────────────────────────────────────────────────────────────────┤")?;
        writeln!(f, "│  PC: {:?}    SP: {:?}  BC: {:#06X}    DE: {:#06X}  HL: {:#06X}│", 
            self.pc, self.sp, self.get_bc(), self.get_de(), self.get_hl())?;
        write!(f,   "└────────────────────────────────────────────────────────────────┘")
    }
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
    pub fn get_hl(&self) -> u16{((self.h.val as u16) << 8) | self.l.val as u16 }
    pub fn get_wz(&self) -> u16{((self.w.val as u16) << 8) | self.z.val as u16 }

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

    #[inline]
    pub fn set_wz(&mut self, val: u16){
        self.w.val = (val >> 8) as u8;
        self.z.val = (val & 0x00FF) as u8;
    }

    pub fn read_reg(&self, idx: u8) -> u8 {
        match idx {
            0 => self.b.val,
            1 => self.c.val,
            2 => self.d.val,
            3 => self.e.val,
            4 => self.h.val,
            5 => self.l.val,
            7 => self.a.val,
            8 => self.w.val,
            9 => self.z.val,
            _ => 0,
        }
    }

    pub fn write_reg(&mut self, idx: u8, val: u8) {
        match idx {
            0 => self.b.val = val,
            1 => self.c.val = val,
            2 => self.d.val = val,
            3 => self.e.val = val,
            4 => self.h.val = val,
            5 => self.l.val = val,
            7 => self.a.val = val,
            8 => self.w.val = val,
            9 => self.z.val = val,
            _ => (),
        }
    }
}