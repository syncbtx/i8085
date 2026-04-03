use crate::registers::Registers;
use crate::pins::Pins;
use crate::flags::Flags;
use crate::alu::Alu;
use crate::cu::ControlUnit;

use std::fmt;

pub struct CPU {
    pub regs: Registers,
    pub pins: Pins,
    pub flags: Flags,
    pub alu: Alu,
    pub control: ControlUnit,
}

impl fmt::Display for CPU {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.regs)?;
        write!(f, "Flags: {}", self.flags)
    }
}

impl CPU {
    pub fn get_psw(&self) -> u16 {
        ((self.regs.a.val as u16) << 8) | (self.flags.pack() as u16)
    }

    pub fn set_psw(&mut self, val: u16) {
        self.regs.a.val = (val >> 8) as u8;
        self.flags.unpack(val as u8);
    }

    pub fn new() -> Self {
        CPU {
            regs: Registers::new(),
            pins: Pins::new(),
            flags: Flags::new(),
            alu: Alu::new(),
            control: ControlUnit::new(),
        }
    }

    pub fn reset(&mut self) {
        self.regs.reset();
        self.pins.reset();
        self.flags.reset();
        self.alu.reset();
        self.control.reset();
    }

    pub fn tick(&mut self, bus: &mut crate::bus::SystemBus) {
        if !self.pins.io_m && !self.pins.s1 && !self.pins.s0 {
            bus.sync(&mut self.pins);
            return;
        }

        ControlUnit::execute_pla(self);
        bus.sync(&mut self.pins);

        if self.pins.ready {
            self.control.advance_clock();
        }
    }

    // ----- Bus cycle helpers -----
    pub(crate) fn fetch_t1(&mut self) {
        let pc = self.regs.pc.val;
        self.pins.a = (pc >> 8) as u8;
        self.pins.ad = (pc & 0xFF) as u8;
        self.pins.ale = true;
        self.pins.io_m = false;
        self.pins.s0 = true;
        self.pins.s1 = true;
        self.pins.rd = true;
        self.pins.wr = true;
    }

    pub(crate) fn fetch_t2(&mut self) {
        self.pins.ale = false;
        self.pins.rd = false;
    }

    pub(crate) fn fetch_t3(&mut self) {
        self.pins.rd = true;
        self.regs.pc.val = self.regs.pc.val.wrapping_add(1);
    }

    pub(crate) fn mem_read_addr(&mut self, addr: u16) {
        self.pins.a = (addr >> 8) as u8;
        self.pins.ad = (addr & 0xFF) as u8;
        self.pins.ale = true;
        self.pins.io_m = false;
        self.pins.s0 = true;
        self.pins.s1 = false;
        self.pins.rd = true;
        self.pins.wr = true;
    }

    pub(crate) fn mem_read_t2(&mut self) {
        self.pins.ale = false;
        self.pins.rd = false;
    }

    pub(crate) fn mem_write_addr(&mut self, addr: u16) {
        self.pins.a = (addr >> 8) as u8;
        self.pins.ad = (addr & 0xFF) as u8;
        self.pins.ale = true;
        self.pins.io_m = false;
        self.pins.s0 = false;
        self.pins.s1 = true;
        self.pins.rd = true;
        self.pins.wr = true;
    }

    pub(crate) fn addr_write_t2(&mut self, data: u8) {
        self.pins.ale = false;
        self.pins.ad = data;
        self.pins.wr = false;
    }

    pub(crate) fn addr_write_t3(&mut self) {
        self.pins.wr = true;
    }

    // ----- Register access -----
    pub(crate) fn read_reg(&self, idx: u8) -> u8 {
        self.regs.read_reg(idx)
    }

    pub(crate) fn write_reg(&mut self, idx: u8, val: u8) {
        self.regs.write_reg(idx, val)
    }
}