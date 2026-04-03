use crate::flags::Flags;

pub struct Alu {
    pub temp: u8,
}

impl Alu {
    pub fn new() -> Self {
        Alu { temp: 0 }
    }

    pub fn reset(&mut self) {
        self.temp = 0;
    }

    pub fn exec(&mut self, acc: &mut u8, flags: &mut Flags, opcode: u8, operand: u8) {
        let a = *acc;
        let group = (opcode >> 3) & 0x07;
        match group {
            0 => { // ADD
                let res = a.wrapping_add(operand);
                Self::set_flags_add(flags, a, operand, res, false);
                *acc = res;
            }
            1 => { // ADC
                let carry_in = flags.c.val;
                let res = a.wrapping_add(operand).wrapping_add(carry_in as u8);
                Self::set_flags_add(flags, a, operand, res, carry_in);
                *acc = res;
            }
            2 => { // SUB
                let res = a.wrapping_sub(operand);
                Self::set_flags_sub(flags, a, operand, res, false);
                *acc = res;
            }
            3 => { // SBB
                let borrow_in = flags.c.val;
                let res = a.wrapping_sub(operand).wrapping_sub(borrow_in as u8);
                Self::set_flags_sub(flags, a, operand, res, borrow_in);
                *acc = res;
            }
            4 => { // ANA
                let res = a & operand;
                Self::set_flags_logic(flags, res, true);
                *acc = res;
            }
            5 => { // XRA
                let res = a ^ operand;
                Self::set_flags_logic(flags, res, false);
                *acc = res;
            }
            6 => { // ORA
                let res = a | operand;
                Self::set_flags_logic(flags, res, false);
                *acc = res;
            }
            7 => { // CMP
                let res = a.wrapping_sub(operand);
                Self::set_flags_sub(flags, a, operand, res, false);
            }
            _ => (),
        }
    }

    fn set_flags_add(flags: &mut Flags, a: u8, b: u8, res: u8, carry_in: bool) {
        let full = a as u16 + b as u16 + carry_in as u16;
        flags.c.val = full > 0xFF;
        flags.s.val = (res & 0x80) != 0;
        flags.z.val = res == 0;
        flags.ac.val = ((a & 0x0F) + (b & 0x0F) + carry_in as u8) > 0x0F;
        flags.p.val = res.count_ones() % 2 == 0;
    }

    fn set_flags_sub(flags: &mut Flags, a: u8, b: u8, res: u8, borrow_in: bool) {
        let full = (a as u16).wrapping_sub(b as u16).wrapping_sub(borrow_in as u16);
        flags.c.val = full > 0xFF;
        flags.s.val = (res & 0x80) != 0;
        flags.z.val = res == 0;
        let half = (a & 0x0F).wrapping_sub(b & 0x0F).wrapping_sub(borrow_in as u8);
        flags.ac.val = half < 0x10;
        flags.p.val = res.count_ones() % 2 == 0;
    }

    fn set_flags_logic(flags: &mut Flags, res: u8, is_ana: bool) {
        flags.c.val = false;
        flags.s.val = (res & 0x80) != 0;
        flags.z.val = res == 0;
        flags.ac.val = is_ana;
        flags.p.val = res.count_ones() % 2 == 0;
    }
}