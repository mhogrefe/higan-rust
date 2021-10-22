//TODO test

use ares::gb::cpu::cpu::CPU;

impl CPU {
    pub fn idle(&mut self) {
        self.cycle_edge();
        self.step(4);
    }

    /*
    pub fn read(&mut self, addr: u16) -> u8 {
        self.cycle_edge();
        self.step(4);
        self.bus.read(addr)
    }*/

    /*
    pub fn write(&mut self, addr: u16, data: u8) {
        self.cycle_edge();
        self.step(4);
        self.bus_write(addr, data);
    }*/

    pub fn cycle_edge(&mut self) {
        if self.processor.r.ei {
            self.processor.r.ei = false;
            self.processor.r.ime = true;
        }
    }

    /*
    pub fn read_debugger(&self, addr: u16) -> u8 {
        self.bus.read(addr)
    }*/
}

/*
impl Bus {
    // VRAM DMA source can only be ROM or RAM
    pub fn read_dma(&self, addr: u16) -> u8 {
        if addr < 0x8000 {
            //0000-7fff
            self.read(addr)
        } else if addr < 0xa000 {
            //8000-9fff
            0xff
        } else if addr < 0xe000 {
            //a000-dfff
            self.read(addr)
        } else {
            //e000-ffff
            0xff
        }
    }
}

impl CPU {
    // VRAM DMA target is always VRAM
    pub fn bus_write_dma(&mut self, mut addr: u16, data: u8) {
        addr = 0x8000 | (addr & 0x1fff); //8000-9fff
        self.bus_write(addr, data);
    }
}*/
