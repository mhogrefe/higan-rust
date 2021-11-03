use ares::gb::system::System;
use ares::platform::Platform;
use malachite_base::num::conversion::traits::WrappingFrom;

// See higan-rust/cpp/ares/component/processor/sm83/memory.cpp
impl<P: Platform> System<P> {
    // synchronized
    pub fn s_cpu_operand(&mut self) -> u8 {
        let sync_point = if self.cpu_resuming_after_sync {
            self.cpu_sync_points.pop()
        } else {
            0
        };
        match sync_point {
            0 => self.s_cpu_operand_fresh(),
            1 => self.s_cpu_operand_resume_at_1(),
            2 => self.s_cpu_operand_resume_at_2(),
            _ => panic!(),
        }
    }

    fn s_cpu_operand_fresh(&mut self) -> u8 {
        if self.cpu.r.halt_bug {
            self.cpu.r.halt_bug = false;
            // ** S1
            let r = self.s_cpu_read(self.cpu.r.get_pc());
            if self.cpu_return_to_sync {
                self.cpu_sync_points.push(1);
                return 0;
            }
            r
        } else {
            let pc = self.cpu.r.post_increment_pc();
            // ** S2
            let r = self.s_cpu_read(pc);
            if self.cpu_return_to_sync {
                self.cpu_sync_points.push(2);
                self.cpu_local_u16s.push(pc);
                return 0;
            }
            r
        }
    }

    fn s_cpu_operand_resume_at_1(&mut self) -> u8 {
        // ** S1
        let r = self.s_cpu_read(self.cpu.r.get_pc());
        if self.cpu_return_to_sync {
            self.cpu_sync_points.push(1);
            return 0;
        }
        r
    }

    fn s_cpu_operand_resume_at_2(&mut self) -> u8 {
        // ** S2
        let pc = self.cpu_local_u16s.pop();
        let r = self.s_cpu_read(pc);
        if self.cpu_return_to_sync {
            self.cpu_sync_points.push(2);
            self.cpu_local_u16s.push(pc);
            return 0;
        }
        r
    }

    // synchronized
    pub fn s_cpu_operands(&mut self) -> u16 {
        let sync_point = if self.cpu_resuming_after_sync {
            self.cpu_sync_points.pop()
        } else {
            0
        };
        match sync_point {
            0 | 1 => self.s_cpu_operands_fresh(),
            2 => self.s_cpu_operands_resume_at_2(),
            _ => panic!(),
        }
    }

    fn s_cpu_operands_fresh(&mut self) -> u16 {
        // ** S1
        let op = self.s_cpu_operand();
        if self.cpu_return_to_sync {
            self.cpu_sync_points.push(1);
            return 0;
        }
        let data = u16::from(op);

        // ** S2
        let op = self.s_cpu_operand();
        if self.cpu_return_to_sync {
            self.cpu_sync_points.push(2);
            self.cpu_local_u16s.push(data);
            return 0;
        }
        data | u16::from(op) << 8
    }

    fn s_cpu_operands_resume_at_2(&mut self) -> u16 {
        // ** S2
        let op = self.s_cpu_operand();
        if self.cpu_return_to_sync {
            self.cpu_sync_points.push(2);
            return 0;
        }
        let data = self.cpu_local_u16s.pop();
        data | u16::from(op) << 8
    }

    // synchronized
    pub fn s_cpu_load(&mut self, address: u16) -> u16 {
        let sync_point = if self.cpu_resuming_after_sync {
            self.cpu_sync_points.pop()
        } else {
            0
        };
        match sync_point {
            0 | 1 => self.s_cpu_load_fresh(address),
            2 => self.s_cpu_load_resume_at_2(),
            _ => panic!(),
        }
    }

    fn s_cpu_load_fresh(&mut self, address: u16) -> u16 {
        // ** S1
        let r = self.s_cpu_read(address);
        if self.cpu_return_to_sync {
            self.cpu_sync_points.push(1);
            return 0;
        }

        let data = u16::from(r);
        let next_address = address + 1;

        // ** S2
        let r = self.s_cpu_read(next_address);
        if self.cpu_return_to_sync {
            self.cpu_sync_points.push(2);
            self.cpu_local_u16s.push(data);
            self.cpu_local_u16s.push(next_address);
            return 0;
        }

        data | u16::from(r) << 8
    }

    fn s_cpu_load_resume_at_2(&mut self) -> u16 {
        // ** S2
        let next_address = self.cpu_local_u16s.pop();
        let r = self.s_cpu_read(next_address);
        if self.cpu_return_to_sync {
            self.cpu_sync_points.push(2);
            self.cpu_local_u16s.push(next_address);
            return 0;
        }
        let data = self.cpu_local_u16s.pop();
        data | u16::from(r) << 8
    }

    // synchronized
    pub fn s_cpu_store(&mut self, address: u16, data: u16) {
        let sync_point = if self.cpu_resuming_after_sync {
            self.cpu_sync_points.pop()
        } else {
            0
        };
        match sync_point {
            0 | 1 => self.s_cpu_store_fresh(address, data),
            2 => self.s_cpu_store_resume_at_2(data),
            _ => panic!(),
        }
    }

    fn s_cpu_store_fresh(&mut self, address: u16, data: u16) {
        // ** S1
        self.s_cpu_write(address, u8::wrapping_from(data));
        if self.cpu_return_to_sync {
            self.cpu_sync_points.push(1);
            return;
        }

        let next_address = address + 1;
        // ** S2
        self.s_cpu_write(next_address, u8::wrapping_from(data >> 8));
        if self.cpu_return_to_sync {
            self.cpu_sync_points.push(2);
            self.cpu_local_u16s.push(next_address);
        }
    }

    fn s_cpu_store_resume_at_2(&mut self, data: u16) {
        // ** S2
        let next_address = self.cpu_local_u16s.pop();
        self.s_cpu_write(next_address, u8::wrapping_from(data >> 8));
        if self.cpu_return_to_sync {
            self.cpu_sync_points.push(2);
            self.cpu_local_u16s.push(next_address);
        }
    }

    pub fn s_cpu_pop(&mut self) -> u16 {
        let sp = self.cpu.r.post_increment_sp();
        let data = u16::from(self.s_cpu_read(sp));
        let sp = self.cpu.r.post_increment_sp();
        data | u16::from(self.s_cpu_read(sp)) << 8
    }

    pub fn s_cpu_push(&mut self, data: u16) {
        let sp = self.cpu.r.pre_decrement_sp();
        self.s_cpu_write(sp, u8::wrapping_from(data));
        let sp = self.cpu.r.pre_decrement_sp();
        self.s_cpu_write(sp, u8::wrapping_from(data));
    }
}
