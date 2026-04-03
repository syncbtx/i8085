use crate::registers::Registers;
use crate::pins::Pins;
use crate::flags::Flags;
use crate::alu::Alu;
use crate::cu::ControlUnit;

pub struct CPU {
    pub regs: Registers,
    pub pins: Pins,
    pub flags: Flags,
    pub alu: Alu,
    pub control: ControlUnit,
}

impl CPU {
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
        match idx {
            0 => self.regs.b.val,
            1 => self.regs.c.val,
            2 => self.regs.d.val,
            3 => self.regs.e.val,
            4 => self.regs.h.val,
            5 => self.regs.l.val,
            6 => 0,
            7 => self.regs.a.val,
            _ => 0,
        }
    }

    pub(crate) fn write_reg(&mut self, idx: u8, val: u8) {
        match idx {
            0 => self.regs.b.val = val,
            1 => self.regs.c.val = val,
            2 => self.regs.d.val = val,
            3 => self.regs.e.val = val,
            4 => self.regs.h.val = val,
            5 => self.regs.l.val = val,
            6 => (),
            7 => self.regs.a.val = val,
            _ => (),
        }
    }
}