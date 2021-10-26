use higan_rust::ares::component::processor::sm83::sm83::LR35902;

#[test]
fn test_af() {
    let mut processor = LR35902::default();
    processor.set_af(123);
    assert_eq!(processor.get_af(), 123);

    let mut processor = LR35902::default();
    processor.set_af(0x12ab);
    assert_eq!(processor.get_a(), 0x12);
    assert_eq!(processor.get_f(), 0xab);

    processor.set_a(0x34);
    assert_eq!(processor.get_a(), 0x34);
    assert_eq!(processor.get_f(), 0xab);
    assert_eq!(processor.get_af(), 0x34ab);

    processor.set_f(0xcd);
    assert_eq!(processor.get_a(), 0x34);
    assert_eq!(processor.get_f(), 0xcd);
    assert_eq!(processor.get_af(), 0x34cd);
}

#[test]
fn test_bc() {
    let mut processor = LR35902::default();
    processor.set_bc(123);
    assert_eq!(processor.get_bc(), 123);

    let mut processor = LR35902::default();
    processor.set_bc(0x12ab);
    assert_eq!(processor.get_b(), 0x12);
    assert_eq!(processor.get_c(), 0xab);

    processor.set_b(0x34);
    assert_eq!(processor.get_b(), 0x34);
    assert_eq!(processor.get_c(), 0xab);
    assert_eq!(processor.get_bc(), 0x34ab);

    processor.set_c(0xcd);
    assert_eq!(processor.get_b(), 0x34);
    assert_eq!(processor.get_c(), 0xcd);
    assert_eq!(processor.get_bc(), 0x34cd);
}

#[test]
fn test_de() {
    let mut processor = LR35902::default();
    processor.set_de(123);
    assert_eq!(processor.get_de(), 123);

    let mut processor = LR35902::default();
    processor.set_de(0x12ab);
    assert_eq!(processor.get_d(), 0x12);
    assert_eq!(processor.get_e(), 0xab);

    processor.set_d(0x34);
    assert_eq!(processor.get_d(), 0x34);
    assert_eq!(processor.get_e(), 0xab);
    assert_eq!(processor.get_de(), 0x34ab);

    processor.set_e(0xcd);
    assert_eq!(processor.get_d(), 0x34);
    assert_eq!(processor.get_e(), 0xcd);
    assert_eq!(processor.get_de(), 0x34cd);
}

#[test]
fn test_hl() {
    let mut processor = LR35902::default();
    processor.set_hl(123);
    assert_eq!(processor.get_hl(), 123);

    let mut processor = LR35902::default();
    processor.set_hl(0x12ab);
    assert_eq!(processor.get_h(), 0x12);
    assert_eq!(processor.get_l(), 0xab);

    processor.set_h(0x34);
    assert_eq!(processor.get_h(), 0x34);
    assert_eq!(processor.get_l(), 0xab);
    assert_eq!(processor.get_hl(), 0x34ab);

    processor.set_l(0xcd);
    assert_eq!(processor.get_h(), 0x34);
    assert_eq!(processor.get_l(), 0xcd);
    assert_eq!(processor.get_hl(), 0x34cd);
}

#[test]
fn test_sp() {
    let mut processor = LR35902::default();
    processor.set_sp(123);
    assert_eq!(processor.get_sp(), 123);

    assert_eq!(processor.post_increment_sp(), 123);
    assert_eq!(processor.get_sp(), 124);

    processor.set_sp(65_535);
    assert_eq!(processor.post_increment_sp(), 65_535);
    assert_eq!(processor.get_sp(), 0);

    processor.set_sp(123);
    assert_eq!(processor.pre_decrement_sp(), 122);

    processor.set_sp(0);
    assert_eq!(processor.pre_decrement_sp(), 65_535);
}

#[test]
fn test_pc() {
    let mut processor = LR35902::default();
    processor.set_pc(123);
    assert_eq!(processor.get_pc(), 123);

    assert_eq!(processor.post_increment_pc(), 123);
    assert_eq!(processor.get_pc(), 124);

    processor.set_pc(65_535);
    assert_eq!(processor.post_increment_pc(), 65_535);
    assert_eq!(processor.get_pc(), 0);
}

#[test]
fn test_cf() {
    let mut processor = LR35902::default();
    processor.set_f(0b10100101);
    assert!(!processor.get_cf());
    processor.set_cf(true);
    assert_eq!(processor.get_f(), 0b10110101);
}

#[test]
fn test_hf() {
    let mut processor = LR35902::default();
    processor.set_f(0b10100101);
    assert!(processor.get_hf());
    processor.set_hf(false);
    assert_eq!(processor.get_f(), 0b10000101);
}

#[test]
fn test_nf() {
    let mut processor = LR35902::default();
    processor.set_f(0b10100101);
    assert!(!processor.get_nf());
    processor.set_nf(true);
    assert_eq!(processor.get_f(), 0b11100101);
}

#[test]
fn test_zf() {
    let mut processor = LR35902::default();
    processor.set_f(0b10100101);
    assert!(processor.get_zf());
    processor.set_zf(false);
    assert_eq!(processor.get_f(), 0b00100101);
}
