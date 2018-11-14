use higan::gb::system::system::System;

pub trait MMIO {
    fn read_io(&self, system: &System, addr: u16) -> u8;

    fn write_io(&mut self, system: &System, addr: u16, data: u8);
}

pub struct Unmapped;

impl MMIO for Unmapped {
    fn read_io(&self, _system: &System, _addr: u16) -> u8 {
        0xff
    }

    fn write_io(&mut self, _system: &System, _addr: u16, _data: u8) {}
}

//TODO memory.cpp
