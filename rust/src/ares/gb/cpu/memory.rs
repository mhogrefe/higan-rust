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
        match self.cpu_idle_sync_point {
            0 => self.cpu_idle_fresh(),
            1 => self.cpu_idle_resume_at_1(),
            2 => self.cpu_idle_resume_at_2(),
            3 => self.cpu_idle_resume_at_3(),
            4 => self.cpu_idle_resume_at_4(),
            _ => panic!(),
        }
    }

    fn cpu_idle_fresh(&mut self) {
        if self.cpu.r.ei {
            self.cpu.r.ei = false;
            self.cpu.r.ime = true
        };
        // ** S1
        self.cpu_step(1);
        if self.cpu_return_to_sync {
            self.cpu_idle_sync_point = 1;
            return;
        }

        // ** S2
        self.cpu_step(1);
        if self.cpu_return_to_sync {
            self.cpu_idle_sync_point = 2;
            return;
        }

        self.cpu.status.interrupt_latch =
            self.cpu.status.interrupt_flag.x() & self.cpu.status.interrupt_enable;
        // ** S3
        self.cpu_step(1);
        if self.cpu_return_to_sync {
            self.cpu_idle_sync_point = 3;
            return;
        }

        // ** S4
        self.cpu_step(1);
        if self.cpu_return_to_sync {
            self.cpu_idle_sync_point = 4;
            return;
        }

        self.cpu_idle_sync_point = 0;
    }

    fn cpu_idle_resume_at_1(&mut self) {
        // ** S1
        self.cpu_step(1);
        if self.cpu_return_to_sync {
            self.cpu_idle_sync_point = 1;
            return;
        }

        // ** S2
        self.cpu_step(1);
        if self.cpu_return_to_sync {
            self.cpu_idle_sync_point = 2;
            return;
        }

        self.cpu.status.interrupt_latch =
            self.cpu.status.interrupt_flag.x() & self.cpu.status.interrupt_enable;
        // ** S3
        self.cpu_step(1);
        if self.cpu_return_to_sync {
            self.cpu_idle_sync_point = 3;
            return;
        }

        // ** S4
        self.cpu_step(1);
        if self.cpu_return_to_sync {
            self.cpu_idle_sync_point = 4;
            return;
        }

        self.cpu_idle_sync_point = 0;
    }

    fn cpu_idle_resume_at_2(&mut self) {
        // ** S2
        self.cpu_step(1);
        if self.cpu_return_to_sync {
            self.cpu_idle_sync_point = 2;
            return;
        }

        self.cpu.status.interrupt_latch =
            self.cpu.status.interrupt_flag.x() & self.cpu.status.interrupt_enable;
        // ** S3
        self.cpu_step(1);
        if self.cpu_return_to_sync {
            self.cpu_idle_sync_point = 3;
            return;
        }

        // ** S4
        self.cpu_step(1);
        if self.cpu_return_to_sync {
            self.cpu_idle_sync_point = 4;
            return;
        }

        self.cpu_idle_sync_point = 0;
    }

    fn cpu_idle_resume_at_3(&mut self) {
        // ** S3
        self.cpu_step(1);
        if self.cpu_return_to_sync {
            self.cpu_idle_sync_point = 3;
            return;
        }

        // ** S4
        self.cpu_step(1);
        if self.cpu_return_to_sync {
            self.cpu_idle_sync_point = 4;
            return;
        }

        self.cpu_idle_sync_point = 0;
    }

    fn cpu_idle_resume_at_4(&mut self) {
        // ** S4
        self.cpu_step(1);
        if self.cpu_return_to_sync {
            self.cpu_idle_sync_point = 4;
            return;
        }

        self.cpu_idle_sync_point = 0;
    }

    pub fn cpu_read(&mut self, address: u16) -> u8 {
        match self.cpu_read_sync_point {
            0 => self.cpu_read_fresh(address),
            1 => self.cpu_read_resume_at_1(address),
            2 => self.cpu_read_resume_at_2(address),
            3 => self.cpu_read_resume_at_3(address),
            4 => self.cpu_read_resume_at_4(address),
            _ => panic!(),
        }
    }

    fn cpu_read_fresh(&mut self, address: u16) -> u8 {
        self.cpu_read_data = 0xff;
        if self.cpu.r.ei {
            self.cpu.r.ei = false;
            self.cpu.r.ime = true
        };
        self.cpu_read_data &= self.bus_read_with_cycle(0, address, self.cpu_read_data);

        // ** S1
        self.cpu_step(1);
        if self.cpu_return_to_sync {
            self.cpu_read_sync_point = 1;
            return 0;
        }

        self.cpu_read_data &= self.bus_read_with_cycle(1, address, self.cpu_read_data);

        // ** S2
        self.cpu_step(1);
        if self.cpu_return_to_sync {
            self.cpu_read_sync_point = 2;
            return 0;
        }

        self.cpu_read_data &= self.bus_read_with_cycle(2, address, self.cpu_read_data);
        self.cpu.status.interrupt_latch =
            self.cpu.status.interrupt_flag.x() & self.cpu.status.interrupt_enable;

        // ** S3
        self.cpu_step(1);
        if self.cpu_return_to_sync {
            self.cpu_read_sync_point = 3;
            return 0;
        }

        self.cpu_read_data &= self.bus_read_with_cycle(3, address, self.cpu_read_data);

        // ** S4
        self.cpu_step(1);
        if self.cpu_return_to_sync {
            self.cpu_read_sync_point = 4;
            return 0;
        }

        self.cpu_read_data &= self.bus_read_with_cycle(4, address, self.cpu_read_data);
        self.cpu_read_sync_point = 0;
        self.cpu_read_data
    }

    fn cpu_read_resume_at_1(&mut self, address: u16) -> u8 {
        // ** S1
        self.cpu_step(1);
        if self.cpu_return_to_sync {
            self.cpu_read_sync_point = 1;
            return 0;
        }

        self.cpu_read_data &= self.bus_read_with_cycle(1, address, self.cpu_read_data);

        // ** S2
        self.cpu_step(1);
        if self.cpu_return_to_sync {
            self.cpu_read_sync_point = 2;
            return 0;
        }

        self.cpu_read_data &= self.bus_read_with_cycle(2, address, self.cpu_read_data);
        self.cpu.status.interrupt_latch =
            self.cpu.status.interrupt_flag.x() & self.cpu.status.interrupt_enable;

        // ** S3
        self.cpu_step(1);
        if self.cpu_return_to_sync {
            self.cpu_read_sync_point = 3;
            return 0;
        }

        self.cpu_read_data &= self.bus_read_with_cycle(3, address, self.cpu_read_data);

        // ** S4
        self.cpu_step(1);
        if self.cpu_return_to_sync {
            self.cpu_read_sync_point = 4;
            return 0;
        }

        self.cpu_read_data &= self.bus_read_with_cycle(4, address, self.cpu_read_data);
        self.cpu_read_sync_point = 0;
        self.cpu_read_data
    }

    fn cpu_read_resume_at_2(&mut self, address: u16) -> u8 {
        // ** S2
        self.cpu_step(1);
        if self.cpu_return_to_sync {
            self.cpu_read_sync_point = 2;
            return 0;
        }

        self.cpu_read_data &= self.bus_read_with_cycle(2, address, self.cpu_read_data);
        self.cpu.status.interrupt_latch =
            self.cpu.status.interrupt_flag.x() & self.cpu.status.interrupt_enable;

        // ** S3
        self.cpu_step(1);
        if self.cpu_return_to_sync {
            self.cpu_read_sync_point = 3;
            return 0;
        }

        self.cpu_read_data &= self.bus_read_with_cycle(3, address, self.cpu_read_data);

        // ** S4
        self.cpu_step(1);
        if self.cpu_return_to_sync {
            self.cpu_read_sync_point = 4;
            return 0;
        }

        self.cpu_read_data &= self.bus_read_with_cycle(4, address, self.cpu_read_data);
        self.cpu_read_sync_point = 0;
        self.cpu_read_data
    }

    fn cpu_read_resume_at_3(&mut self, address: u16) -> u8 {
        // ** S3
        self.cpu_step(1);
        if self.cpu_return_to_sync {
            self.cpu_read_sync_point = 3;
            return 0;
        }

        self.cpu_read_data &= self.bus_read_with_cycle(3, address, self.cpu_read_data);

        // ** S4
        self.cpu_step(1);
        if self.cpu_return_to_sync {
            self.cpu_read_sync_point = 4;
            return 0;
        }

        self.cpu_read_data &= self.bus_read_with_cycle(4, address, self.cpu_read_data);
        self.cpu_read_sync_point = 0;
        self.cpu_read_data
    }

    fn cpu_read_resume_at_4(&mut self, address: u16) -> u8 {
        // ** S4
        self.cpu_step(1);
        if self.cpu_return_to_sync {
            self.cpu_read_sync_point = 4;
            return 0;
        }

        self.cpu_read_data &= self.bus_read_with_cycle(4, address, self.cpu_read_data);
        self.cpu_read_sync_point = 0;
        self.cpu_read_data
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
