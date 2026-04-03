#[derive(Debug)]
pub struct Pins {
    pub a: u8,
    pub ad: u8,
    pub ale: bool,
    pub rd: bool,
    pub wr: bool,
    pub io_m: bool,
    pub s1: bool,
    pub s0: bool,
    pub ready: bool,
    pub trap: bool,
    pub rst7_5: bool,
    pub rst6_5: bool,
    pub rst5_5: bool,
    pub intr: bool,
    pub inta: bool,
    pub sid: bool,
    pub sod: bool,
    pub hold: bool,
    pub hlda: bool,
    pub clk_out: bool,
    pub x1: bool,
    pub x2: bool,
    pub reset_in: bool,
    pub reset_out: bool,
    pub vcc: bool,
    pub vss: bool,
}

impl Pins {
    pub fn new() -> Self {
        Pins {
            ad: 0,
            a: 0,
            ale: false,
            rd: true,
            wr: true,
            io_m: false,
            s1: true,
            s0: true,
            ready: true,
            trap: false,
            rst7_5: false,
            rst6_5: false,
            rst5_5: false,
            intr: false,
            inta: true,
            sid: false,
            sod: false,
            hold: false,
            hlda: false,
            clk_out: false,
            x1: false,
            x2: false,
            reset_in: true,
            reset_out: false,
            vcc: true,
            vss: false,
        }
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }
}