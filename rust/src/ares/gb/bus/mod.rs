use ares::gb::system::System;
use ares::platform::Platform;

// See higan-rust/cpp/ares/gb/bus/bus.cpp
impl<P: Platform> System<P> {
    pub fn bus_read_with_cycle(&mut self, cycle: u32, address: u16, mut data: u8) -> u8 {
        data &= self.cpu_read_io(cycle, address, data);
        data &= self.apu.read_io(cycle, address, data);
        // data &= ppu.readIO(cycle, address, data);
        // data &= cartridge.read(cycle, address, data);
        data
    }

    pub fn s_bus_write_with_cycle(&mut self, cycle: u32, address: u16, data: u8) {
        self.s_cpu_write_io(cycle, address, data);
        self.apu.write_io(cycle, address, data);
        //ppu.writeIO(cycle, address, data);
        //cartridge.write(cycle, address, data);
    }

    pub fn bus_read(&mut self, address: u16, mut data: u8) -> u8 {
        data &= self.bus_read_with_cycle(2, address, data);
        data &= self.bus_read_with_cycle(4, address, data);
        data
    }

    pub fn s_bus_write(&mut self, address: u16, data: u8) {
        self.s_bus_write_with_cycle(2, address, data);
        self.s_bus_write_with_cycle(4, address, data);
    }
}
