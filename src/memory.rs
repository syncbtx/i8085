use std::ops::{Index, IndexMut};

pub type Addr = u16;

pub const ROM_START: Addr = 0x0000;
pub const ROM_END: Addr = 0x0FFF;
pub const ROM_SIZE: usize = 0x1000;

pub struct ROM{
    mem: [u8; ROM_SIZE],
}

impl ROM{
    pub fn blank() -> ROM{
        ROM{mem: [0x00; ROM_SIZE]}
    }

    fn burn(image: &[u8]) -> Self{
        assert_eq!(image.len(), ROM_SIZE);
        let mut rom = ROM{mem: [0; ROM_SIZE]};
        rom.mem[..image.len()].copy_from_slice(image);
        rom
    }

    pub fn reset(&mut self){
        self.mem = [0x00; ROM_SIZE];
    }

    pub fn read(&self, addr: Addr) -> u8{
        self.mem[(addr - ROM_START) as usize]
    }
}

pub const RAM_START: Addr = 0x1000;
pub const RAM_END: Addr = 0xFFFF;
pub const RAM_SIZE: usize = 0x10000;

pub struct RAM{
    mem: [u8; RAM_SIZE],
}

impl RAM{
    pub fn new() -> RAM{
        RAM{mem: [0x00; RAM_SIZE]}
    }

    pub fn reset(&mut self){
        self.mem = [0x00; RAM_SIZE];
    }

    #[inline]
    pub fn read(&self, addr: Addr) -> u8{
        self.mem[(addr) as usize]
    }

    #[inline]
    pub fn write(&mut self, addr: Addr, val: u8){
        self.mem[(addr) as usize] = val;
    }
}

pub struct Memory{
    pub rom: ROM,
    pub ram: RAM,
}
impl Memory{
    pub fn new() -> Self{
        Memory{
            rom: ROM::blank(),
            ram: RAM::new(),
        }
    }

    pub fn with_bootloader(image: &[u8]) -> Self{
        Memory{
            rom: ROM::burn(image),
            ram: RAM::new(),
        }
    }

    pub fn reset(&mut self){
        self.rom.reset();
        self.ram.reset();
    }

    #[inline]
    pub fn read(&self, addr: Addr) -> u8{
        if addr <= ROM_END as u16 {
            self.rom.read(addr)
        }
        else{
            self.ram.read(addr)
        }
    }

    #[inline]
    pub fn write(&mut self, addr: Addr, val: u8) {
        if addr <= ROM_END {
            #[cfg(debug_assertions)]
            panic!("attempt to write to ROM at address {:#06X} failed!", addr);
            #[cfg(not(debug_assertions))]
            let _ = val;
        } else {
            self.ram.write(addr, val);
        }
    }
}

impl Index<Addr> for Memory {
    type Output = u8;
    fn index(&self, index: Addr) -> &Self::Output {
        if index <= ROM_END {
            &self.rom.mem[index as usize]
        } else {
            &self.ram.mem[index as usize]
        }
    }
}

impl IndexMut<Addr> for Memory {
    fn index_mut(&mut self, index: Addr) -> &mut Self::Output {
        if index <= ROM_END {
            panic!("attempt to write to ROM at address {:#06X}", index);
        }
        &mut self.ram.mem[index as usize]
    }
}
