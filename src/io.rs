pub const IO_PORTS: usize = 256;

pub struct IOPorts{
    pub ports: [u8; IO_PORTS],
}

impl IOPorts{
    pub fn new() -> Self{
        IOPorts{ports: [0; IO_PORTS]}
    }

    pub fn read(&self, port: u8) -> u8{
        self.ports[port as usize]
    }

    pub fn write(&mut self, port: u8, val: u8){
        self.ports[port as usize] = val;
    }
}