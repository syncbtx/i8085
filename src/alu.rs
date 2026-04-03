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

    pub fn set_flags_add(flags: &mut Flags, a: u8, b: u8, res: u8, carry_in: bool) {
        let full = a as u16 + b as u16 + carry_in as u16;
        let carry_out_7 = full > 0xFF;
        let carry_out_6 = (a as u16 & 0x7F) + (b as u16 & 0x7F) + carry_in as u16 > 0x7F;

        flags.c.val = carry_out_7;
        flags.s.val = (res & 0x80) != 0;
        flags.z.val = res == 0;
        flags.ac.val = ((a & 0x0F) + (b & 0x0F) + carry_in as u8) > 0x0F;
        flags.p.val = res.count_ones() % 2 == 0;

        // Undocumented V and K
        flags.v.val = carry_out_6 ^ carry_out_7;
        flags.k.val = flags.v.val ^ flags.s.val;
    }

    pub fn set_flags_sub(flags: &mut Flags, a: u8, b: u8, res: u8, borrow_in: bool) {
        let full = (a as u16).wrapping_sub(b as u16).wrapping_sub(borrow_in as u16);
        let borrow_out_7 = full > 0xFF; 
        let borrow_out_6 = (a as u16 & 0x7F).wrapping_sub(b as u16 & 0x7F).wrapping_sub(borrow_in as u16) > 0x7F;

        flags.c.val = borrow_out_7;
        flags.s.val = (res & 0x80) != 0;
        flags.z.val = res == 0;
        flags.ac.val = (a & 0x0F) < ((b & 0x0F) + borrow_in as u8);
        flags.p.val = res.count_ones() % 2 == 0;

        flags.v.val = borrow_out_6 ^ borrow_out_7;
        flags.k.val = flags.v.val ^ flags.s.val;
    }

    pub fn set_flags_logic(flags: &mut Flags, res: u8, is_ana: bool) {
        flags.c.val = false;
        flags.s.val = (res & 0x80) != 0;
        flags.z.val = res == 0;
        flags.ac.val = is_ana;
        flags.p.val = res.count_ones() % 2 == 0;
        flags.v.val = false;
        flags.k.val = flags.s.val;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::flags::Flags;

    #[test]
    fn test_v_and_k_flags() {
        let mut flags = Flags::new();
        Alu::set_flags_add(&mut flags, 0x7F, 0x01, 0x80, false);
        assert!(flags.v.val, "V should be set on 127 + 1");
        assert!(flags.s.val, "S should be set on 127 + 1");
        assert!(!flags.k.val, "K should be 0 because V^S = 1^1 = 0");
        Alu::set_flags_add(&mut flags, 0x80, 0xFF, 0x7F, false);
        assert!(flags.v.val, "V should be set on -128 + -1");
        assert!(!flags.s.val, "S should be 0 on -128 + -1");
        assert!(flags.k.val, "K should be 1 because V^S = 1^0 = 1");
        Alu::set_flags_sub(&mut flags, 0x00, 0x01, 0xFF, false);
        assert!(flags.c.val, "C should be set (borrow)");
        assert!(flags.ac.val, "AC should be set (nibble borrow)");
    }
}