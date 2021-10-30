use ares::emulator::types::U13;
use ares::gb::bus::Bus;

// See higan-rust/cpp/ares/gb/cpu/memory.cpp
impl Bus {
    pub fn cpu_stop(&mut self) {
        self.cpu_idle();
        if self.cpu.model_is_super_game_boy {
            //TODO scheduler.exit(Event::Step);
        }
    }

    pub fn cpu_halt(&mut self) {
        self.cpu_idle();
        if self.cpu.status.interrupt_latch != 0 {
            self.cpu.r.halt = false;
        }
        if self.cpu.model_is_super_game_boy {
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
        data &= self.read_with_cycle(0, address, data);
        self.cpu_step(1);
        data &= self.read_with_cycle(1, address, data);
        self.cpu_step(1);
        data &= self.read_with_cycle(2, address, data);
        self.cpu.status.interrupt_latch =
            self.cpu.status.interrupt_flag.x() & self.cpu.status.interrupt_enable;
        self.cpu_step(1);
        data &= self.read_with_cycle(3, address, data);
        self.cpu_step(1);
        data &= self.read_with_cycle(4, address, data);
        data
    }

    pub fn cpu_write(&mut self, address: u16, data: u8) {
        if self.cpu.r.ei {
            self.cpu.r.ei = false;
            self.cpu.r.ime = true
        };
        self.write_with_cycle(0, address, data);
        self.cpu_step(1);
        self.write_with_cycle(1, address, data);
        self.cpu_step(1);
        self.write_with_cycle(2, address, data);
        self.cpu_step(1);
        self.write_with_cycle(3, address, data);
        self.cpu_step(1);
        self.write_with_cycle(4, address, data);
        self.cpu.status.interrupt_latch =
            self.cpu.status.interrupt_flag.x() & self.cpu.status.interrupt_enable;
    }

    // VRAM DMA source can only be ROM or RAM
    pub fn cpu_read_dma(&mut self, address: u16, data: u8) -> u8 {
        if address < 0x8000 {
            self.read(address, data)
        } else if address < 0xa000 {
            data
        } else if address < 0xe000 {
            self.read(address, data)
        } else {
            data
        }
    }

    // VRAM DMA target is always VRAM
    pub fn cpu_write_dma(&mut self, address: U13, data: u8) {
        self.write(0x8000 | address.x(), data)
    }

    pub fn cpu_read_debugger(&mut self, address: u16) -> u8 {
        self.read(address, 0xff)
    }
}
