use crate::flags::Flags;

pub struct ALU{
    pub temp: u8
}

impl ALU {
    pub fn new() -> Self {
        ALU{
            temp: 0
        }
    }

    pub fn reset(&mut self) {
        self.temp = 0;
    }

    pub fn exec(&mut self, acc: &mut u8, flags: &mut Flags, opcode: u8, operand: u8) {
        let a = *acc;
        let group = (opcode >> 3) & 0x07;
        match group {
            0 => {
                
            }
            _ => unreachable!()
        }
    }

    pub fn set_flags_add(){}
    pub fn set_flags_sub(){}
    pub fn set_flags_logic(){}
}