use ares::gb::bus::Bus;

// See higan-rust/rust/src/ares/component/processor/sm83/instructions.rs
// and higan-rust/cpp/ares/component/processor/sm83/memory.cpp
impl Bus {
    pub fn instruction_adc_direct_data(&mut self, target: &mut u8) {
        let op = self.cpu_operand();
        *target = self.cpu.r.add(*target, op, self.cpu.r.get_cf());
    }

    pub fn instruction_adc_direct_direct(&mut self, target: &mut u8, source: u8) {
        *target = self.cpu.r.add(*target, source, self.cpu.r.get_cf());
    }

    pub fn instruction_adc_direct_indirect(&mut self, target: &mut u8, source: u16) {
        let s = self.cpu_read(source);
        *target = self.cpu.r.add(*target, s, self.cpu.r.get_cf());
    }

    pub fn instruction_add_direct_data(&mut self, target: &mut u8) {
        let op = self.cpu_operand();
        *target = self.cpu.r.add(*target, op, false);
    }

    pub fn instruction_add_direct_direct(&mut self, target: &mut u8, source: u8) {
        *target = self.cpu.r.add(*target, source, false);
    }
}
