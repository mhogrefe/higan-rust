//TODO test

use higan::gb::apu::apu::APU;
use higan::gb::system::system::System;

pub trait MMIO {
    fn read_io(&self, system: &System, addr: u16) -> u8;

    fn write_io(&mut self, system: &System, addr: u16, data: u8);
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Unmapped;

impl MMIO for Unmapped {
    fn read_io(&self, _system: &System, _addr: u16) -> u8 {
        0xff
    }

    fn write_io(&mut self, _system: &System, _addr: u16, _data: u8) {}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MMIOType {
    Unmapped,
    APU,
}

impl Default for MMIOType {
    fn default() -> MMIOType {
        MMIOType::Unmapped
    }
}

#[derive(Clone)]
pub struct Bus {
    pub mmio: [MMIOType; 65536],
    pub unmapped: Unmapped,
    pub apu: APU,
}

impl Default for Bus {
    fn default() -> Bus {
        Bus {
            mmio: [MMIOType::default(); 65536],
            unmapped: Unmapped,
            apu: APU::default(),
        }
    }
}

impl Bus {
    pub fn read(&self, system: &System, addr: u16) -> u8 {
        let data = match &self.mmio[addr as usize] {
            MMIOType::Unmapped => self.unmapped.read_io(system, addr),
            MMIOType::APU => self.apu.read_io(system, addr),
        };

        //TODO if(cheat) {
        //TODO   if(auto result = cheat.find(addr, data)) return result();
        //TODO }

        data
    }

    pub fn write(&mut self, system: &System, addr: u16, data: u8) {
        match self.mmio[addr as usize] {
            MMIOType::Unmapped => self.unmapped.write_io(system, addr, data),
            MMIOType::APU => self.apu.write_io(system, addr, data),
        }
    }

    pub fn power(&mut self) {
        for mmio in self.mmio.iter_mut() {
            *mmio = MMIOType::Unmapped;
        }
    }
}
