//TODO test

use higan::emulator::types::{U22, U3};
use higan::gb::memory::memory::Bus;
use higan::processor::lr35902::lr35902::LR35902;

#[derive(Clone, Debug)]
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

#[derive(Clone)]
pub struct CPU {
    pub processor: LR35902,
    pub bus: Bus,
    pub status: Status,
    pub wram: [u8; 32_768], //GB=8192, GBC=32768
    pub hram: [u8; 128],
}
