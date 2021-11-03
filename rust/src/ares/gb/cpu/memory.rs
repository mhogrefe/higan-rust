use ares::emulator::types::U13;
use ares::gb::system::{Model, System};
use ares::platform::Platform;

// See higan-rust/cpp/ares/gb/cpu/memory.cpp
impl<P: Platform> System<P> {
    // synchronized
    pub fn s_cpu_stop(&mut self) {
        self.s_cpu_idle();
        if self.cpu_return_to_sync {
            return;
        }
        if self.cpu.model == Model::SuperGameBoy {
            //TODO scheduler.exit(Event::Step);
        }
    }

    // synchronized
    pub fn s_cpu_halt(&mut self) {
        self.s_cpu_idle();
        if self.cpu_return_to_sync {
            return;
        }
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

    // synchronized
    pub fn s_cpu_idle(&mut self) {
        let sync_point = if self.cpu_resuming_after_sync {
            self.cpu_sync_points.pop()
        } else {
            0
        };
        match sync_point {
            0 => self.s_cpu_idle_fresh(),
            1 => self.s_cpu_idle_resume_at_1(),
            2 => self.s_cpu_idle_resume_at_2(),
            3 => self.s_cpu_idle_resume_at_3(),
            4 => self.s_cpu_idle_resume_at_4(),
            _ => panic!(),
        }
    }

    fn s_cpu_idle_fresh(&mut self) {
        if self.cpu.r.ei {
            self.cpu.r.ei = false;
            self.cpu.r.ime = true
        };
        // ** S1
        self.s_cpu_step(1);
        if self.cpu_return_to_sync {
            self.cpu_sync_points.push(1);
            return;
        }

        // ** S2
        self.s_cpu_step(1);
        if self.cpu_return_to_sync {
            self.cpu_sync_points.push(2);
            return;
        }

        self.cpu.status.interrupt_latch =
            self.cpu.status.interrupt_flag.x() & self.cpu.status.interrupt_enable;
        // ** S3
        self.s_cpu_step(1);
        if self.cpu_return_to_sync {
            self.cpu_sync_points.push(3);
            return;
        }

        // ** S4
        self.s_cpu_step(1);
        if self.cpu_return_to_sync {
            self.cpu_sync_points.push(4);
        }
    }

    fn s_cpu_idle_resume_at_1(&mut self) {
        // ** S1
        self.s_cpu_step(1);
        if self.cpu_return_to_sync {
            self.cpu_sync_points.push(1);
            return;
        }

        // ** S2
        self.s_cpu_step(1);
        if self.cpu_return_to_sync {
            self.cpu_sync_points.push(2);
            return;
        }

        self.cpu.status.interrupt_latch =
            self.cpu.status.interrupt_flag.x() & self.cpu.status.interrupt_enable;
        // ** S3
        self.s_cpu_step(1);
        if self.cpu_return_to_sync {
            self.cpu_sync_points.push(3);
            return;
        }

        // ** S4
        self.s_cpu_step(1);
        if self.cpu_return_to_sync {
            self.cpu_sync_points.push(4);
        }
    }

    fn s_cpu_idle_resume_at_2(&mut self) {
        // ** S2
        self.s_cpu_step(1);
        if self.cpu_return_to_sync {
            self.cpu_sync_points.push(2);
            return;
        }

        self.cpu.status.interrupt_latch =
            self.cpu.status.interrupt_flag.x() & self.cpu.status.interrupt_enable;
        // ** S3
        self.s_cpu_step(1);
        if self.cpu_return_to_sync {
            self.cpu_sync_points.push(3);
            return;
        }

        // ** S4
        self.s_cpu_step(1);
        if self.cpu_return_to_sync {
            self.cpu_sync_points.push(4);
        }
    }

    fn s_cpu_idle_resume_at_3(&mut self) {
        // ** S3
        self.s_cpu_step(1);
        if self.cpu_return_to_sync {
            self.cpu_sync_points.push(3);
            return;
        }

        // ** S4
        self.s_cpu_step(1);
        if self.cpu_return_to_sync {
            self.cpu_sync_points.push(4);
        }
    }

    fn s_cpu_idle_resume_at_4(&mut self) {
        // ** S4
        self.s_cpu_step(1);
        if self.cpu_return_to_sync {
            self.cpu_sync_points.push(4);
        }
    }

    // synchronized
    pub fn s_cpu_read(&mut self, address: u16) -> u8 {
        let sync_point = if self.cpu_resuming_after_sync {
            self.cpu_sync_points.pop()
        } else {
            0
        };
        match sync_point {
            0 => self.s_cpu_read_fresh(address),
            1 => self.s_cpu_read_resume_at_1(address),
            2 => self.s_cpu_read_resume_at_2(address),
            3 => self.s_cpu_read_resume_at_3(address),
            4 => self.s_cpu_read_resume_at_4(address),
            _ => panic!(),
        }
    }

    fn s_cpu_read_fresh(&mut self, address: u16) -> u8 {
        let mut data = 0xff;
        if self.cpu.r.ei {
            self.cpu.r.ei = false;
            self.cpu.r.ime = true
        };
        data &= self.bus_read_with_cycle(0, address, data);

        // ** S1
        self.s_cpu_step(1);
        if self.cpu_return_to_sync {
            self.cpu_sync_points.push(1);
            self.cpu_local_u8s.push(data);
            return 0;
        }

        data &= self.bus_read_with_cycle(1, address, data);

        // ** S2
        self.s_cpu_step(1);
        if self.cpu_return_to_sync {
            self.cpu_sync_points.push(2);
            self.cpu_local_u8s.push(data);
            return 0;
        }

        data &= self.bus_read_with_cycle(2, address, data);
        self.cpu.status.interrupt_latch =
            self.cpu.status.interrupt_flag.x() & self.cpu.status.interrupt_enable;

        // ** S3
        self.s_cpu_step(1);
        if self.cpu_return_to_sync {
            self.cpu_sync_points.push(3);
            self.cpu_local_u8s.push(data);
            return 0;
        }

        data &= self.bus_read_with_cycle(3, address, data);

        // ** S4
        self.s_cpu_step(1);
        if self.cpu_return_to_sync {
            self.cpu_sync_points.push(4);
            self.cpu_local_u8s.push(data);
            return 0;
        }

        data &= self.bus_read_with_cycle(4, address, data);
        data
    }

    fn s_cpu_read_resume_at_1(&mut self, address: u16) -> u8 {
        // ** S1
        self.s_cpu_step(1);
        if self.cpu_return_to_sync {
            self.cpu_sync_points.push(1);
            return 0;
        }

        let mut data = self.cpu_local_u8s.pop();
        data &= self.bus_read_with_cycle(1, address, data);

        // ** S2
        self.s_cpu_step(1);
        if self.cpu_return_to_sync {
            self.cpu_sync_points.push(2);
            self.cpu_local_u8s.push(data);
            return 0;
        }

        data &= self.bus_read_with_cycle(2, address, data);
        self.cpu.status.interrupt_latch =
            self.cpu.status.interrupt_flag.x() & self.cpu.status.interrupt_enable;

        // ** S3
        self.s_cpu_step(1);
        if self.cpu_return_to_sync {
            self.cpu_sync_points.push(3);
            self.cpu_local_u8s.push(data);
            return 0;
        }

        data &= self.bus_read_with_cycle(3, address, data);

        // ** S4
        self.s_cpu_step(1);
        if self.cpu_return_to_sync {
            self.cpu_sync_points.push(4);
            self.cpu_local_u8s.push(data);
            return 0;
        }

        data &= self.bus_read_with_cycle(4, address, data);
        data
    }

    fn s_cpu_read_resume_at_2(&mut self, address: u16) -> u8 {
        // ** S2
        self.s_cpu_step(1);
        if self.cpu_return_to_sync {
            self.cpu_sync_points.push(2);
            return 0;
        }

        let mut data = self.cpu_local_u8s.pop();
        data &= self.bus_read_with_cycle(2, address, data);
        self.cpu.status.interrupt_latch =
            self.cpu.status.interrupt_flag.x() & self.cpu.status.interrupt_enable;

        // ** S3
        self.s_cpu_step(1);
        if self.cpu_return_to_sync {
            self.cpu_sync_points.push(3);
            self.cpu_local_u8s.push(data);
            return 0;
        }

        data &= self.bus_read_with_cycle(3, address, data);

        // ** S4
        self.s_cpu_step(1);
        if self.cpu_return_to_sync {
            self.cpu_sync_points.push(4);
            self.cpu_local_u8s.push(data);
            return 0;
        }

        data &= self.bus_read_with_cycle(4, address, data);
        data
    }

    fn s_cpu_read_resume_at_3(&mut self, address: u16) -> u8 {
        // ** S3
        self.s_cpu_step(1);
        if self.cpu_return_to_sync {
            self.cpu_sync_points.push(3);
            return 0;
        }

        let mut data = self.cpu_local_u8s.pop();
        data &= self.bus_read_with_cycle(3, address, data);

        // ** S4
        self.s_cpu_step(1);
        if self.cpu_return_to_sync {
            self.cpu_sync_points.push(4);
            self.cpu_local_u8s.push(data);
        }

        data &= self.bus_read_with_cycle(4, address, data);
        data
    }

    fn s_cpu_read_resume_at_4(&mut self, address: u16) -> u8 {
        // ** S4
        self.s_cpu_step(1);
        if self.cpu_return_to_sync {
            self.cpu_sync_points.push(4);
            return 0;
        }

        let mut data = self.cpu_local_u8s.pop();
        data &= self.bus_read_with_cycle(4, address, data);
        data
    }

    // synchronized
    pub fn s_cpu_write(&mut self, address: u16, data: u8) {
        let sync_point = if self.cpu_resuming_after_sync {
            self.cpu_sync_points.pop()
        } else {
            0
        };
        match sync_point {
            0 => self.s_cpu_write_fresh(address, data),
            1 => self.s_cpu_write_fresh_resume_at_1(address, data),
            2 => self.s_cpu_write_fresh_resume_at_2(address, data),
            3 => self.s_cpu_write_fresh_resume_at_3(address, data),
            4 => self.s_cpu_write_fresh_resume_at_4(address, data),
            5 => self.s_cpu_write_fresh_resume_at_5(address, data),
            6 => self.s_cpu_write_fresh_resume_at_6(address, data),
            7 => self.s_cpu_write_fresh_resume_at_7(address, data),
            8 => self.s_cpu_write_fresh_resume_at_8(address, data),
            9 => self.s_cpu_write_fresh_resume_at_9(address, data),
            _ => panic!(),
        }
    }

    fn s_cpu_write_fresh(&mut self, address: u16, data: u8) {
        if self.cpu.r.ei {
            self.cpu.r.ei = false;
            self.cpu.r.ime = true
        };
        // ** S1
        self.s_bus_write_with_cycle(0, address, data);
        if self.cpu_return_to_sync {
            self.cpu_sync_points.push(1);
            return;
        }

        // ** S2
        self.s_cpu_step(1);
        if self.cpu_return_to_sync {
            self.cpu_sync_points.push(2);
            return;
        }

        // ** S3
        self.s_bus_write_with_cycle(1, address, data);
        if self.cpu_return_to_sync {
            self.cpu_sync_points.push(3);
            return;
        }

        // ** S4
        self.s_cpu_step(1);
        if self.cpu_return_to_sync {
            self.cpu_sync_points.push(4);
            return;
        }

        // ** S5
        self.s_bus_write_with_cycle(2, address, data);
        if self.cpu_return_to_sync {
            self.cpu_sync_points.push(5);
            return;
        }

        // ** S6
        self.s_cpu_step(1);
        if self.cpu_return_to_sync {
            self.cpu_sync_points.push(6);
            return;
        }

        // ** S7
        self.s_bus_write_with_cycle(3, address, data);
        if self.cpu_return_to_sync {
            self.cpu_sync_points.push(7);
            return;
        }

        // ** S8
        self.s_cpu_step(1);
        if self.cpu_return_to_sync {
            self.cpu_sync_points.push(8);
            return;
        }

        // ** S9
        self.s_bus_write_with_cycle(4, address, data);
        if self.cpu_return_to_sync {
            self.cpu_sync_points.push(9);
            return;
        }

        self.cpu.status.interrupt_latch =
            self.cpu.status.interrupt_flag.x() & self.cpu.status.interrupt_enable;
    }

    fn s_cpu_write_fresh_resume_at_1(&mut self, address: u16, data: u8) {
        // ** S1
        self.s_bus_write_with_cycle(0, address, data);
        if self.cpu_return_to_sync {
            self.cpu_sync_points.push(1);
            return;
        }

        // ** S2
        self.s_cpu_step(1);
        if self.cpu_return_to_sync {
            self.cpu_sync_points.push(2);
            return;
        }

        // ** S3
        self.s_bus_write_with_cycle(1, address, data);
        if self.cpu_return_to_sync {
            self.cpu_sync_points.push(3);
            return;
        }

        // ** S4
        self.s_cpu_step(1);
        if self.cpu_return_to_sync {
            self.cpu_sync_points.push(4);
            return;
        }

        // ** S5
        self.s_bus_write_with_cycle(2, address, data);
        if self.cpu_return_to_sync {
            self.cpu_sync_points.push(5);
            return;
        }

        // ** S6
        self.s_cpu_step(1);
        if self.cpu_return_to_sync {
            self.cpu_sync_points.push(6);
            return;
        }

        // ** S7
        self.s_bus_write_with_cycle(3, address, data);
        if self.cpu_return_to_sync {
            self.cpu_sync_points.push(7);
            return;
        }

        // ** S8
        self.s_cpu_step(1);
        if self.cpu_return_to_sync {
            self.cpu_sync_points.push(8);
            return;
        }

        // ** S9
        self.s_bus_write_with_cycle(4, address, data);
        if self.cpu_return_to_sync {
            self.cpu_sync_points.push(9);
            return;
        }

        self.cpu.status.interrupt_latch =
            self.cpu.status.interrupt_flag.x() & self.cpu.status.interrupt_enable;
    }

    fn s_cpu_write_fresh_resume_at_2(&mut self, address: u16, data: u8) {
        // ** S2
        self.s_cpu_step(1);
        if self.cpu_return_to_sync {
            self.cpu_sync_points.push(2);
            return;
        }

        // ** S3
        self.s_bus_write_with_cycle(1, address, data);
        if self.cpu_return_to_sync {
            self.cpu_sync_points.push(3);
            return;
        }

        // ** S4
        self.s_cpu_step(1);
        if self.cpu_return_to_sync {
            self.cpu_sync_points.push(4);
            return;
        }

        // ** S5
        self.s_bus_write_with_cycle(2, address, data);
        if self.cpu_return_to_sync {
            self.cpu_sync_points.push(5);
            return;
        }

        // ** S6
        self.s_cpu_step(1);
        if self.cpu_return_to_sync {
            self.cpu_sync_points.push(6);
            return;
        }

        // ** S7
        self.s_bus_write_with_cycle(3, address, data);
        if self.cpu_return_to_sync {
            self.cpu_sync_points.push(7);
            return;
        }

        // ** S8
        self.s_cpu_step(1);
        if self.cpu_return_to_sync {
            self.cpu_sync_points.push(8);
            return;
        }

        // ** S9
        self.s_bus_write_with_cycle(4, address, data);
        if self.cpu_return_to_sync {
            self.cpu_sync_points.push(9);
            return;
        }

        self.cpu.status.interrupt_latch =
            self.cpu.status.interrupt_flag.x() & self.cpu.status.interrupt_enable;
    }

    fn s_cpu_write_fresh_resume_at_3(&mut self, address: u16, data: u8) {
        // ** S3
        self.s_bus_write_with_cycle(1, address, data);
        if self.cpu_return_to_sync {
            self.cpu_sync_points.push(3);
            return;
        }

        // ** S4
        self.s_cpu_step(1);
        if self.cpu_return_to_sync {
            self.cpu_sync_points.push(4);
            return;
        }

        // ** S5
        self.s_bus_write_with_cycle(2, address, data);
        if self.cpu_return_to_sync {
            self.cpu_sync_points.push(5);
            return;
        }

        // ** S6
        self.s_cpu_step(1);
        if self.cpu_return_to_sync {
            self.cpu_sync_points.push(6);
            return;
        }

        // ** S7
        self.s_bus_write_with_cycle(3, address, data);
        if self.cpu_return_to_sync {
            self.cpu_sync_points.push(7);
            return;
        }

        // ** S8
        self.s_cpu_step(1);
        if self.cpu_return_to_sync {
            self.cpu_sync_points.push(8);
            return;
        }

        // ** S9
        self.s_bus_write_with_cycle(4, address, data);
        if self.cpu_return_to_sync {
            self.cpu_sync_points.push(9);
            return;
        }

        self.cpu.status.interrupt_latch =
            self.cpu.status.interrupt_flag.x() & self.cpu.status.interrupt_enable;
    }

    fn s_cpu_write_fresh_resume_at_4(&mut self, address: u16, data: u8) {
        // ** S4
        self.s_cpu_step(1);
        if self.cpu_return_to_sync {
            self.cpu_sync_points.push(4);
            return;
        }

        // ** S5
        self.s_bus_write_with_cycle(2, address, data);
        if self.cpu_return_to_sync {
            self.cpu_sync_points.push(5);
            return;
        }

        // ** S6
        self.s_cpu_step(1);
        if self.cpu_return_to_sync {
            self.cpu_sync_points.push(6);
            return;
        }

        // ** S7
        self.s_bus_write_with_cycle(3, address, data);
        if self.cpu_return_to_sync {
            self.cpu_sync_points.push(7);
            return;
        }

        // ** S8
        self.s_cpu_step(1);
        if self.cpu_return_to_sync {
            self.cpu_sync_points.push(8);
            return;
        }

        // ** S9
        self.s_bus_write_with_cycle(4, address, data);
        if self.cpu_return_to_sync {
            self.cpu_sync_points.push(9);
            return;
        }

        self.cpu.status.interrupt_latch =
            self.cpu.status.interrupt_flag.x() & self.cpu.status.interrupt_enable;
    }

    fn s_cpu_write_fresh_resume_at_5(&mut self, address: u16, data: u8) {
        // ** S5
        self.s_bus_write_with_cycle(2, address, data);
        if self.cpu_return_to_sync {
            self.cpu_sync_points.push(5);
            return;
        }

        // ** S6
        self.s_cpu_step(1);
        if self.cpu_return_to_sync {
            self.cpu_sync_points.push(6);
            return;
        }

        // ** S7
        self.s_bus_write_with_cycle(3, address, data);
        if self.cpu_return_to_sync {
            self.cpu_sync_points.push(7);
            return;
        }

        // ** S8
        self.s_cpu_step(1);
        if self.cpu_return_to_sync {
            self.cpu_sync_points.push(8);
            return;
        }

        // ** S9
        self.s_bus_write_with_cycle(4, address, data);
        if self.cpu_return_to_sync {
            self.cpu_sync_points.push(9);
            return;
        }

        self.cpu.status.interrupt_latch =
            self.cpu.status.interrupt_flag.x() & self.cpu.status.interrupt_enable;
    }

    fn s_cpu_write_fresh_resume_at_6(&mut self, address: u16, data: u8) {
        // ** S6
        self.s_cpu_step(1);
        if self.cpu_return_to_sync {
            self.cpu_sync_points.push(6);
            return;
        }

        // ** S7
        self.s_bus_write_with_cycle(3, address, data);
        if self.cpu_return_to_sync {
            self.cpu_sync_points.push(7);
            return;
        }

        // ** S8
        self.s_cpu_step(1);
        if self.cpu_return_to_sync {
            self.cpu_sync_points.push(8);
            return;
        }

        // ** S9
        self.s_bus_write_with_cycle(4, address, data);
        if self.cpu_return_to_sync {
            self.cpu_sync_points.push(9);
            return;
        }

        self.cpu.status.interrupt_latch =
            self.cpu.status.interrupt_flag.x() & self.cpu.status.interrupt_enable;
    }

    fn s_cpu_write_fresh_resume_at_7(&mut self, address: u16, data: u8) {
        // ** S7
        self.s_bus_write_with_cycle(3, address, data);
        if self.cpu_return_to_sync {
            self.cpu_sync_points.push(7);
            return;
        }

        // ** S8
        self.s_cpu_step(1);
        if self.cpu_return_to_sync {
            self.cpu_sync_points.push(8);
            return;
        }

        // ** S9
        self.s_bus_write_with_cycle(4, address, data);
        if self.cpu_return_to_sync {
            self.cpu_sync_points.push(9);
            return;
        }

        self.cpu.status.interrupt_latch =
            self.cpu.status.interrupt_flag.x() & self.cpu.status.interrupt_enable;
    }

    fn s_cpu_write_fresh_resume_at_8(&mut self, address: u16, data: u8) {
        // ** S8
        self.s_cpu_step(1);
        if self.cpu_return_to_sync {
            self.cpu_sync_points.push(8);
            return;
        }

        // ** S9
        self.s_bus_write_with_cycle(4, address, data);
        if self.cpu_return_to_sync {
            self.cpu_sync_points.push(9);
            return;
        }

        self.cpu.status.interrupt_latch =
            self.cpu.status.interrupt_flag.x() & self.cpu.status.interrupt_enable;
    }

    fn s_cpu_write_fresh_resume_at_9(&mut self, address: u16, data: u8) {
        // ** S9
        self.s_bus_write_with_cycle(4, address, data);
        if self.cpu_return_to_sync {
            self.cpu_sync_points.push(9);
            return;
        }

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
    // synchronized
    pub fn s_cpu_write_dma(&mut self, address: U13, data: u8) {
        self.s_bus_write(0x8000 | address.x(), data);
    }

    pub fn cpu_read_debugger(&mut self, address: u16) -> u8 {
        self.bus_read(address, 0xff)
    }
}
