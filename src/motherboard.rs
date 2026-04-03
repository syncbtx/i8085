use crate::cpu::CPU;
use crate::memory::Memory;
use crate::io::IOPorts;
use crate::bus::SystemBus;

pub struct Motherboard {
    pub cpu: CPU,
    pub mem: Memory,
    pub io: IOPorts,
    pub address_latch: u8,
    pub address_lane: u8,
    pub data_lane: u8,
    pub total_cycles: u64,
}

impl Motherboard {
    pub fn new() -> Self {
        Motherboard {
            cpu: CPU::new(),
            mem: Memory::new(),
            io: IOPorts::new(),
            address_latch: 0,
            address_lane: 0,
            data_lane: 0,
            total_cycles: 0,
        }
    }

    pub fn with_bootloader(image: &[u8]) -> Self {
        Motherboard {
            cpu: CPU::new(),
            mem: Memory::with_bootloader(image),
            io: IOPorts::new(),
            address_latch: 0,
            address_lane: 0,
            data_lane: 0,
            total_cycles: 0,
        }
    }

    pub fn tick(&mut self) {
        let mut bus = SystemBus {
            memory: &mut self.mem,
            io_ports: &mut self.io,
            address_latch: self.address_latch,
            address_lane: self.address_lane,
            data_lane: self.data_lane,
        };
        self.cpu.tick(&mut bus);
        self.address_latch = bus.address_latch;
        self.address_lane = bus.address_lane;
        self.data_lane = bus.data_lane;
        self.total_cycles += 1;
    }

    pub fn load_to_ram(&mut self, offset: u16, data: &[u8]) -> Result<(), &'static str> {
        let start = crate::memory::RAM_START + offset;
        self.mem.write_slice(start, data)
    }

    pub fn run_until_limit(&mut self, max_ticks: u64) {
        for _ in 0..max_ticks {
            let halted = !self.cpu.pins.io_m && !self.cpu.pins.s1 && !self.cpu.pins.s0;
            if halted {
                break;
            }
            self.tick();
        }
    }

    pub fn run_until_halt(&mut self) {
        let mut halt = !self.cpu.pins.io_m && !self.cpu.pins.s1 && !self.cpu.pins.s0;
        while !halt {
            self.tick();
            halt = !self.cpu.pins.io_m && !self.cpu.pins.s1 && !self.cpu.pins.s0;
        }
    }
    
    
}