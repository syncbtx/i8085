use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("microcode.rs");
    let mut f = File::create(&dest_path).unwrap();

    macro_rules! newline {
        () => {
            writeln!(f, "").unwrap();
        };
    }

    writeln!(f, "pub const B: u8 = 0;").unwrap();
    writeln!(f, "pub const C: u8 = 1;").unwrap();
    writeln!(f, "pub const D: u8 = 2;").unwrap();
    writeln!(f, "pub const E: u8 = 3;").unwrap();
    writeln!(f, "pub const H: u8 = 4;").unwrap();
    writeln!(f, "pub const L: u8 = 5;").unwrap();
    writeln!(f, "pub const M: u8 = 6;").unwrap();
    writeln!(f, "pub const A: u8 = 7;").unwrap();

    writeln!(f, "pub const PC: u8 = 0;").unwrap();
    writeln!(f, "pub const BC: u8 = 1;").unwrap();
    writeln!(f, "pub const DE: u8 = 2;").unwrap();
    writeln!(f, "pub const HL: u8 = 3;").unwrap();
    writeln!(f, "pub const SP: u8 = 4;").unwrap();
    writeln!(f, "pub const WZ: u8 = 5;").unwrap();
    newline!();

    writeln!(f, "const M1: usize = 0;").unwrap();
    writeln!(f, "const M2: usize = 1;").unwrap();
    writeln!(f, "const M3: usize = 2;").unwrap();
    writeln!(f, "const M4: usize = 3;").unwrap();
    writeln!(f, "const M5: usize = 4;").unwrap();
    writeln!(f, "const T1: usize = 0;").unwrap();
    writeln!(f, "const T2: usize = 1;").unwrap();
    writeln!(f, "const T3: usize = 2;").unwrap();
    writeln!(f, "const T4: usize = 3;").unwrap();
    writeln!(f, "const T5: usize = 4;").unwrap();
    writeln!(f, "const T6: usize = 5;").unwrap();
    newline!();

    writeln!(f, "const NOP: usize =       0x00;").unwrap();
    writeln!(f, "const LXI_B_D16: usize = 0x01;").unwrap();
    writeln!(f, "const STAX_B: usize =    0x02;").unwrap();
    writeln!(f, "const INX_B: usize =     0x03;").unwrap();
    writeln!(f, "const INR_B: usize =     0x04;").unwrap();
    writeln!(f, "const DCR_B: usize =     0x05;").unwrap();
    writeln!(f, "const MVI_B_D8: usize =  0x06;").unwrap();
    writeln!(f, "const RLC: usize =       0x07;").unwrap();
    writeln!(f, "const DSUB: usize =      0x08;").unwrap();
    writeln!(f, "const DAD_B: usize =     0x09;").unwrap();
    writeln!(f, "const LDAX_B: usize =    0x0A;").unwrap();
    writeln!(f, "const DCX_B: usize =     0x0B;").unwrap();
    writeln!(f, "const INR_C: usize =     0x0C;").unwrap();
    writeln!(f, "const DCR_C: usize =     0x0D;").unwrap();
    writeln!(f, "const MVI_C_D8: usize =  0x0E;").unwrap();
    writeln!(f, "const RRC: usize =       0x0F;").unwrap();
    writeln!(f, "const LXI_D_D16: usize = 0x11;").unwrap();
    writeln!(f, "const STAX_D: usize =    0x12;").unwrap();
    writeln!(f, "const INX_D: usize =     0x13;").unwrap();
    writeln!(f, "const INR_D: usize =     0x14;").unwrap();
    writeln!(f, "const DCR_D: usize =     0x15;").unwrap();
    writeln!(f, "const MVI_D_D8: usize =  0x16;").unwrap();
    writeln!(f, "const DCX_D: usize =     0x1B;").unwrap();
    writeln!(f, "const HLT: usize =       0x76;").unwrap();
    newline!();

    writeln!(f, "#[repr(u8)]").unwrap();
    writeln!(f, "#[derive(Copy, Clone)]").unwrap();
    writeln!(f, "pub enum MicroOp {{").unwrap();
    writeln!(f, "    Nop,").unwrap();
    writeln!(f, "    FetchT1, FetchT2, FetchT3, DecodeT4,").unwrap();
    writeln!(f, "    MemReadT1(u8), MemReadT2, MemReadT3(u8),").unwrap();
    writeln!(f, "    MemWriteT1(u8), MemWriteT2(u8), MemWriteT3,").unwrap();
    writeln!(f, "    IOReadT1(u8), IOReadT2, IOReadT3(u8),").unwrap();
    writeln!(f, "    IOWriteT1(u8), IOWriteT2(u8), IOWriteT3,").unwrap();
    writeln!(f, "    MovRR, MovMR(u8), MovRM(u8), MviR(u8),").unwrap();
    writeln!(f, "    Halt, JmpWz, CondJmp(u8), AluReg, AluMem, AluImm,").unwrap();
    writeln!(f, "    Call, Ret, Push(u8), Pop(u8), Xthl, Sphl, Pchl,").unwrap();
    writeln!(f, "    Dad(u8), Inx(u8), Dcx(u8), Rst(u8), Rim, Sim,").unwrap();
    writeln!(f, "    Ei, Di, Rlc, Rrc, Ral, Rar, Cma, Stc, Cmc, Daa,").unwrap();
    writeln!(f, "    Inr(u8), Dcr(u8),").unwrap();
    writeln!(f, "}}").unwrap();

    writeln!(f, "pub const MICROCODE_ROM: [[[MicroOp; 6]; 5]; 256] = {{").unwrap();
    writeln!(f, "    let mut table = [[[MicroOp::Nop; 6]; 5]; 256];").unwrap();

    for op in 0..256 {
        writeln!(f, "    table[{}][M1][T1] = MicroOp::FetchT1;", op).unwrap();
        writeln!(f, "    table[{}][M1][T2] = MicroOp::FetchT2;", op).unwrap();
        writeln!(f, "    table[{}][M1][T3] = MicroOp::FetchT3;", op).unwrap();
        writeln!(f, "    table[{}][M1][T4] = MicroOp::DecodeT4;", op).unwrap();
    }
    newline!();

    macro_rules! microcode {
        ($f:expr, $op:ident, $m:ident, $t:ident, $microop:expr) => {
            writeln!($f, "    table[{}][{}][{}] = {};", stringify!($op), stringify!($m), stringify!($t), stringify!($microop)).unwrap();
        };
    }



    microcode!(f, LXI_B_D16, M2, T1, MicroOp::MemReadT1(PC));
    microcode!(f, LXI_B_D16, M2, T2, MicroOp::MemReadT2);
    microcode!(f, LXI_B_D16, M2, T3, MicroOp::MemReadT3(C));
    microcode!(f, LXI_B_D16, M3, T1, MicroOp::MemReadT1(PC));
    microcode!(f, LXI_B_D16, M3, T2, MicroOp::MemReadT2);
    microcode!(f, LXI_B_D16, M3, T3, MicroOp::MemReadT3(B));

    microcode!(f, STAX_B, M2, T1, MicroOp::MemWriteT1(BC));
    microcode!(f, STAX_B, M2, T2, MicroOp::MemWriteT2(A));
    microcode!(f, STAX_B, M2, T3, MicroOp::MemWriteT3);

    microcode!(f, INX_B, M1, T6, MicroOp::Inx(BC));
    microcode!(f, INR_B, M1, T4, MicroOp::Inr(B));
    microcode!(f, DCR_B, M1, T4, MicroOp::Dcr(B));

    microcode!(f, MVI_B_D8, M2, T1, MicroOp::MemReadT1(PC));
    microcode!(f, MVI_B_D8, M2, T2, MicroOp::MemReadT2);
    microcode!(f, MVI_B_D8, M2, T3, MicroOp::MviR(B));

    microcode!(f, RLC, M1, T4, MicroOp::Rlc);
    microcode!(f, DAD_B, M3, T3, MicroOp::Dad(BC));

    microcode!(f, LDAX_B, M2, T1, MicroOp::MemReadT1(BC));
    microcode!(f, LDAX_B, M2, T2, MicroOp::MemReadT2);
    microcode!(f, LDAX_B, M2, T3, MicroOp::MemReadT3(A));

    microcode!(f, DCX_B, M1, T6, MicroOp::Dcx(BC));
    microcode!(f, INR_C, M1, T4, MicroOp::Inr(C));
    microcode!(f, DCR_C, M1, T4, MicroOp::Dcr(C));

    microcode!(f, MVI_C_D8, M2, T1, MicroOp::MemReadT1(PC));
    microcode!(f, MVI_C_D8, M2, T2, MicroOp::MemReadT2);
    microcode!(f, MVI_C_D8, M2, T3, MicroOp::MviR(C));

    microcode!(f, RRC, M1, T4, MicroOp::Rrc);

    microcode!(f, LXI_D_D16, M2, T1, MicroOp::MemReadT1(PC));
    microcode!(f, LXI_D_D16, M2, T2, MicroOp::MemReadT2);
    microcode!(f, LXI_D_D16, M2, T3, MicroOp::MemReadT3(E));
    microcode!(f, LXI_D_D16, M3, T1, MicroOp::MemReadT1(PC));
    microcode!(f, LXI_D_D16, M3, T2, MicroOp::MemReadT2);
    microcode!(f, LXI_D_D16, M3, T3, MicroOp::MemReadT3(D));

    microcode!(f, STAX_D, M2, T1, MicroOp::MemWriteT1(DE));
    microcode!(f, STAX_D, M2, T2, MicroOp::MemWriteT2(A));
    microcode!(f, STAX_D, M2, T3, MicroOp::MemWriteT3);

    microcode!(f, INX_D, M1, T6, MicroOp::Inx(DE));
    microcode!(f, INR_D, M1, T4, MicroOp::Inr(D));
    microcode!(f, DCR_D, M1, T4, MicroOp::Dcr(D));

    microcode!(f, MVI_D_D8, M2, T1, MicroOp::MemReadT1(PC));
    microcode!(f, MVI_D_D8, M2, T2, MicroOp::MemReadT2);
    microcode!(f, MVI_D_D8, M2, T3, MicroOp::MviR(D));

    writeln!(f, "    table").unwrap();
    writeln!(f, "}};").unwrap();

    writeln!(f, "pub const T_STATE_COUNT: [[u8; 5]; 256] = {{").unwrap();
    writeln!(f, "    let mut last_t = [[4; 5]; 256];").unwrap();
    writeln!(f, "    last_t[HLT][0] =   5;").unwrap();
    writeln!(f, "    last_t[INX_B][0] = 6;").unwrap();
    writeln!(f, "    last_t[DCX_B][0] = 6;").unwrap();
    writeln!(f, "    last_t[INX_D][0] = 6;").unwrap();
    writeln!(f, "    last_t[DCX_D][0] = 6;").unwrap();
    writeln!(f, "    last_t").unwrap();
    writeln!(f, "}};").unwrap();

    writeln!(f, "pub const M_CYCLE_COUNT: [u8; 256] = {{").unwrap();
    writeln!(f, "    let mut last_m = [1; 256];").unwrap();
    writeln!(f, "    last_m[NOP] = 1;").unwrap();
    writeln!(f, "    last_m[LXI_B_D16] = 3;").unwrap();
    writeln!(f, "    last_m[STAX_B]    = 2;").unwrap();
    writeln!(f, "    last_m[INX_B]     = 1;").unwrap();
    writeln!(f, "    last_m[INR_B]     = 1;").unwrap();
    writeln!(f, "    last_m[DCR_B]     = 1;").unwrap();
    writeln!(f, "    last_m[MVI_B_D8]  = 2;").unwrap();
    writeln!(f, "    last_m[RLC]       = 1;").unwrap();
    writeln!(f, "    last_m[DAD_B]     = 3;").unwrap();
    writeln!(f, "    last_m[LDAX_B]    = 2;").unwrap();
    writeln!(f, "    last_m[DCX_B]     = 1;").unwrap();
    writeln!(f, "    last_m[INR_C]     = 1;").unwrap();
    writeln!(f, "    last_m[DCR_C]     = 1;").unwrap();
    writeln!(f, "    last_m[MVI_C_D8]  = 2;").unwrap();
    writeln!(f, "    last_m[RRC]       = 1;").unwrap();
    writeln!(f, "    last_m[LXI_D_D16] = 3;").unwrap();
    writeln!(f, "    last_m[STAX_D]    = 2;").unwrap();
    writeln!(f, "    last_m[INX_D]     = 1;").unwrap();
    writeln!(f, "    last_m[INR_D]     = 1;").unwrap();
    writeln!(f, "    last_m[DCR_D]     = 1;").unwrap();
    writeln!(f, "    last_m[MVI_D_D8]  = 2;").unwrap();
    writeln!(f, "    last_m").unwrap();
    writeln!(f, "}};").unwrap();
}