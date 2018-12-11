use higan_rust::higan::processor::lr35902::lr35902::LR35902;

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
                let mut index = 0;
                if processor.get_cf() {
                    index |= 4;
                }
                if processor.get_hf() {
                    index |= 2;
                }
                if processor.get_zf() {
                    index |= 1;
                }
                outcomes[index] += 1;
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
