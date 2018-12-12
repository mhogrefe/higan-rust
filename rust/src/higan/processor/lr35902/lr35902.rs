//TODO test

use malachite_base::num::{JoinHalves, SplitInHalf};

#[derive(Clone, Debug, Default)]
pub struct Register(u16);

impl Register {
    pub fn get_word(&self) -> u16 {
        self.0
    }

    pub fn set_word(&mut self, word: u16) {
        self.0 = word;
    }

    pub fn get_hi(&self) -> u8 {
        self.0.upper_half()
    }

    pub fn get_lo(&self) -> u8 {
        self.0.lower_half()
    }

    pub fn set_hi(&mut self, hi: u8) {
        self.0 = u16::join_halves(hi, self.0.lower_half());
    }

    pub fn set_lo(&mut self, lo: u8) {
        self.0 = u16::join_halves(self.0.upper_half(), lo);
    }
}

#[derive(Clone, Debug, Default)]
pub struct Registers {
    pub af: Register,
    pub bc: Register,
    pub de: Register,
    pub hl: Register,
    pub sp: Register,
    pub pc: Register,

    pub ei: bool,
    pub halt: bool,
    pub stop: bool,
    pub ime: bool,
}

#[derive(Clone, Debug, Default)]
pub struct LR35902 {
    pub r: Registers,
}
