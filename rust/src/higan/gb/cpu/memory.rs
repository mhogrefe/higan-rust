//TODO test

use higan::gb::cpu::cpu::CPU;
use higan::gb::system::system::System;

impl CPU {
    pub fn idle(&mut self) {
        self.cycle_edge();
        self.step(4);
    }

    pub fn read(&mut self, system: &System, addr: u16) -> u8 {
        self.cycle_edge();
        self.step(4);
        self.bus.read(system, addr)
    }

    pub fn write(&mut self, system: &System, addr: u16, data: u8) {
        self.cycle_edge();
        self.step(4);
        self.bus.write(system, addr, data);
    }

    pub fn cycle_edge(&mut self) {
        if self.processor.r.ei {
            self.processor.r.ei = false;
            self.processor.r.ime = true;
        }
    }

    //VRAM DMA source can only be ROM or RAM
    pub fn read_dma(&self, system: &System, addr: u16) -> u8 {
        if addr < 0x8000 {
            //0000-7fff
            self.bus.read(system, addr)
        } else if addr < 0xa000 {
            //8000-9fff
            0xff
        } else if addr < 0xe000 {
            //a000-dfff
            self.bus.read(system, addr)
        } else {
            //e000-ffff
            0xff
        }
    }

    //VRAM DMA target is always VRAM
    pub fn write_dma(&mut self, system: &System, mut addr: u16, data: u8) {
        addr = 0x8000 | (addr & 0x1fff); //8000-9fff
        self.bus.write(system, addr, data)
    }

    pub fn read_debugger(&self, system: &System, addr: u16) -> u8 {
        self.bus.read(system, addr)
    }
}
