use ares::emulator::types::U13;
use ares::gb::system::{Model, System};
use ares::platform::Platform;

// See higan-rust/cpp/ares/gb/cpu/memory.cpp
impl<P: Platform> System<P> {
    pub fn cpu_stop(&mut self) {
        self.cpu_idle();
        if self.cpu.model == Model::SuperGameBoy {
            //TODO scheduler.exit(Event::Step);
        }
    }

    pub fn cpu_halt(&mut self) {
        self.cpu_idle();
        if self.cpu.status.interrupt_latch != 0 {
            self.cpu.r.halt = false;
        }
        if self.cpu.model == Model::SuperGameBoy {
            //TODO scheduler.exit(Event::Step);
        }
    }

    pub fn cpu_halt_bug_trigger(&mut self) {
        // halt bug is triggered when IME is off, and IE & IF != 0
        // does not properly emulate two halts in a rom
        if !self.cpu.r.ime && self.cpu.status.interrupt_latch != 0 {
            self.cpu.r.halt_bug = true;
        }
    }

    pub fn cpu_idle(&mut self) {
        if self.cpu.r.ei {
            self.cpu.r.ei = false;
            self.cpu.r.ime = true
        };
        self.cpu_step(1);
        self.cpu_step(1);
        self.cpu.status.interrupt_latch =
            self.cpu.status.interrupt_flag.x() & self.cpu.status.interrupt_enable;
        self.cpu_step(1);
        self.cpu_step(1);
    }

    pub fn cpu_read(&mut self, address: u16) -> u8 {
        let mut data = 0xff;
        if self.cpu.r.ei {
            self.cpu.r.ei = false;
            self.cpu.r.ime = true
        };
        data &= self.bus_read_with_cycle(0, address, data);
        self.cpu_step(1);
        data &= self.bus_read_with_cycle(1, address, data);
        self.cpu_step(1);
        data &= self.bus_read_with_cycle(2, address, data);
        self.cpu.status.interrupt_latch =
            self.cpu.status.interrupt_flag.x() & self.cpu.status.interrupt_enable;
        self.cpu_step(1);
        data &= self.bus_read_with_cycle(3, address, data);
        self.cpu_step(1);
        data &= self.bus_read_with_cycle(4, address, data);
        data
    }

    pub fn cpu_write(&mut self, address: u16, data: u8) {
        if self.cpu.r.ei {
            self.cpu.r.ei = false;
            self.cpu.r.ime = true
        };
        self.bus_write_with_cycle(0, address, data);
        self.cpu_step(1);
        self.bus_write_with_cycle(1, address, data);
        self.cpu_step(1);
        self.bus_write_with_cycle(2, address, data);
        self.cpu_step(1);
        self.bus_write_with_cycle(3, address, data);
        self.cpu_step(1);
        self.bus_write_with_cycle(4, address, data);
        self.cpu.status.interrupt_latch =
            self.cpu.status.interrupt_flag.x() & self.cpu.status.interrupt_enable;
    }

    // VRAM DMA source can only be ROM or RAM
    pub fn cpu_read_dma(&mut self, address: u16, data: u8) -> u8 {
        if address < 0x8000 {
            self.bus_read(address, data)
        } else if address < 0xa000 {
            data
        } else if address < 0xe000 {
            self.bus_read(address, data)
        } else {
            data
        }
    }

    // VRAM DMA target is always VRAM
    pub fn cpu_write_dma(&mut self, address: U13, data: u8) {
        self.bus_write(0x8000 | address.x(), data)
    }

    pub fn cpu_read_debugger(&mut self, address: u16) -> u8 {
        self.bus_read(address, 0xff)
    }
}
