use crate::cpu::CPU;
use crate::microcode::{MicroOp, MICROCODE_ROM, T_STATE_COUNT, M_CYCLE_COUNT, PC, BC, DE, HL, SP, WZ};

pub struct ControlUnit {
    pub ir: u8,
    pub m_cycle: u8,
    pub t_state: u8,
    pub inte: bool,
    pub mask_55: bool,
    pub mask_65: bool,
    pub mask_75: bool,
    pub rst75_pending: bool,
}

impl ControlUnit {
    pub fn new() -> Self {
        ControlUnit {
            ir: 0,
            m_cycle: 0,
            t_state: 0,
            inte: false,
            mask_55: false,
            mask_65: false,
            mask_75: false,
            rst75_pending: false,
        }
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }

    pub fn fetch_opcode(cpu: &mut CPU) {
        cpu.control.ir = cpu.pins.ad;
    }

    pub fn execute_pla(cpu: &mut CPU) {
        let op = cpu.control.ir as usize;
        let m = (cpu.control.m_cycle) as usize;
        let t = (cpu.control.t_state) as usize;
        match MICROCODE_ROM[op][m][t] {
            MicroOp::Nop => (),
            MicroOp::FetchT1 => cpu.fetch_t1(),
            MicroOp::FetchT2 => cpu.fetch_t2(),
            MicroOp::FetchT3 => {
                cpu.fetch_t3();
                Self::fetch_opcode(cpu);
            }
            MicroOp::DecodeT4 => (),

            MicroOp::MemReadT1(src) => {
                let addr = match src {
                    PC => cpu.regs.pc.val,
                    BC => cpu.regs.get_bc(),
                    DE => cpu.regs.get_de(),
                    HL => cpu.regs.get_hl(),
                    SP => cpu.regs.sp.val,
                    WZ => cpu.regs.get_wz(),
                    _ => 0,
                };
                cpu.mem_read_addr(addr);
            }
            MicroOp::MemReadT2 => cpu.mem_read_t2(),
            MicroOp::MemReadT3(dst) => {
                let data = cpu.pins.ad;
                cpu.regs.write_reg(dst, data);
                cpu.regs.pc.val = cpu.regs.pc.val.wrapping_add(1);
                cpu.pins.rd = true;
            }

            MicroOp::MemWriteT1(src) => {
                let addr = match src {
                    PC => cpu.regs.pc.val,
                    BC => cpu.regs.get_bc(),
                    DE => cpu.regs.get_de(),
                    HL => cpu.regs.get_hl(),
                    SP => cpu.regs.sp.val,
                    WZ => cpu.regs.get_wz(),
                    _ => 0,
                };
                cpu.mem_write_addr(addr);
            }
            MicroOp::MemWriteT2(src_reg) => {
                let data = cpu.regs.read_reg(src_reg);
                cpu.addr_write_t2(data);
            }
            MicroOp::MemWriteT3 => cpu.addr_write_t3(),

            MicroOp::MovRR => {
                let op = cpu.control.ir;
                let src = op & 0x07;
                let dst = (op >> 3) & 0x07;
                let val = cpu.read_reg(src);
                cpu.write_reg(dst, val);
            }
            MicroOp::MovRM(dst) => {
                let data = cpu.pins.ad;
                cpu.write_reg(dst, data);
                cpu.pins.rd = true;
            }
            MicroOp::MovMR(src) => {
                let data = cpu.read_reg(src);
                cpu.addr_write_t2(data);
            }
            MicroOp::MviR(reg) => {
                let data = cpu.pins.ad;
                if reg == 6 {
                    cpu.regs.z.val = data;
                } else {
                    cpu.write_reg(reg, data);
                }
                cpu.regs.pc.val = cpu.regs.pc.val.wrapping_add(1);
                cpu.pins.rd = true;
            }

            MicroOp::Inx(pair) => {
                let new = match pair {
                    BC => cpu.regs.get_bc().wrapping_add(1),
                    DE => cpu.regs.get_de().wrapping_add(1),
                    HL => cpu.regs.get_hl().wrapping_add(1),
                    SP => cpu.regs.sp.val.wrapping_add(1),
                    _ => 0,
                };
                match pair {
                    BC => cpu.regs.set_bc(new),
                    DE => cpu.regs.set_de(new),
                    HL => cpu.regs.set_hl(new),
                    SP => cpu.regs.sp.val = new,
                    _ => (),
                }
            }
            MicroOp::Dcx(pair) => {
                let new = match pair {
                    BC => cpu.regs.get_bc().wrapping_sub(1),
                    DE => cpu.regs.get_de().wrapping_sub(1),
                    HL => cpu.regs.get_hl().wrapping_sub(1),
                    SP => cpu.regs.sp.val.wrapping_sub(1),
                    _ => 0,
                };
                match pair {
                    BC => cpu.regs.set_bc(new),
                    DE => cpu.regs.set_de(new),
                    HL => cpu.regs.set_hl(new),
                    SP => cpu.regs.sp.val = new,
                    _ => (),
                }
            }
            MicroOp::Inr(reg) => {
                let val = cpu.read_reg(reg).wrapping_add(1);
                cpu.write_reg(reg, val);
                cpu.flags.s.val = (val & 0x80) != 0;
                cpu.flags.z.val = val == 0;
                cpu.flags.ac.val = (val & 0x0F) == 0;
                cpu.flags.p.val = val.count_ones() % 2 == 0;
            }
            MicroOp::Dcr(reg) => {
                let val = cpu.read_reg(reg).wrapping_sub(1);
                cpu.write_reg(reg, val);
                cpu.flags.s.val = (val & 0x80) != 0;
                cpu.flags.z.val = val == 0;
                cpu.flags.ac.val = (val & 0x0F) == 0x0F;
                cpu.flags.p.val = val.count_ones() % 2 == 0;
            }
            MicroOp::Dad(pair) => {
                let hl = cpu.regs.get_hl();
                let reg_pair = match pair {
                    BC => cpu.regs.get_bc(),
                    DE => cpu.regs.get_de(),
                    HL => hl,
                    SP => cpu.regs.sp.val,
                    _ => 0,
                };
                let (res, carry) = hl.overflowing_add(reg_pair);
                cpu.regs.set_hl(res);
                cpu.flags.c.val = carry;
            }
            MicroOp::Rlc => {
                let a = cpu.regs.a.val;
                cpu.flags.c.val = (a & 0x80) != 0;
                cpu.regs.a.val = a.rotate_left(1);
            }
            MicroOp::Rrc => {
                let a = cpu.regs.a.val;
                cpu.flags.c.val = (a & 0x01) != 0;
                cpu.regs.a.val = a.rotate_right(1);
            }
            MicroOp::AluReg => {
                let src = cpu.control.ir & 0x07;
                let val = cpu.read_reg(src);
                cpu.alu.exec(&mut cpu.regs.a.val, &mut cpu.flags, cpu.control.ir, val);
            }
            MicroOp::AluMem => {
                let val = cpu.pins.ad;
                cpu.pins.rd = true;
                cpu.alu.exec(&mut cpu.regs.a.val, &mut cpu.flags, cpu.control.ir, val);
            }
            MicroOp::AluImm => {
                let val = cpu.pins.ad;
                cpu.regs.pc.val = cpu.regs.pc.val.wrapping_add(1);
                cpu.pins.rd = true;
                cpu.alu.exec(&mut cpu.regs.a.val, &mut cpu.flags, cpu.control.ir, val);
            }
            MicroOp::Halt => {
                cpu.pins.io_m = false;
                cpu.pins.s1 = false;
                cpu.pins.s0 = false;
                cpu.pins.ale = false;
                cpu.pins.rd = true;
                cpu.pins.wr = true;
                cpu.pins.a = 0;
                cpu.pins.ad = 0;
            }
            MicroOp::JmpWz => {
                cpu.regs.pc.val = cpu.regs.get_wz();
            }
            _ => (),
        }
    }

    pub fn advance_clock(&mut self) {
        let op = self.ir as usize;
        let last_t = T_STATE_COUNT[op][(self.m_cycle) as usize];
        let last_m = M_CYCLE_COUNT[op];

        if self.t_state < last_t - 1 {
            self.t_state += 1;
        } else if self.m_cycle < last_m - 1 {
            self.m_cycle += 1;
            self.t_state = 0;
        } else {
            self.m_cycle = 0;
            self.t_state = 0;
        }
    }
}