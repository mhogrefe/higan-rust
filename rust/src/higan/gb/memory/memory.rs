//TODO test

use higan::gb::apu::apu::APU;
use higan::gb::cpu::cpu::CPUIO;

pub trait MMIO {
    fn read_io(&self, addr: u16) -> u8;

    fn write_io(&mut self, addr: u16, data: u8);
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Unmapped;

impl MMIO for Unmapped {
    fn read_io(&self, _addr: u16) -> u8 {
        0xff
    }

    fn write_io(&mut self, _addr: u16, _data: u8) {}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MMIOType {
    Unmapped,
    APU,
    CPU,
}

impl Default for MMIOType {
    fn default() -> MMIOType {
        MMIOType::Unmapped
    }
}

const MMIO_SIZE: usize = 65_536;

#[derive(Clone)]
pub struct Bus {
    pub mmio: [MMIOType; MMIO_SIZE],
    pub unmapped: Unmapped,
    pub apu: APU,
    pub cpu_io: CPUIO,
}

impl Default for Bus {
    fn default() -> Bus {
        Bus {
            mmio: [MMIOType::default(); MMIO_SIZE],
            unmapped: Unmapped,
            apu: APU::default(),
            cpu_io: CPUIO::default(),
        }
    }
}

impl Bus {
    pub fn read(&self, addr: u16) -> u8 {
        let data = match &self.mmio[addr as usize] {
            MMIOType::Unmapped => self.unmapped.read_io(addr),
            MMIOType::APU => self.apu.read_io(addr),
            MMIOType::CPU => self.cpu_io.read_io(addr),
        };

        //TODO if(cheat) {
        //TODO   if(auto result = cheat.find(addr, data)) return result();
        //TODO }

        data
    }

    // returns whether to do DMA stuff
    pub fn write(&mut self, addr: u16, data: u8) -> bool {
        match self.mmio[addr as usize] {
            MMIOType::Unmapped => self.unmapped.write_io(addr, data),
            MMIOType::APU => self.apu.write_io(addr, data),
            MMIOType::CPU => return self.cpu_io.write_io(addr, data),
        }
        false
    }

    pub fn power(&mut self) {
        for mmio in self.mmio.iter_mut() {
            *mmio = MMIOType::Unmapped;
        }
    }
}
