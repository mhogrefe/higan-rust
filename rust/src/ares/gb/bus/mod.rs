use ares::gb::system::{System, ThreadState};
use ares::platform::Platform;

// See higan-rust/cpp/ares/gb/bus/bus.cpp
impl<P: Platform> System<P> {
    pub fn bus_read_with_cycle(&mut self, cycle: u32, address: u16, mut data: u8) -> u8 {
        data &= self.cpu_read_io(cycle, address, data);
        data &= self.apu.read_io(cycle, address, data);
        data &= self.ppu_read_io(cycle, address, data);
        // data &= cartridge.read(cycle, address, data);
        data
    }

    //TODO sync
    pub fn s_bus_write_with_cycle(&mut self, cycle: u32, address: u16, data: u8) {
        self.s_cpu_write_io(cycle, address, data);
        self.apu.write_io(cycle, address, data);
        //ppu.writeIO(cycle, address, data);
        //cartridge.write(cycle, address, data);
    }

    pub fn bus_read(&mut self, address: u16, mut data: u8) -> u8 {
        data &= self.bus_read_with_cycle(2, address, data);
        data &= self.bus_read_with_cycle(4, address, data);
        data
    }

    // synchronized
    pub fn s_bus_write(&mut self, address: u16, data: u8) {
        let sync_point = if self.cpu_thread_state == ThreadState::Resuming {
            self.cpu_sync_points.pop()
        } else {
            0
        };
        match sync_point {
            0 | 1 => self.s_bus_write_fresh(address, data),
            2 => self.s_bus_write_resume_at_2(address, data),
            _ => panic!(),
        }
    }

    fn s_bus_write_fresh(&mut self, address: u16, data: u8) {
        // ** S1
        self.s_bus_write_with_cycle(2, address, data);
        if self.cpu_thread_state == ThreadState::Pausing {
            self.cpu_sync_points.push(1);
            return;
        }

        // ** S2
        self.s_bus_write_with_cycle(4, address, data);
        if self.cpu_thread_state == ThreadState::Pausing {
            self.cpu_sync_points.push(2);
        }
    }

    fn s_bus_write_resume_at_2(&mut self, address: u16, data: u8) {
        // ** S2
        self.s_bus_write_with_cycle(4, address, data);
        if self.cpu_thread_state == ThreadState::Pausing {
            self.cpu_sync_points.push(2);
        }
    }
}
