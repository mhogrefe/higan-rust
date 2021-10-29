use ares::component::processor::sm83::sm83::Registers;
use ares::emulator::types::U3;
use malachite_base::num::arithmetic::traits::{
    DivisibleByPowerOf2, ModPowerOf2, Parity, WrappingAddAssign, WrappingSubAssign,
};
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::num::logic::traits::BitAccess;

// See higan-rust/cpp/ares/component/processor/sm83/algorithms.cpp
impl Registers {
    pub fn add(&mut self, target: u8, source: u8, carry: bool) -> u8 {
        let mut x: u16 = u16::from(target) + u16::from(source);
        let mut y: u16 = u16::from(target.mod_power_of_2(4)) + u16::from(source.mod_power_of_2(4));
        if carry {
            x += 1;
            y += 1;
        }
        self.set_cf(x > 0xff);
        self.set_hf(y > 0x0f);
        self.set_nf(false);
        self.set_zf(x.divisible_by_power_of_2(8));
        u8::wrapping_from(x)
    }

    pub fn and(&mut self, mut target: u8, source: u8) -> u8 {
        target &= source;
        self.set_cf(false);
        self.set_hf(true);
        self.set_nf(false);
        self.set_zf(target == 0);
        target
    }

    pub fn bit(&mut self, index: U3, target: u8) {
        self.set_hf(true);
        self.set_nf(false);
        self.set_zf(!target.get_bit(u64::from(index)));
    }

    pub fn cp(&mut self, target: u8, source: u8) {
        let x: u16 = u16::from(target).wrapping_sub(u16::from(source));
        let y: u16 =
            u16::from(target.mod_power_of_2(4)).wrapping_sub(u16::from(source.mod_power_of_2(4)));
        self.set_cf(x > 0xff);
        self.set_hf(y > 0x0f);
        self.set_nf(true);
        self.set_zf(x.divisible_by_power_of_2(8));
    }

    pub fn dec(&mut self, mut target: u8) -> u8 {
        target.wrapping_sub_assign(1);
        self.set_hf(target.mod_power_of_2(4) == 0x0f);
        self.set_nf(true);
        self.set_zf(target == 0);
        target
    }

    pub fn inc(&mut self, mut target: u8) -> u8 {
        target.wrapping_add_assign(1);
        self.set_hf(target.divisible_by_power_of_2(4));
        self.set_nf(false);
        self.set_zf(target == 0);
        target
    }

    pub fn or(&mut self, mut target: u8, source: u8) -> u8 {
        target |= source;
        self.set_cf(false);
        self.set_hf(false);
        self.set_nf(false);
        self.set_zf(target == 0);
        target
    }

    pub fn rl(&mut self, mut target: u8) -> u8 {
        let carry = target.get_bit(7);
        target <<= 1;
        if self.get_cf() {
            target |= 1;
        }
        self.set_cf(carry);
        self.set_hf(false);
        self.set_nf(false);
        self.set_zf(target == 0);
        target
    }

    pub fn rlc(&mut self, mut target: u8) -> u8 {
        target = target << 1 | target >> 7;
        self.set_cf(target.get_bit(0));
        self.set_hf(false);
        self.set_nf(false);
        self.set_zf(target == 0);
        target
    }

    pub fn rr(&mut self, mut target: u8) -> u8 {
        let carry = target.get_bit(0);
        target >>= 1;
        if self.get_cf() {
            target.set_bit(7);
        }
        self.set_cf(carry);
        self.set_hf(false);
        self.set_nf(false);
        self.set_zf(target == 0);
        target
    }

    pub fn rrc(&mut self, mut target: u8) -> u8 {
        target = target << 7 | target >> 1;
        self.set_cf(target.get_bit(7));
        self.set_hf(false);
        self.set_nf(false);
        self.set_zf(target == 0);
        target
    }

    pub fn sla(&mut self, mut target: u8) -> u8 {
        let carry = target.get_bit(7);
        target <<= 1;
        self.set_cf(carry);
        self.set_hf(false);
        self.set_nf(false);
        self.set_zf(target == 0);
        target
    }

    pub fn sra(&mut self, mut target: u8) -> u8 {
        let carry = target.odd();
        target = u8::wrapping_from(i8::wrapping_from(target) >> 1);
        self.set_cf(carry);
        self.set_hf(false);
        self.set_nf(false);
        self.set_zf(target == 0);
        target
    }

    pub fn srl(&mut self, mut target: u8) -> u8 {
        let carry = target.odd();
        target >>= 1;
        self.set_cf(carry);
        self.set_hf(false);
        self.set_nf(false);
        self.set_zf(target == 0);
        target
    }
}
