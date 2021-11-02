use ares::gb::system::System;
use ares::platform::Platform;
use malachite_base::num::conversion::traits::WrappingFrom;

// See higan-rust/cpp/ares/component/processor/sm83/memory.cpp
impl<P: Platform> System<P> {
    pub fn s_cpu_operand(&mut self) -> u8 {
        if self.cpu.r.halt_bug {
            self.cpu.r.halt_bug = false;
            self.s_cpu_read(self.cpu.r.get_pc())
        } else {
            let pc = self.cpu.r.post_increment_pc();
            self.s_cpu_read(pc)
        }
    }

    pub fn s_cpu_operands(&mut self) -> u16 {
        let data = u16::from(self.s_cpu_operand());
        data | u16::from(self.s_cpu_operand()) << 8
    }

    pub fn s_cpu_load(&mut self, mut address: u16) -> u16 {
        let data = u16::from(self.s_cpu_read(address));
        address += 1;
        data | u16::from(self.s_cpu_read(address)) << 8
    }

    pub fn s_cpu_store(&mut self, mut address: u16, data: u16) {
        self.s_cpu_write(address, u8::wrapping_from(data));
        address += 1;
        self.s_cpu_write(address, u8::wrapping_from(data >> 8));
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
