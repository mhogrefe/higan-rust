//TODO test

use higan::emulator::types::{U22, U3};
use higan::gb::memory::memory::Bus;
use higan::processor::lr35902::lr35902::LR35902;

#[derive(Clone, Copy, Debug)]
pub enum Interrupt {
    Vblank,
    Stat,
    Timer,
    Serial,
    Joypad,
}

#[derive(Clone, Debug, Default)]
pub struct Status {
    pub clock: U22,

    //$ff00  JOYP
    pub p15: bool,
    pub p14: bool,
    pub joyp: u8,
    pub mlt_req: u8,

    //$ff01  SB
    pub serial_data: u8,
    pub serial_bits: u32,

    //$ff02  SC
    pub serial_transfer: bool,
    pub serial_clock: bool,

    //$ff04  DIV
    pub div: u16,

    //$ff05  TIMA
    pub tima: u8,

    //$ff06  TMA
    pub tma: u8,

    //$ff07  TAC
    pub timer_enable: bool,
    pub timer_clock: u32,

    //$ff0f  IF
    pub interrupt_request_joypad: bool,
    pub interrupt_request_serial: bool,
    pub interrupt_request_timer: bool,
    pub interrupt_request_stat: bool,
    pub interrupt_request_vblank: bool,

    //$ff4d  KEY1
    pub speed_double: bool,
    pub speed_switch: bool,

    //$ff51,$ff52  HDMA1,HDMA2
    pub dma_source: u16,

    //$ff53,$ff54  HDMA3,HDMA4
    pub dma_target: u16,

    //$ff55  HDMA5
    pub dma_mode: bool,
    pub dma_length: u16,
    pub dma_completed: bool,

    //$ff6c  ???
    pub ff6c: u8,

    //$ff70  SVBK
    pub wram_bank: U3,

    //$ff72-$ff75  ???
    pub ff72: u8,
    pub ff73: u8,
    pub ff74: u8,
    pub ff75: u8,

    //$ffff  IE
    pub interrupt_enable_joypad: bool,
    pub interrupt_enable_serial: bool,
    pub interrupt_enable_timer: bool,
    pub interrupt_enable_stat: bool,
    pub interrupt_enable_vblank: bool,
}

pub const CPU_WRAM_SIZE: usize = 32_768; //GB=8192, GBC=32768
pub const CPU_HRAM_SIZE: usize = 128;

#[derive(Clone)]
pub struct CPUIO {
    pub status: Status,
    pub wram: [u8; CPU_WRAM_SIZE],
    pub hram: [u8; CPU_HRAM_SIZE],
}

#[derive(Clone, Default)]
pub struct CPU {
    pub processor: LR35902,
    pub bus: Bus,
}

impl CPU {
    //TODO this is probably a Thread thing
    pub fn interrupt(&self, _: u32) {}

    pub fn set_frequency(&self, _: u32) {}

    pub fn enter(&mut self) {
        loop {
            //TODO scheduler.synchronize();
            self.main();
        }
    }

    pub fn main(&mut self) {
        self.interrupt_test();
        //TODO instruction();
    }

    pub fn raise(&mut self, id: Interrupt) {
        match id {
            Interrupt::Vblank => {
                self.bus.cpu_io.status.interrupt_request_vblank = true;
                if self.bus.cpu_io.status.interrupt_enable_vblank {
                    self.processor.r.halt = false;
                }
            }
            Interrupt::Stat => {
                self.bus.cpu_io.status.interrupt_request_stat = true;
                if self.bus.cpu_io.status.interrupt_enable_stat {
                    self.processor.r.halt = false;
                }
            }
            Interrupt::Timer => {
                self.bus.cpu_io.status.interrupt_request_timer = true;
                if self.bus.cpu_io.status.interrupt_enable_timer {
                    self.processor.r.halt = false;
                }
            }
            Interrupt::Serial => {
                self.bus.cpu_io.status.interrupt_request_serial = true;
                if self.bus.cpu_io.status.interrupt_enable_serial {
                    self.processor.r.halt = false;
                }
            }
            Interrupt::Joypad => {
                self.bus.cpu_io.status.interrupt_request_joypad = true;
                if self.bus.cpu_io.status.interrupt_enable_joypad {
                    self.processor.r.halt = false;
                    self.processor.r.stop = false;
                }
            }
        }
    }

    pub fn interrupt_test(&mut self) {
        if !self.processor.r.ime {
        } else if self.bus.cpu_io.status.interrupt_request_vblank
            && self.bus.cpu_io.status.interrupt_enable_vblank
        {
            self.bus.cpu_io.status.interrupt_request_vblank = false;
            self.interrupt(0x0040);
        } else if self.bus.cpu_io.status.interrupt_request_stat
            && self.bus.cpu_io.status.interrupt_enable_stat
        {
            self.bus.cpu_io.status.interrupt_request_stat = false;
            self.interrupt(0x0048);
        } else if self.bus.cpu_io.status.interrupt_request_timer
            && self.bus.cpu_io.status.interrupt_enable_timer
        {
            self.bus.cpu_io.status.interrupt_request_timer = false;
            self.interrupt(0x0050);
        } else if self.bus.cpu_io.status.interrupt_request_serial
            && self.bus.cpu_io.status.interrupt_enable_serial
        {
            self.bus.cpu_io.status.interrupt_request_serial = false;
            self.interrupt(0x0058);
        } else if self.bus.cpu_io.status.interrupt_request_joypad
            && self.bus.cpu_io.status.interrupt_enable_joypad
        {
            self.bus.cpu_io.status.interrupt_request_joypad = false;
            self.interrupt(0x0060);
        }
    }

    pub fn stop(&mut self) -> bool {
        if self.bus.cpu_io.status.speed_switch {
            self.bus.cpu_io.status.speed_switch = false;
            self.bus.cpu_io.status.speed_double ^= true;
            if !self.bus.cpu_io.status.speed_double {
                self.set_frequency(4 * 1024 * 1024);
            }
            if self.bus.cpu_io.status.speed_double {
                self.set_frequency(8 * 1024 * 1024);
            }
            true
        } else {
            false
        }
    }

    //TODO CPU power
}
