use higan::emulator::types::U3;
use higan::gb::cpu::cpu::CPUIO;
use malachite_base::misc::WrappingFrom;

impl CPUIO {
    pub fn wram_address(&self, addr: u16) -> u32 {
        let addr = u32::from(addr & 0x1fff);
        if addr < 0x1000 {
            addr
        } else {
            let bank = u32::from(
                self.status.wram_bank.0 + if self.status.wram_bank.0 == 0 { 1 } else { 0 },
            );
            (bank * 0x1000) + (addr & 0x0fff)
        }
    }

    pub fn read_io(&self, addr: u16) -> u8 {
        match addr {
            0xc000..=0xfdff => self.wram[self.wram_address(addr) as usize],

            0xff80..=0xfffe => self.hram[(addr & 0x7f) as usize],

            //JOYP
            0xff00 => {
                //TODO joypPoll();
                0xc0 | (if self.status.p15 { 1 } else { 0 } << 5)
                    | (if self.status.p14 { 1 } else { 0 } << 4)
                    | (self.status.joyp << 0)
            }

            //SB
            0xff01 => 0x00,

            //SC
            0xff02 => {
                (if self.status.serial_transfer { 1 } else { 0 } << 7)
                    | 0x7e
                    | (if self.status.serial_clock { 1 } else { 0 } << 0)
            }

            //DIV
            0xff04 => u8::wrapping_from(self.status.div >> 8),

            //TIMA
            0xff05 => self.status.tima,

            //TMA
            0xff06 => self.status.tma,

            //TAC
            0xff07 => {
                0xf8 | (if self.status.timer_enable { 1 } else { 0 } << 2)
                    | (u8::wrapping_from(self.status.timer_clock << 0))
            }

            //IF
            0xff0f => {
                0xe0 | (if self.status.interrupt_request_joypad {
                    1
                } else {
                    0
                } << 4)
                    | (if self.status.interrupt_request_serial {
                        1
                    } else {
                        0
                    } << 3)
                    | (if self.status.interrupt_request_timer {
                        1
                    } else {
                        0
                    } << 2)
                    | (if self.status.interrupt_request_stat {
                        1
                    } else {
                        0
                    } << 1)
                    | (if self.status.interrupt_request_vblank {
                        1
                    } else {
                        0
                    } << 0)
            }

            //KEY1
            0xff4d => (if self.status.speed_double { 1 } else { 0 }) << 7,

            //HDMA5
            0xff55 => {
                (if self.status.dma_completed { 1 } else { 0 } << 7)
                    | u8::wrapping_from(((self.status.dma_length / 16) - 1) & 0x7f)
            }

            //RP
            0xff56 => 0x02,

            //???
            0xff6c => 0xfe | self.status.ff6c,

            //SVBK
            0xff70 => self.status.wram_bank.0,

            //???
            0xff72 => self.status.ff72,

            //???
            0xff73 => self.status.ff73,

            //???
            0xff74 => self.status.ff74,

            //???
            0xff75 => 0x8f | self.status.ff75,

            //???
            0xff76 => 0xff,

            //???
            0xff77 => 0xff,

            //IE
            0xffff => {
                0xe0 | (if self.status.interrupt_enable_joypad {
                    1
                } else {
                    0
                } << 4)
                    | (if self.status.interrupt_enable_serial {
                        1
                    } else {
                        0
                    } << 3)
                    | (if self.status.interrupt_enable_timer {
                        1
                    } else {
                        0
                    } << 2)
                    | (if self.status.interrupt_enable_stat {
                        1
                    } else {
                        0
                    } << 1)
                    | (if self.status.interrupt_enable_vblank {
                        1
                    } else {
                        0
                    } << 0)
            }

            _ => 0xff,
        }
    }

    // returns whether to do DMA stuff
    pub fn write_io(&mut self, addr: u16, data: u8) -> bool {
        match addr {
            0xc000..=0xfdff => self.wram[self.wram_address(addr) as usize] = data,

            0xff80..=0xfffe => self.hram[(addr & 0x7f) as usize] = data,

            //JOYP
            0xff00 => {
                self.status.p15 = (data & 0x20) != 0;
                self.status.p14 = (data & 0x10) != 0;
                if self.model_is_super_game_boy {
                    //TODO superGameBoy -> joypWrite(status.p15, status.p14);
                }
            }

            //SB
            0xff01 => self.status.serial_data = data,

            //SC
            0xff02 => {
                self.status.serial_transfer = (data & 0x80) != 0;
                self.status.serial_clock = (data & 0x01) != 0;
                if self.status.serial_transfer {
                    self.status.serial_bits = 8;
                }
            }

            //DIV
            0xff04 => self.status.div = 0,

            //TIMA
            0xff05 => self.status.tima = data,

            //TMA
            0xff06 => self.status.tma = data,

            //TAC
            0xff07 => {
                self.status.timer_enable = (data & 0x04) != 0;
                self.status.timer_clock = u32::from(data & 0x03);
            }

            //IF
            0xff0f => {
                self.status.interrupt_request_joypad = (data & 0x10) != 0;
                self.status.interrupt_request_serial = (data & 0x08) != 0;
                self.status.interrupt_request_timer = (data & 0x04) != 0;
                self.status.interrupt_request_stat = (data & 0x02) != 0;
                self.status.interrupt_request_vblank = (data & 0x01) != 0;
            }

            //KEY1
            0xff4d => self.status.speed_switch = (data & 0x01) != 0,

            //HDMA1
            0xff51 => {
                self.status.dma_source = (self.status.dma_source & 0x00ff) | (u16::from(data) << 8)
            }

            //HDMA2
            0xff52 => {
                self.status.dma_source =
                    (self.status.dma_source & 0xff00) | (u16::from(data) & 0xf0)
            }

            //HDMA3
            0xff53 => {
                self.status.dma_target = (self.status.dma_target & 0x00ff) | (u16::from(data) << 8)
            }

            //HDMA4
            0xff54 => {
                self.status.dma_target =
                    (self.status.dma_target & 0xff00) | (u16::from(data) & 0xf0)
            }

            //HDMA5
            0xff55 => {
                self.status.dma_mode = (data & 0x80) != 0;
                self.status.dma_length = u16::wrapping_from(((data & 0x7f) + 1) * 16);
                self.status.dma_completed = !self.status.dma_mode;
                if !self.status.dma_mode {
                    return true;
                }
            }

            //RP
            0xff56 => {}

            //???
            0xff6c => self.status.ff6c = data & 0x01,

            //???
            0xff72 => self.status.ff72 = data,

            //???
            0xff73 => self.status.ff73 = data,

            //???
            0xff74 => self.status.ff74 = data,

            //???
            0xff75 => self.status.ff75 = data & 0x70,

            //SVBK
            0xff70 => self.status.wram_bank = U3::wrapping_from(data & 0x07),

            //IE
            0xffff => {
                self.status.interrupt_enable_joypad = data & 0x10 != 0;
                self.status.interrupt_enable_serial = data & 0x08 != 0;
                self.status.interrupt_enable_timer = data & 0x04 != 0;
                self.status.interrupt_enable_stat = data & 0x02 != 0;
                self.status.interrupt_enable_vblank = data & 0x01 != 0;
            }
            _ => {}
        }
        false
    }
}
