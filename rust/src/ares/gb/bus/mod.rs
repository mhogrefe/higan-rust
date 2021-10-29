use ares::gb::apu::apu::APU;
use ares::gb::cpu::cpu::CPU;

#[derive(Clone, Debug, Default)]
pub struct Bus {
    pub apu: APU,
    pub cpu: CPU,
}

// See higan-rust/cpp/ares/gb/bus/bus.cpp
impl Bus {
    pub fn read_with_cycle(&self, cycle: u32, address: u16, mut data: u8) -> u8 {
        // data &= cpu.readIO(cycle, address, data);
        data &= self.apu.read_io(cycle, address, data);
        // data &= ppu.readIO(cycle, address, data);
        // data &= cartridge.read(cycle, address, data);
        data
    }

    pub fn write_with_cycle(&mut self, cycle: u32, address: u16, data: u8) {
        // cpu.writeIO(cycle, address, data);
        self.apu.write_io(cycle, address, data);
        //ppu.writeIO(cycle, address, data);
        //cartridge.write(cycle, address, data);
    }

    pub fn read(&self, address: u16, mut data: u8) -> u8 {
        data &= self.read_with_cycle(2, address, data);
        data &= self.read_with_cycle(4, address, data);
        data
    }

    pub fn write(&mut self, address: u16, data: u8) {
        self.write_with_cycle(2, address, data);
        self.write_with_cycle(4, address, data);
    }
}
