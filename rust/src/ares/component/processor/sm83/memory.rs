use ares::gb::bus::Bus;
use malachite_base::num::conversion::traits::WrappingFrom;

// See higan-rust/cpp/ares/component/processor/sm83/memory.cpp
impl Bus {
    pub fn cpu_operand(&mut self) -> u8 {
        if self.cpu.r.halt_bug {
            self.cpu.r.halt_bug = false;
            self.cpu_read(self.cpu.r.get_pc())
        } else {
            let pc = self.cpu.r.post_increment_pc();
            self.cpu_read(pc)
        }
    }

    pub fn cpu_operands(&mut self) -> u16 {
        let data = u16::from(self.cpu_operand());
        data | u16::from(self.cpu_operand()) << 8
    }

    pub fn cpu_load(&mut self, mut address: u16) -> u16 {
        let data = u16::from(self.cpu_read(address));
        address += 1;
        data | u16::from(self.cpu_read(address)) << 8
    }

    pub fn cpu_store(&mut self, mut address: u16, data: u16) {
        self.cpu_write(address, u8::wrapping_from(data));
        address += 1;
        self.cpu_write(address, u8::wrapping_from(data >> 8));
    }

    pub fn cpu_pop(&mut self) -> u16 {
        let sp = self.cpu.r.post_increment_sp();
        let data = u16::from(self.cpu_read(sp));
        let sp = self.cpu.r.post_increment_sp();
        data | u16::from(self.cpu_read(sp)) << 8
    }

    pub fn cpu_push(&mut self, data: u16) {
        let sp = self.cpu.r.pre_decrement_sp();
        self.cpu_write(sp, u8::wrapping_from(data));
        let sp = self.cpu.r.pre_decrement_sp();
        self.cpu_write(sp, u8::wrapping_from(data));
    }
}
