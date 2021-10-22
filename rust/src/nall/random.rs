use malachite_base::num::arithmetic::traits::WrappingAddAssign;
use malachite_base::num::conversion::traits::WrappingFrom;

pub fn random_seed() -> u32 {
    4
}

pub trait Rng {
    fn read(&mut self) -> u64;

    fn random(&mut self) -> u64 {
        let mut value = 0;
        for _ in 0u32..2 {
            value = value << 32 | self.read();
        }
        value
    }
}

#[derive(Default)]
pub struct Pcg {
    state: u64,
    increment: u64,
}

impl Pcg {
    pub fn new() -> Pcg {
        let mut pcg = Pcg::default();
        pcg.seed();
        pcg
    }

    pub fn seed(&mut self) {
        let seed = random_seed();
        let sequence = 0;

        self.state = 0;
        self.increment = sequence << 1 | 1;
        self.read();
        let r = self.read();
        self.state.wrapping_add_assign(r);
        self.read();
        self.state.wrapping_add_assign(u64::from(seed));
        self.read();
    }
}

impl Rng for Pcg {
    fn read(&mut self) -> u64 {
        let state = self.state;
        self.state = state
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(self.increment);
        let xorshift = u32::wrapping_from(((state >> 18) ^ state) >> 27);
        let rotate = u32::wrapping_from(state >> 59);
        return (u64::from(xorshift) >> rotate)
            | (u64::from(xorshift) << (rotate.wrapping_neg() & 31));
    }
}
