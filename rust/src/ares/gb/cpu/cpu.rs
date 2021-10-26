//TODO test

use ares::component::processor::sm83::sm83::SM83;
use ares::emulator::types::{U2, U22, U3, U4, U5, U7};
use ares::gb::memory::memory::Bus;

/// See higan-rust/cpp/ares/gb/cpu/cpu.hpp
#[derive(Clone, Copy, Debug)]
pub enum Interrupt {
    VerticalBlank,
    Stat,
    Timer,
    Serial,
    Joypad,
}

/// See higan-rust/cpp/ares/gb/cpu/cpu.hpp
#[derive(Clone, Debug, Default)]
pub struct Status {
    pub clock: U22,
    pub interrupt_latch: u8,
    pub h_blank_pending: bool,

    //$ff00  JOYP
    pub joyp: U4,
    pub p14: bool,
    pub p15: bool,

    //$ff01  SB
    pub serial_data: u8,
    pub serial_bits: U4,

    //$ff02  SC
    pub serial_clock: bool,
    pub serial_speed: bool,
    pub serial_transfer: bool,

    //$ff04  DIV
    pub div: u16,

    //$ff05  TIMA
    pub tima: u8,

    //$ff06  TMA
    pub tma: u8,

    //$ff07  TAC
    pub timer_clock: U2,
    pub timer_enable: bool,

    //$ff0f  IF
    pub interrupt_flag: U5,

    //$ff4d  KEY1
    pub speed_switch: bool,
    pub speed_double: bool,

    //$ff51,$ff52  HDMA1,HDMA2
    pub dma_source: u16,

    //$ff53,$ff54  HDMA3,HDMA4
    pub dma_target: u16,

    //$ff55  HDMA5
    dma_length: U7,
    hdma_active: bool,

    //$ff6c  ???
    pub ff6c: bool,

    //$ff70  SVBK
    pub wram_bank: U3,

    //$ff72-$ff75  ???
    pub ff72: u8,
    pub ff73: u8,
    pub ff74: u8,
    pub ff75: U3,

    //$ffff  IE
    pub interrupt_enable: u8,
}

const CPU_WRAM_SIZE: usize = 32_768; //GB=8192, GBC=32768
const CPU_HRAM_SIZE: usize = 128;

#[derive(Clone)]
pub struct CPUIO {
    pub model_is_super_game_boy: bool,
    pub status: Status,
    pub wram: [u8; CPU_WRAM_SIZE],
    pub hram: [u8; CPU_HRAM_SIZE],
}

impl Default for CPUIO {
    fn default() -> CPUIO {
        CPUIO {
            model_is_super_game_boy: false,
            status: Status::default(),
            wram: [0; CPU_WRAM_SIZE],
            hram: [0; CPU_HRAM_SIZE],
        }
    }
}

#[derive(Clone, Default)]
pub struct CPU {
    pub model_is_game_boy_color: bool,
    pub processor: SM83,
    pub bus: Bus,
}

impl CPU {
    //TODO this is probably a Thread thing
    pub fn interrupt(&self, _: u32) {}

    //TODO
    pub fn set_frequency(&self, _: u32) {}

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
}
