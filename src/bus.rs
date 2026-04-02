use crate::io::IOPorts;
use crate::memory::{Addr, Memory};
use crate::pins::Pins;

pub struct SystemBus<'a>{
    memory: &'a mut Memory,
    io_ports: &'a mut IOPorts,
    address_latch: u8,
    address_lane: u8,
    data_lane: u8
}

impl<'a> SystemBus<'a> {
    pub fn connect(memory: &'a mut Memory, io_ports: &'a mut IOPorts) -> Self {
        SystemBus{
            memory,
            io_ports,
            address_latch: 0,
            address_lane: 0,
            data_lane: 0,
        }
    }

    #[inline]
    pub fn full_address(&self) -> Addr {
        (self.address_lane as Addr) << 8 | self.address_latch as Addr
    }

    #[inline]
    pub fn sync(&mut self, pins: &mut Pins){
        if pins.ale{
            self.address_latch = pins.ad
        }

        self.address_lane = pins.a;

        if pins.io_m{
            let port = self.address_latch;
            if !pins.rd{
                self.data_lane = self.io_ports.read(port);
                pins.ad = self.data_lane;
            }
            else if !pins.wr{
                self.data_lane = pins.ad;
                self.io_ports.write(port, self.data_lane);
            }
        }
        else{
            let full_address = self.full_address();
            if !pins.rd{
                self.data_lane = self.memory.read(full_address);
                pins.ad = self.data_lane;

            }
            else if !pins.wr{
                self.data_lane = pins.ad;
                self.memory.write(full_address, self.data_lane);
            }
        }
    }
}