//TODO test

pub trait RandomNumberGenerator {
    fn seed(&mut self, seed: u64);

    fn call(&mut self) -> u64;
}

//Galois LFSR using CRC64 polynomials
pub struct LinearFeedbackShiftRegisterGenerator {
    lfsr: u64,
}

const CRC64: u64 = 0xc96c_5795_d787_0f42;

impl LinearFeedbackShiftRegisterGenerator {
    pub fn new() -> LinearFeedbackShiftRegisterGenerator {
        LinearFeedbackShiftRegisterGenerator { lfsr: CRC64 }
    }
}

impl RandomNumberGenerator for LinearFeedbackShiftRegisterGenerator {
    fn seed(&mut self, seed: u64) {
        self.lfsr = seed;
        for _ in 0..8 {
            self.call();
        }
    }

    fn call(&mut self) -> u64 {
        self.lfsr = (self.lfsr >> 1) ^ ((self.lfsr & 1).wrapping_neg() & CRC64);
        self.lfsr
    }
}
