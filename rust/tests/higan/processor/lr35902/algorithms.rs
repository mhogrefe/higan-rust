use higan_rust::higan::emulator::types::U3;
use higan_rust::higan::processor::lr35902::lr35902::LR35902;
use malachite_base::misc::WrappingFrom;
use malachite_base::num::BitAccess;

#[test]
fn test_add() {
    // CF false, HF false, ZF false
    let mut processor = LR35902::default();
    assert_eq!(processor.add(3, 4, false), 7);
    assert!(!processor.get_cf());
    assert!(!processor.get_hf());
    assert!(!processor.get_nf());
    assert!(!processor.get_zf());

    // CF false, HF false, ZF true
    let mut processor = LR35902::default();
    assert_eq!(processor.add(0, 0, false), 0);
    assert!(!processor.get_cf());
    assert!(!processor.get_hf());
    assert!(!processor.get_nf());
    assert!(processor.get_zf());

    // CF false, HF true, ZF false
    let mut processor = LR35902::default();
    assert_eq!(processor.add(9, 8, false), 17);
    assert!(!processor.get_cf());
    assert!(processor.get_hf());
    assert!(!processor.get_nf());
    assert!(!processor.get_zf());

    // CF false, HF true, ZF true impossible

    // CF true, HF false, ZF false
    let mut processor = LR35902::default();
    assert_eq!(processor.add(128, 128, true), 1);
    assert!(processor.get_cf());
    assert!(!processor.get_hf());
    assert!(!processor.get_nf());
    assert!(!processor.get_zf());

    // CF true, HF false, ZF true
    let mut processor = LR35902::default();
    assert_eq!(processor.add(128, 128, false), 0);
    assert!(processor.get_cf());
    assert!(!processor.get_hf());
    assert!(!processor.get_nf());
    assert!(processor.get_zf());

    // Variant of previous case: sum is same but HF is different!
    let mut processor = LR35902::default();
    assert_eq!(processor.add(128, 127, true), 0);
    assert!(processor.get_cf());
    assert!(processor.get_hf());
    assert!(!processor.get_nf());
    assert!(processor.get_zf());

    // CF true, HF true, ZF false
    let mut processor = LR35902::default();
    assert_eq!(processor.add(143, 143, false), 30);
    assert!(processor.get_cf());
    assert!(processor.get_hf());
    assert!(!processor.get_nf());
    assert!(!processor.get_zf());

    // CF true, HF true, ZF true
    let mut processor = LR35902::default();
    assert_eq!(processor.add(3, 252, true), 0);
    assert!(processor.get_cf());
    assert!(processor.get_hf());
    assert!(!processor.get_nf());
    assert!(processor.get_zf());
}

#[test]
fn exhaustive_test_add() {
    let mut outcomes = [0u32; 8];
    for x in 0..=255 {
        for y in 0..=255 {
            for &carry in [true, false].iter() {
                let mut processor = LR35902::default();
                processor.add(x, y, carry);
                let mut index = 0u32;
                index.assign_bit(2, processor.get_cf());
                index.assign_bit(1, processor.get_hf());
                index.assign_bit(0, processor.get_zf());
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
    let mut processor = LR35902::default();
    assert_eq!(processor.and(6, 7), 6);
    assert!(!processor.get_cf());
    assert!(processor.get_hf());
    assert!(!processor.get_nf());
    assert!(!processor.get_zf());

    // ZF true
    let mut processor = LR35902::default();
    assert_eq!(processor.and(6, 8), 0);
    assert!(!processor.get_cf());
    assert!(processor.get_hf());
    assert!(!processor.get_nf());
    assert!(processor.get_zf());
}

#[test]
fn exhaustive_test_and() {
    let mut outcomes = [0u32; 2];
    for x in 0..=255 {
        for y in 0..=255 {
            let mut processor = LR35902::default();
            processor.and(x, y);
            let mut index = 0u32;
            index.assign_bit(0, processor.get_zf());
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
    let mut processor = LR35902::default();
    processor.bit(U3::wrapping_from(1), 0b10100101);
    assert!(processor.get_hf());
    assert!(!processor.get_nf());
    assert!(processor.get_zf());

    // ZF true
    let mut processor = LR35902::default();
    processor.bit(U3::wrapping_from(2), 0b10100101);
    assert!(processor.get_hf());
    assert!(!processor.get_nf());
    assert!(!processor.get_zf());
}

#[test]
fn exhaustive_test_bit() {
    let mut outcomes = [0u32; 2];
    for index in 0..=7 {
        for x in 0..=255 {
            let mut processor = LR35902::default();
            processor.bit(U3::wrapping_from(index), x);
            let mut outcome_index = 0u32;
            outcome_index.assign_bit(0, processor.get_zf());
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
    let mut processor = LR35902::default();
    processor.cp(10, 5);
    assert!(!processor.get_cf());
    assert!(!processor.get_hf());
    assert!(processor.get_nf());
    assert!(!processor.get_zf());

    // CF false, HF false, ZF true
    let mut processor = LR35902::default();
    processor.cp(10, 10);
    assert!(!processor.get_cf());
    assert!(!processor.get_hf());
    assert!(processor.get_nf());
    assert!(processor.get_zf());

    // CF false, HF true, ZF false
    let mut processor = LR35902::default();
    processor.cp(100, 10);
    assert!(!processor.get_cf());
    assert!(processor.get_hf());
    assert!(processor.get_nf());
    assert!(!processor.get_zf());

    // CF false, HF true, ZF true impossible

    // CF true, HF false, ZF false
    let mut processor = LR35902::default();
    processor.cp(2, 17);
    assert!(processor.get_cf());
    assert!(!processor.get_hf());
    assert!(processor.get_nf());
    assert!(!processor.get_zf());

    // CF true, HF false, ZF true impossible

    // CF true, HF true, ZF false
    let mut processor = LR35902::default();
    processor.cp(2, 19);
    assert!(processor.get_cf());
    assert!(processor.get_hf());
    assert!(processor.get_nf());
    assert!(!processor.get_zf());

    // CF true, HF true, ZF true impossible
}

#[test]
fn exhaustive_test_cp() {
    let mut outcomes = [0u32; 8];
    for x in 0..=255 {
        for y in 0..=255 {
            let mut processor = LR35902::default();
            processor.cp(x, y);
            let mut index = 0u32;
            index.assign_bit(2, processor.get_cf());
            index.assign_bit(1, processor.get_hf());
            index.assign_bit(0, processor.get_zf());
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