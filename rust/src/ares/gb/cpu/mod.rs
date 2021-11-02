use ares::component::processor::sm83::sm83::Registers;
use ares::emulator::types::{U2, U22, U3, U4, U5, U7};
use ares::gb::system::Model;
use ares::gb::system::System;
use ares::platform::Platform;
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::num::logic::traits::{BitAccess, BitScan, NotAssign};

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

    pub fn lower(&mut self, interrupt_id: u32) {
        self.status
            .interrupt_flag
            .clear_bit(u64::from(interrupt_id));
    }
}

impl<P: Platform> System<P> {
    pub fn cpu_main(&mut self) {
        match self.cpu_main_sync_point {
            0 => self.cpu_main_fresh(),
            1 => self.cpu_main_resume_at_1(),
            2 => self.cpu_main_resume_at_2(),
            3 => self.cpu_main_resume_at_3(),
            4 => self.cpu_main_resume_at_4(),
            5 => self.cpu_main_resume_at_5(),
            6 => self.cpu_main_resume_at_6(),
            _ => panic!(),
        }
    }

    fn cpu_main_fresh(&mut self) {
        if self.cpu.status.h_blank_pending {
            self.cpu.status.h_blank_pending = false;
            self.cpu_h_blank_trigger();
        }
        //are interrupts enabled?
        if self.cpu.r.ime {
            //are any interrupts pending?
            if self.cpu.status.interrupt_latch != 0 {
                //TODO debugger.interrupt("IRQ");
                // ** S1
                self.cpu_idle();
                if self.cpu_return_to_sync {
                    self.cpu_main_sync_point = 1;
                    return;
                }

                // ** S2
                self.cpu_idle();
                if self.cpu_return_to_sync {
                    self.cpu_main_sync_point = 2;
                    return;
                }

                // ** S3
                self.cpu_idle();
                if self.cpu_return_to_sync {
                    self.cpu_main_sync_point = 3;
                    return;
                }

                self.cpu.r.ime = false;
                // upper byte may write to IE before it is polled again
                self.cpu_main_sp = self.cpu.r.pre_decrement_sp();
                self.cpu_main_pc = u8::wrapping_from(self.cpu.r.get_pc() >> 8);

                // ** S4
                self.cpu_write(self.cpu_main_sp, self.cpu_main_pc);
                if self.cpu_return_to_sync {
                    self.cpu_main_sync_point = 4;
                    return;
                }

                self.cpu_main_mask =
                    self.cpu.status.interrupt_flag.x() & self.cpu.status.interrupt_enable;
                // lower byte write to IE has no effect
                self.cpu_main_sp = self.cpu.r.pre_decrement_sp();
                self.cpu_main_pc = u8::wrapping_from(self.cpu.r.get_pc());

                // ** S5
                self.cpu_write(self.cpu_main_sp, self.cpu_main_pc);
                if self.cpu_return_to_sync {
                    self.cpu_main_sync_point = 5;
                    return;
                }

                if self.cpu_main_mask != 0 {
                    // find highest priority interrupt
                    let interrupt_id = self.cpu_main_mask.index_of_next_true_bit(0).unwrap();
                    self.cpu.lower(u32::wrapping_from(interrupt_id));
                    self.cpu
                        .r
                        .set_pc(u16::wrapping_from(0x0040 + interrupt_id * 8))
                } else {
                    // if push(PCH) writes to IE and disables all requested interrupts,
                    // PC is forced to zero
                    self.cpu.r.set_pc(0x0000);
                }
            }
        }
        //TODO debugger.instruction();

        // ** S6
        self.instruction();
        if self.cpu_return_to_sync {
            self.cpu_main_sync_point = 6;
            return;
        }

        self.cpu_main_sync_point = 0;
        if self.model == Model::SuperGameBoy {
            //TODO scheduler.exit(Event::Step);
        }
    }

    fn cpu_main_resume_at_1(&mut self) {
        // ** S1
        self.cpu_idle();
        if self.cpu_return_to_sync {
            self.cpu_main_sync_point = 1;
            return;
        }

        // ** S2
        self.cpu_idle();
        if self.cpu_return_to_sync {
            self.cpu_main_sync_point = 2;
            return;
        }

        // ** S3
        self.cpu_idle();
        if self.cpu_return_to_sync {
            self.cpu_main_sync_point = 3;
            return;
        }

        self.cpu.r.ime = false;
        // upper byte may write to IE before it is polled again
        self.cpu_main_sp = self.cpu.r.pre_decrement_sp();
        self.cpu_main_pc = u8::wrapping_from(self.cpu.r.get_pc() >> 8);

        // ** S4
        self.cpu_write(self.cpu_main_sp, self.cpu_main_pc);
        if self.cpu_return_to_sync {
            self.cpu_main_sync_point = 4;
            return;
        }

        self.cpu_main_mask = self.cpu.status.interrupt_flag.x() & self.cpu.status.interrupt_enable;
        // lower byte write to IE has no effect
        self.cpu_main_sp = self.cpu.r.pre_decrement_sp();
        self.cpu_main_pc = u8::wrapping_from(self.cpu.r.get_pc());

        // ** S5
        self.cpu_write(self.cpu_main_sp, self.cpu_main_pc);
        if self.cpu_return_to_sync {
            self.cpu_main_sync_point = 5;
            return;
        }

        if self.cpu_main_mask != 0 {
            // find highest priority interrupt
            let interrupt_id = self.cpu_main_mask.index_of_next_true_bit(0).unwrap();
            self.cpu.lower(u32::wrapping_from(interrupt_id));
            self.cpu
                .r
                .set_pc(u16::wrapping_from(0x0040 + interrupt_id * 8))
        } else {
            // if push(PCH) writes to IE and disables all requested interrupts,
            // PC is forced to zero
            self.cpu.r.set_pc(0x0000);
        }
        //TODO debugger.instruction();

        // ** S6
        self.instruction();
        if self.cpu_return_to_sync {
            self.cpu_main_sync_point = 6;
            return;
        }

        self.cpu_main_sync_point = 0;
        if self.model == Model::SuperGameBoy {
            //TODO scheduler.exit(Event::Step);
        }
    }

    fn cpu_main_resume_at_2(&mut self) {
        // ** S2
        self.cpu_idle();
        if self.cpu_return_to_sync {
            self.cpu_main_sync_point = 2;
            return;
        }

        // ** S3
        self.cpu_idle();
        if self.cpu_return_to_sync {
            self.cpu_main_sync_point = 3;
            return;
        }

        self.cpu.r.ime = false;
        // upper byte may write to IE before it is polled again
        self.cpu_main_sp = self.cpu.r.pre_decrement_sp();
        self.cpu_main_pc = u8::wrapping_from(self.cpu.r.get_pc() >> 8);

        // ** S4
        self.cpu_write(self.cpu_main_sp, self.cpu_main_pc);
        if self.cpu_return_to_sync {
            self.cpu_main_sync_point = 4;
            return;
        }

        self.cpu_main_mask = self.cpu.status.interrupt_flag.x() & self.cpu.status.interrupt_enable;
        // lower byte write to IE has no effect
        self.cpu_main_sp = self.cpu.r.pre_decrement_sp();
        self.cpu_main_pc = u8::wrapping_from(self.cpu.r.get_pc());

        // ** S5
        self.cpu_write(self.cpu_main_sp, self.cpu_main_pc);
        if self.cpu_return_to_sync {
            self.cpu_main_sync_point = 5;
            return;
        }

        if self.cpu_main_mask != 0 {
            // find highest priority interrupt
            let interrupt_id = self.cpu_main_mask.index_of_next_true_bit(0).unwrap();
            self.cpu.lower(u32::wrapping_from(interrupt_id));
            self.cpu
                .r
                .set_pc(u16::wrapping_from(0x0040 + interrupt_id * 8))
        } else {
            // if push(PCH) writes to IE and disables all requested interrupts,
            // PC is forced to zero
            self.cpu.r.set_pc(0x0000);
        }
        //TODO debugger.instruction();

        // ** S6
        self.instruction();
        if self.cpu_return_to_sync {
            self.cpu_main_sync_point = 6;
            return;
        }

        self.cpu_main_sync_point = 0;
        if self.model == Model::SuperGameBoy {
            //TODO scheduler.exit(Event::Step);
        }
    }

    fn cpu_main_resume_at_3(&mut self) {
        // ** S3
        self.cpu_idle();
        if self.cpu_return_to_sync {
            self.cpu_main_sync_point = 3;
            return;
        }

        self.cpu.r.ime = false;
        // upper byte may write to IE before it is polled again
        self.cpu_main_sp = self.cpu.r.pre_decrement_sp();
        self.cpu_main_pc = u8::wrapping_from(self.cpu.r.get_pc() >> 8);

        // ** S4
        self.cpu_write(self.cpu_main_sp, self.cpu_main_pc);
        if self.cpu_return_to_sync {
            self.cpu_main_sync_point = 4;
            return;
        }

        self.cpu_main_mask = self.cpu.status.interrupt_flag.x() & self.cpu.status.interrupt_enable;
        // lower byte write to IE has no effect
        self.cpu_main_sp = self.cpu.r.pre_decrement_sp();
        self.cpu_main_pc = u8::wrapping_from(self.cpu.r.get_pc());

        // ** S5
        self.cpu_write(self.cpu_main_sp, self.cpu_main_pc);
        if self.cpu_return_to_sync {
            self.cpu_main_sync_point = 5;
            return;
        }

        if self.cpu_main_mask != 0 {
            // find highest priority interrupt
            let interrupt_id = self.cpu_main_mask.index_of_next_true_bit(0).unwrap();
            self.cpu.lower(u32::wrapping_from(interrupt_id));
            self.cpu
                .r
                .set_pc(u16::wrapping_from(0x0040 + interrupt_id * 8))
        } else {
            // if push(PCH) writes to IE and disables all requested interrupts,
            // PC is forced to zero
            self.cpu.r.set_pc(0x0000);
        }
        //TODO debugger.instruction();

        // ** S6
        self.instruction();
        if self.cpu_return_to_sync {
            self.cpu_main_sync_point = 6;
            return;
        }

        self.cpu_main_sync_point = 0;
        if self.model == Model::SuperGameBoy {
            //TODO scheduler.exit(Event::Step);
        }
    }

    fn cpu_main_resume_at_4(&mut self) {
        // ** S4
        self.cpu_write(self.cpu_main_sp, self.cpu_main_pc);
        if self.cpu_return_to_sync {
            self.cpu_main_sync_point = 4;
            return;
        }

        self.cpu_main_mask = self.cpu.status.interrupt_flag.x() & self.cpu.status.interrupt_enable;
        // lower byte write to IE has no effect
        self.cpu_main_sp = self.cpu.r.pre_decrement_sp();
        self.cpu_main_pc = u8::wrapping_from(self.cpu.r.get_pc());

        // ** S5
        self.cpu_write(self.cpu_main_sp, self.cpu_main_pc);
        if self.cpu_return_to_sync {
            self.cpu_main_sync_point = 5;
            return;
        }

        if self.cpu_main_mask != 0 {
            // find highest priority interrupt
            let interrupt_id = self.cpu_main_mask.index_of_next_true_bit(0).unwrap();
            self.cpu.lower(u32::wrapping_from(interrupt_id));
            self.cpu
                .r
                .set_pc(u16::wrapping_from(0x0040 + interrupt_id * 8))
        } else {
            // if push(PCH) writes to IE and disables all requested interrupts,
            // PC is forced to zero
            self.cpu.r.set_pc(0x0000);
        }
        //TODO debugger.instruction();

        // ** S6
        self.instruction();
        if self.cpu_return_to_sync {
            self.cpu_main_sync_point = 6;
            return;
        }

        self.cpu_main_sync_point = 0;
        if self.model == Model::SuperGameBoy {
            //TODO scheduler.exit(Event::Step);
        }
    }

    fn cpu_main_resume_at_5(&mut self) {
        // ** S5
        self.cpu_write(self.cpu_main_sp, self.cpu_main_pc);
        if self.cpu_return_to_sync {
            self.cpu_main_sync_point = 5;
            return;
        }

        if self.cpu_main_mask != 0 {
            // find highest priority interrupt
            let interrupt_id = self.cpu_main_mask.index_of_next_true_bit(0).unwrap();
            self.cpu.lower(u32::wrapping_from(interrupt_id));
            self.cpu
                .r
                .set_pc(u16::wrapping_from(0x0040 + interrupt_id * 8))
        } else {
            // if push(PCH) writes to IE and disables all requested interrupts,
            // PC is forced to zero
            self.cpu.r.set_pc(0x0000);
        }
        //TODO debugger.instruction();
        // ** S6

        self.instruction();
        if self.cpu_return_to_sync {
            self.cpu_main_sync_point = 6;
            return;
        }

        self.cpu_main_sync_point = 0;
        if self.model == Model::SuperGameBoy {
            //TODO scheduler.exit(Event::Step);
        }
    }

    fn cpu_main_resume_at_6(&mut self) {
        // ** S6
        self.instruction();
        if self.cpu_return_to_sync {
            self.cpu_main_sync_point = 6;
            return;
        }

        self.cpu_main_sync_point = 0;
        if self.model == Model::SuperGameBoy {
            //TODO scheduler.exit(Event::Step);
        }
    }
}

pub mod io;
pub mod memory;
pub mod timing;
