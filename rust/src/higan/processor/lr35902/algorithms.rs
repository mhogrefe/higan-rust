use higan::emulator::types::{U3, U4};
use higan::processor::lr35902::lr35902::LR35902;
use malachite_base::num::arithmetic::traits::{WrappingAddAssign, WrappingSubAssign};
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::num::logic::traits::BitAccess;

impl LR35902 {
    pub fn add(&mut self, target: u8, source: u8, carry: bool) -> u8 {
        let x: u16 = u16::from(target) + u16::from(source) + if carry { 1 } else { 0 };
        let y: u16 = u16::from(U4::wrapping_from(target).x())
            + u16::from(U4::wrapping_from(source).x())
            + if carry { 1 } else { 0 };
        self.set_cf(x > 0xff);
        self.set_hf(y > 0x0f);
        self.set_nf(false);
        self.set_zf(u8::wrapping_from(x) == 0);
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
        self.set_zf(!target.get_bit(u64::from(index.x())));
    }

    pub fn cp(&mut self, target: u8, source: u8) {
        let x: u16 = u16::from(target).wrapping_sub(u16::from(source));
        let y: u16 = u16::from(U4::wrapping_from(target).x())
            .wrapping_sub(u16::from(U4::wrapping_from(source).x()));
        self.set_cf(x > 0xff);
        self.set_hf(y > 0x0f);
        self.set_nf(true);
        self.set_zf(u8::wrapping_from(x) == 0);
    }

    pub fn dec(&mut self, mut target: u8) -> u8 {
        target.wrapping_sub_assign(1);
        self.set_hf(U4::wrapping_from(target).x() == 0x0f);
        self.set_nf(true);
        self.set_zf(target == 0);
        target
    }

    pub fn inc(&mut self, mut target: u8) -> u8 {
        target.wrapping_add_assign(1);
        self.set_hf(U4::wrapping_from(target).x() == 0x00);
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
        target = target << 1 | if self.get_cf() { 1 } else { 0 };
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
        return target;
    }
}
