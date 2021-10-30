//TODO test

use malachite_base::num::conversion::traits::{JoinHalves, SplitInHalf};

#[derive(Clone, Debug, Default)]
pub struct Register(u16);

/// See higan-rust/cpp/ares/component/processor/sm83/sm83.hpp
impl Register {
    #[inline]
    pub fn get_word(&self) -> u16 {
        self.0
    }

    #[inline]
    pub fn get_word_mut(&mut self) -> &mut u16 {
        &mut self.0
    }

    #[inline]
    pub fn set_word(&mut self, word: u16) {
        self.0 = word;
    }

    #[inline]
    pub fn get_hi(&self) -> u8 {
        self.0.upper_half()
    }

    #[inline]
    pub fn get_lo(&self) -> u8 {
        self.0.lower_half()
    }

    #[inline]
    pub fn set_hi(&mut self, hi: u8) {
        self.0 = u16::join_halves(hi, self.0.lower_half());
    }

    #[inline]
    pub fn set_lo(&mut self, lo: u8) {
        self.0 = u16::join_halves(self.0.upper_half(), lo);
    }
}

/// See higan-rust/cpp/ares/component/processor/sm83/sm83.hpp
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

    pub halt_bug: bool,
}
