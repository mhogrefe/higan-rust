//TODO test

use ares::gb::cpu::cpu::CPU;

impl CPU {
    pub fn cycle_edge(&mut self) {
        if self.processor.r.ei {
            self.processor.r.ei = false;
            self.processor.r.ime = true;
        }
    }
}
