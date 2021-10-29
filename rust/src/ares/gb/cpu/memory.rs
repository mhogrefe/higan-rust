use ares::gb::bus::Bus;

// See higan-rust/cpp/ares/gb/cpu/memory.cpp
impl Bus {
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
}
