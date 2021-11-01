use ares::component::processor::sm83::sm83::Registers;
use ares::emulator::types::{U2, U22, U3, U4, U5, U7};
use ares::gb::system::Model;
use malachite_base::num::logic::traits::{BitAccess, NotAssign};

/// See higan-rust/cpp/ares/gb/cpu/cpu.hpp
#[derive(Clone, Copy, Debug)]
pub enum Interrupt {
    VerticalBlank,
    Stat,
    Timer,
    Serial,
    Joypad,
}

impl Interrupt {
    pub fn value(self) -> u32 {
        match self {
            Interrupt::VerticalBlank => 0,
            Interrupt::Stat => 1,
            Interrupt::Timer => 2,
            Interrupt::Serial => 3,
            Interrupt::Joypad => 4,
        }
    }
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

#[derive(Clone, Debug, Default)]
pub struct CPU {
    pub model: Model,
    pub r: Registers,
    pub status: Status,
}

// See higan-rust/cpp/ares/gb/cpu/cpu.cpp
impl CPU {
    pub fn stoppable(&mut self) -> bool {
        if self.status.speed_switch {
            self.status.speed_switch = false;
            self.status.speed_double.not_assign();
            if !self.status.speed_double {
                //TODO setFrequency(4 * 1024 * 1024);
            }
            if self.status.speed_double {
                //TODO setFrequency(8 * 1024 * 1024);
            }
            false
        } else {
            true
        }
    }

    pub fn raise(&mut self, interrupt_id: u32) {
        self.status.interrupt_flag.set_bit(u64::from(interrupt_id));
        if self
            .status
            .interrupt_enable
            .get_bit(u64::from(interrupt_id))
        {
            self.r.halt = false;
            if interrupt_id == Interrupt::Joypad.value() {
                self.r.stop = false;
            }
        }
    }
}

pub mod io;
pub mod memory;
pub mod timing;
