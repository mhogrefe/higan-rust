use ares::gb::cpu::cpu::CPUIO;

impl CPUIO {
    pub fn wram_address(&self, addr: u16) -> u32 {
        let addr = u32::from(addr & 0x1fff);
        if addr < 0x1000 {
            addr
        } else {
            let bank = u32::from(
                self.status.wram_bank.x() + if self.status.wram_bank.x() == 0 { 1 } else { 0 },
            );
            (bank * 0x1000) + (addr & 0x0fff)
        }
    }
}
