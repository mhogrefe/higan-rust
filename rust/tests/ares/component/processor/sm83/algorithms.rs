use higan_rust::ares::component::processor::sm83::sm83::Registers;
use higan_rust::ares::emulator::types::U3;
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::num::logic::traits::BitAccess;

#[test]
fn test_add() {
    // CF false, HF false, ZF false
    let mut registers = Registers::default();
    assert_eq!(registers.add(3, 4, false), 7);
    assert!(!registers.get_cf());
    assert!(!registers.get_hf());
    assert!(!registers.get_nf());
    assert!(!registers.get_zf());

    // CF false, HF false, ZF true
    let mut registers = Registers::default();
    assert_eq!(registers.add(0, 0, false), 0);
    assert!(!registers.get_cf());
    assert!(!registers.get_hf());
    assert!(!registers.get_nf());
    assert!(registers.get_zf());

    // CF false, HF true, ZF false
    let mut registers = Registers::default();
    assert_eq!(registers.add(9, 8, false), 17);
    assert!(!registers.get_cf());
    assert!(registers.get_hf());
    assert!(!registers.get_nf());
    assert!(!registers.get_zf());

    // CF false, HF true, ZF true impossible

    // CF true, HF false, ZF false
    let mut registers = Registers::default();
    assert_eq!(registers.add(128, 128, true), 1);
    assert!(registers.get_cf());
    assert!(!registers.get_hf());
    assert!(!registers.get_nf());
    assert!(!registers.get_zf());

    // CF true, HF false, ZF true
    let mut registers = Registers::default();
    assert_eq!(registers.add(128, 128, false), 0);
    assert!(registers.get_cf());
    assert!(!registers.get_hf());
    assert!(!registers.get_nf());
    assert!(registers.get_zf());

    // Variant of previous case: sum is same but HF is different!
    let mut registers = Registers::default();
    assert_eq!(registers.add(128, 127, true), 0);
    assert!(registers.get_cf());
    assert!(registers.get_hf());
    assert!(!registers.get_nf());
    assert!(registers.get_zf());

    // CF true, HF true, ZF false
    let mut registers = Registers::default();
    assert_eq!(registers.add(143, 143, false), 30);
    assert!(registers.get_cf());
    assert!(registers.get_hf());
    assert!(!registers.get_nf());
    assert!(!registers.get_zf());

    // CF true, HF true, ZF true
    let mut registers = Registers::default();
    assert_eq!(registers.add(3, 252, true), 0);
    assert!(registers.get_cf());
    assert!(registers.get_hf());
    assert!(!registers.get_nf());
    assert!(registers.get_zf());
}

#[test]
fn exhaustive_test_add() {
    let mut outcomes = [0u32; 8];
    for x in 0..=255 {
        for y in 0..=255 {
            for &carry in [true, false].iter() {
                let mut registers = Registers::default();
                registers.add(x, y, carry);
                let mut index = 0u32;
                index.assign_bit(2, registers.get_cf());
                index.assign_bit(1, registers.get_hf());
                index.assign_bit(0, registers.get_zf());
                outcomes[index as usize] += 1;
            }
        }
    }
    // CF false, HF false, ZF false
    assert_eq!(outcomes[0b000], 34_815);

    // CF false, HF false, ZF true
    assert_eq!(outcomes[0b001], 1);

    // CF false, HF true, ZF false
    assert_eq!(outcomes[0b010], 30_720);

    // CF false, HF true, ZF true
    assert_eq!(outcomes[0b011], 0);

    // CF true, HF false, ZF false
    assert_eq!(outcomes[0b100], 30_705);

    // CF true, HF false, ZF true
    assert_eq!(outcomes[0b101], 15);

    // CF true, HF true, ZF false
    assert_eq!(outcomes[0b110], 34_320);

    // CF true, HF true, ZF true
    assert_eq!(outcomes[0b111], 496);
}

#[test]
fn test_and() {
    // ZF false
    let mut registers = Registers::default();
    assert_eq!(registers.and(6, 7), 6);
    assert!(!registers.get_cf());
    assert!(registers.get_hf());
    assert!(!registers.get_nf());
    assert!(!registers.get_zf());

    // ZF true
    let mut registers = Registers::default();
    assert_eq!(registers.and(6, 8), 0);
    assert!(!registers.get_cf());
    assert!(registers.get_hf());
    assert!(!registers.get_nf());
    assert!(registers.get_zf());
}

#[test]
fn exhaustive_test_and() {
    let mut outcomes = [0u32; 2];
    for x in 0..=255 {
        for y in 0..=255 {
            let mut registers = Registers::default();
            registers.and(x, y);
            let mut index = 0u32;
            index.assign_bit(0, registers.get_zf());
            outcomes[index as usize] += 1;
        }
    }
    // ZF false
    assert_eq!(outcomes[0b0], 58_975);

    // ZF true
    assert_eq!(outcomes[0b1], 6_561);
}

#[test]
fn test_bit() {
    // ZF false
    let mut registers = Registers::default();
    registers.bit(U3::wrapping_from(1), 0b10100101);
    assert!(registers.get_hf());
    assert!(!registers.get_nf());
    assert!(registers.get_zf());

    // ZF true
    let mut registers = Registers::default();
    registers.bit(U3::wrapping_from(2), 0b10100101);
    assert!(registers.get_hf());
    assert!(!registers.get_nf());
    assert!(!registers.get_zf());
}

#[test]
fn exhaustive_test_bit() {
    let mut outcomes = [0u32; 2];
    for index in 0..=7 {
        for x in 0..=255 {
            let mut registers = Registers::default();
            registers.bit(U3::wrapping_from(index), x);
            let mut outcome_index = 0u32;
            outcome_index.assign_bit(0, registers.get_zf());
            outcomes[outcome_index as usize] += 1;
        }
    }
    // ZF false
    assert_eq!(outcomes[0b0], 1_024);

    // ZF true
    assert_eq!(outcomes[0b1], 1_024);
}

#[test]
fn test_cp() {
    // CF false, HF false, ZF false
    let mut registers = Registers::default();
    registers.cp(10, 5);
    assert!(!registers.get_cf());
    assert!(!registers.get_hf());
    assert!(registers.get_nf());
    assert!(!registers.get_zf());

    // CF false, HF false, ZF true
    let mut registers = Registers::default();
    registers.cp(10, 10);
    assert!(!registers.get_cf());
    assert!(!registers.get_hf());
    assert!(registers.get_nf());
    assert!(registers.get_zf());

    // CF false, HF true, ZF false
    let mut registers = Registers::default();
    registers.cp(100, 10);
    assert!(!registers.get_cf());
    assert!(registers.get_hf());
    assert!(registers.get_nf());
    assert!(!registers.get_zf());

    // CF false, HF true, ZF true impossible

    // CF true, HF false, ZF false
    let mut registers = Registers::default();
    registers.cp(2, 17);
    assert!(registers.get_cf());
    assert!(!registers.get_hf());
    assert!(registers.get_nf());
    assert!(!registers.get_zf());

    // CF true, HF false, ZF true impossible

    // CF true, HF true, ZF false
    let mut registers = Registers::default();
    registers.cp(2, 19);
    assert!(registers.get_cf());
    assert!(registers.get_hf());
    assert!(registers.get_nf());
    assert!(!registers.get_zf());

    // CF true, HF true, ZF true impossible
}

#[test]
fn exhaustive_test_cp() {
    let mut outcomes = [0u32; 8];
    for x in 0..=255 {
        for y in 0..=255 {
            let mut registers = Registers::default();
            registers.cp(x, y);
            let mut index = 0u32;
            index.assign_bit(2, registers.get_cf());
            index.assign_bit(1, registers.get_hf());
            index.assign_bit(0, registers.get_zf());
            outcomes[index as usize] += 1;
        }
    }
    // CF false, HF false, ZF false
    assert_eq!(outcomes[0b000], 18_240);

    // CF false, HF false, ZF true
    assert_eq!(outcomes[0b001], 256);

    // CF false, HF true, ZF false
    assert_eq!(outcomes[0b010], 14_400);

    // CF false, HF true, ZF true
    assert_eq!(outcomes[0b011], 0);

    // CF true, HF false, ZF false
    assert_eq!(outcomes[0b100], 16_320);

    // CF true, HF false, ZF true
    assert_eq!(outcomes[0b101], 0);

    // CF true, HF true, ZF false
    assert_eq!(outcomes[0b110], 16_320);

    // CF true, HF true, ZF true
    assert_eq!(outcomes[0b111], 0);
}

#[test]
fn test_dec() {
    // HF false, ZF false
    let mut registers = Registers::default();
    assert_eq!(registers.dec(10), 9);
    assert!(!registers.get_hf());
    assert!(registers.get_nf());
    assert!(!registers.get_zf());

    // HF false, ZF true
    let mut registers = Registers::default();
    assert_eq!(registers.dec(1), 0);
    assert!(!registers.get_hf());
    assert!(registers.get_nf());
    assert!(registers.get_zf());

    // HF true, ZF false
    let mut registers = Registers::default();
    assert_eq!(registers.dec(32), 31);
    assert!(registers.get_hf());
    assert!(registers.get_nf());
    assert!(!registers.get_zf());

    // HF true, ZF true impossible

    let mut registers = Registers::default();
    assert_eq!(registers.dec(0), 255);
    assert!(registers.get_hf());
    assert!(registers.get_nf());
    assert!(!registers.get_zf());
}

#[test]
fn exhaustive_test_dec() {
    let mut outcomes = [0u32; 4];
    for x in 0..=255 {
        let mut registers = Registers::default();
        registers.dec(x);
        let mut index = 0u32;
        index.assign_bit(1, registers.get_hf());
        index.assign_bit(0, registers.get_zf());
        outcomes[index as usize] += 1;
    }
    // HF false, ZF false
    assert_eq!(outcomes[0b00], 239);

    // HF false, ZF true
    assert_eq!(outcomes[0b01], 1);

    // HF true, ZF false
    assert_eq!(outcomes[0b10], 16);

    // HF true, ZF true
    assert_eq!(outcomes[0b11], 0);
}

#[test]
fn test_inc() {
    // HF false, ZF false
    let mut registers = Registers::default();
    assert_eq!(registers.inc(10), 11);
    assert!(!registers.get_hf());
    assert!(!registers.get_nf());
    assert!(!registers.get_zf());

    // HF false, ZF true impossible

    // HF true, ZF false
    let mut registers = Registers::default();
    assert_eq!(registers.inc(31), 32);
    assert!(registers.get_hf());
    assert!(!registers.get_nf());
    assert!(!registers.get_zf());

    // HF true, ZF true
    let mut registers = Registers::default();
    assert_eq!(registers.inc(255), 0);
    assert!(registers.get_hf());
    assert!(!registers.get_nf());
    assert!(registers.get_zf());
}

#[test]
fn exhaustive_test_inc() {
    let mut outcomes = [0u32; 4];
    for x in 0..=255 {
        let mut registers = Registers::default();
        registers.inc(x);
        let mut index = 0u32;
        index.assign_bit(1, registers.get_hf());
        index.assign_bit(0, registers.get_zf());
        outcomes[index as usize] += 1;
    }
    // HF false, ZF false
    assert_eq!(outcomes[0b00], 240);

    // HF false, ZF true
    assert_eq!(outcomes[0b01], 0);

    // HF true, ZF false
    assert_eq!(outcomes[0b10], 15);

    // HF true, ZF true
    assert_eq!(outcomes[0b11], 1);
}

#[test]
fn test_or() {
    // ZF false
    let mut registers = Registers::default();
    assert_eq!(registers.or(6, 9), 15);
    assert!(!registers.get_cf());
    assert!(!registers.get_hf());
    assert!(!registers.get_nf());
    assert!(!registers.get_zf());

    // ZF true
    let mut registers = Registers::default();
    assert_eq!(registers.or(0, 0), 0);
    assert!(!registers.get_cf());
    assert!(!registers.get_hf());
    assert!(!registers.get_nf());
    assert!(registers.get_zf());
}

#[test]
fn exhaustive_test_or() {
    let mut outcomes = [0u32; 2];
    for x in 0..=255 {
        for y in 0..=255 {
            let mut registers = Registers::default();
            registers.or(x, y);
            let mut index = 0u32;
            index.assign_bit(0, registers.get_zf());
            outcomes[index as usize] += 1;
        }
    }
    // ZF false
    assert_eq!(outcomes[0b0], 65_535);

    // ZF true
    assert_eq!(outcomes[0b1], 1);
}

#[test]
fn test_rl() {
    // CF false, ZF false
    let mut registers = Registers::default();
    assert_eq!(registers.rl(10), 20);
    assert!(!registers.get_cf());
    assert!(!registers.get_hf());
    assert!(!registers.get_nf());
    assert!(!registers.get_zf());

    // CF false, ZF true
    let mut registers = Registers::default();
    assert_eq!(registers.rl(0), 0);
    assert!(!registers.get_cf());
    assert!(!registers.get_hf());
    assert!(!registers.get_nf());
    assert!(registers.get_zf());

    // CF true, ZF false
    let mut registers = Registers::default();
    assert_eq!(registers.rl(130), 4);
    assert!(registers.get_cf());
    assert!(!registers.get_hf());
    assert!(!registers.get_nf());
    assert!(!registers.get_zf());

    // CF true, ZF true
    let mut registers = Registers::default();
    assert_eq!(registers.rl(0b10000000), 0);
    assert!(registers.get_cf());
    assert!(!registers.get_hf());
    assert!(!registers.get_nf());
    assert!(registers.get_zf());

    let mut registers = Registers::default();
    registers.set_cf(true);
    assert_eq!(registers.rl(0), 1);
    assert!(!registers.get_cf());
    assert!(!registers.get_hf());
    assert!(!registers.get_nf());
    assert!(!registers.get_zf());

    let mut registers = Registers::default();
    registers.set_cf(true);
    assert_eq!(registers.rl(0b10000000), 1);
    assert!(registers.get_cf());
    assert!(!registers.get_hf());
    assert!(!registers.get_nf());
    assert!(!registers.get_zf());
}

#[test]
fn exhaustive_test_rl() {
    let mut outcomes = [0u32; 4];
    for x in 0..=255 {
        let mut registers = Registers::default();
        registers.rl(x);
        let mut index = 0u32;
        index.assign_bit(1, registers.get_cf());
        index.assign_bit(0, registers.get_zf());
        outcomes[index as usize] += 1;
    }
    // CF false, ZF false
    assert_eq!(outcomes[0b00], 127);

    // CF false, ZF true
    assert_eq!(outcomes[0b01], 1);

    // CF true, ZF false
    assert_eq!(outcomes[0b10], 127);

    // CF true, ZF true
    assert_eq!(outcomes[0b11], 1);
}

#[test]
fn test_rlc() {
    // CF false, ZF false
    let mut registers = Registers::default();
    assert_eq!(registers.rlc(10), 20);
    assert!(!registers.get_cf());
    assert!(!registers.get_hf());
    assert!(!registers.get_nf());
    assert!(!registers.get_zf());

    // CF false, ZF true
    let mut registers = Registers::default();
    assert_eq!(registers.rlc(0), 0);
    assert!(!registers.get_cf());
    assert!(!registers.get_hf());
    assert!(!registers.get_nf());
    assert!(registers.get_zf());

    // CF true, ZF false
    let mut registers = Registers::default();
    assert_eq!(registers.rlc(130), 5);
    assert!(registers.get_cf());
    assert!(!registers.get_hf());
    assert!(!registers.get_nf());
    assert!(!registers.get_zf());

    // CF true, ZF true impossible
}

#[test]
fn exhaustive_test_rlc() {
    let mut outcomes = [0u32; 4];
    for x in 0..=255 {
        let mut registers = Registers::default();
        registers.rlc(x);
        let mut index = 0u32;
        index.assign_bit(1, registers.get_cf());
        index.assign_bit(0, registers.get_zf());
        outcomes[index as usize] += 1;
    }
    // CF false, ZF false
    assert_eq!(outcomes[0b00], 127);

    // CF false, ZF true
    assert_eq!(outcomes[0b01], 1);

    // CF true, ZF false
    assert_eq!(outcomes[0b10], 128);

    // CF true, ZF true
    assert_eq!(outcomes[0b11], 0);
}

#[test]
fn test_rr() {
    // CF false, ZF false
    let mut registers = Registers::default();
    assert_eq!(registers.rr(10), 5);
    assert!(!registers.get_cf());
    assert!(!registers.get_hf());
    assert!(!registers.get_nf());
    assert!(!registers.get_zf());

    // CF false, ZF true
    let mut registers = Registers::default();
    assert_eq!(registers.rr(0), 0);
    assert!(!registers.get_cf());
    assert!(!registers.get_hf());
    assert!(!registers.get_nf());
    assert!(registers.get_zf());

    // CF true, ZF false
    let mut registers = Registers::default();
    assert_eq!(registers.rr(3), 1);
    assert!(registers.get_cf());
    assert!(!registers.get_hf());
    assert!(!registers.get_nf());
    assert!(!registers.get_zf());

    // CF true, ZF true
    let mut registers = Registers::default();
    assert_eq!(registers.rr(1), 0);
    assert!(registers.get_cf());
    assert!(!registers.get_hf());
    assert!(!registers.get_nf());
    assert!(registers.get_zf());

    let mut registers = Registers::default();
    registers.set_cf(true);
    assert_eq!(registers.rr(0), 0b10000000);
    assert!(!registers.get_cf());
    assert!(!registers.get_hf());
    assert!(!registers.get_nf());
    assert!(!registers.get_zf());

    let mut registers = Registers::default();
    registers.set_cf(true);
    assert_eq!(registers.rr(1), 0b10000000);
    assert!(registers.get_cf());
    assert!(!registers.get_hf());
    assert!(!registers.get_nf());
    assert!(!registers.get_zf());
}

#[test]
fn exhaustive_test_rr() {
    let mut outcomes = [0u32; 4];
    for x in 0..=255 {
        let mut registers = Registers::default();
        registers.rr(x);
        let mut index = 0u32;
        index.assign_bit(1, registers.get_cf());
        index.assign_bit(0, registers.get_zf());
        outcomes[index as usize] += 1;
    }
    // CF false, ZF false
    assert_eq!(outcomes[0b00], 127);

    // CF false, ZF true
    assert_eq!(outcomes[0b01], 1);

    // CF true, ZF false
    assert_eq!(outcomes[0b10], 127);

    // CF true, ZF true
    assert_eq!(outcomes[0b11], 1);
}

#[test]
fn test_rrc() {
    // CF false, ZF false
    let mut registers = Registers::default();
    assert_eq!(registers.rrc(2), 1);
    assert!(!registers.get_cf());
    assert!(!registers.get_hf());
    assert!(!registers.get_nf());
    assert!(!registers.get_zf());

    // CF false, ZF true
    let mut registers = Registers::default();
    assert_eq!(registers.rrc(0), 0);
    assert!(!registers.get_cf());
    assert!(!registers.get_hf());
    assert!(!registers.get_nf());
    assert!(registers.get_zf());

    // CF true, ZF false
    let mut registers = Registers::default();
    assert_eq!(registers.rrc(1), 0b10000000);
    assert!(registers.get_cf());
    assert!(!registers.get_hf());
    assert!(!registers.get_nf());
    assert!(!registers.get_zf());

    // CF true, ZF true impossible
}

#[test]
fn exhaustive_test_rrc() {
    let mut outcomes = [0u32; 4];
    for x in 0..=255 {
        let mut registers = Registers::default();
        registers.rrc(x);
        let mut index = 0u32;
        index.assign_bit(1, registers.get_cf());
        index.assign_bit(0, registers.get_zf());
        outcomes[index as usize] += 1;
    }
    // CF false, ZF false
    assert_eq!(outcomes[0b00], 127);

    // CF false, ZF true
    assert_eq!(outcomes[0b01], 1);

    // CF true, ZF false
    assert_eq!(outcomes[0b10], 128);

    // CF true, ZF true
    assert_eq!(outcomes[0b11], 0);
}
