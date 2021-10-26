//TODO test

use ares::gb::apu::apu::APU;
use ares::gb::cpu::cpu::CPUIO;

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
    pub fn power(&mut self) {
        for mmio in self.mmio.iter_mut() {
            *mmio = MMIOType::Unmapped;
        }
    }
}
