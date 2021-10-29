use higan_rust::ares::component::processor::sm83::sm83::Registers;

#[test]
fn test_af() {
    let mut registers = Registers::default();
    registers.set_af(123);
    assert_eq!(registers.get_af(), 123);

    let mut registers = Registers::default();
    registers.set_af(0x12ab);
    assert_eq!(registers.get_a(), 0x12);
    assert_eq!(registers.get_f(), 0xab);

    registers.set_a(0x34);
    assert_eq!(registers.get_a(), 0x34);
    assert_eq!(registers.get_f(), 0xab);
    assert_eq!(registers.get_af(), 0x34ab);

    registers.set_f(0xcd);
    assert_eq!(registers.get_a(), 0x34);
    assert_eq!(registers.get_f(), 0xcd);
    assert_eq!(registers.get_af(), 0x34cd);
}

#[test]
fn test_bc() {
    let mut registers = Registers::default();
    registers.set_bc(123);
    assert_eq!(registers.get_bc(), 123);

    let mut registers = Registers::default();
    registers.set_bc(0x12ab);
    assert_eq!(registers.get_b(), 0x12);
    assert_eq!(registers.get_c(), 0xab);

    registers.set_b(0x34);
    assert_eq!(registers.get_b(), 0x34);
    assert_eq!(registers.get_c(), 0xab);
    assert_eq!(registers.get_bc(), 0x34ab);

    registers.set_c(0xcd);
    assert_eq!(registers.get_b(), 0x34);
    assert_eq!(registers.get_c(), 0xcd);
    assert_eq!(registers.get_bc(), 0x34cd);
}

#[test]
fn test_de() {
    let mut registers = Registers::default();
    registers.set_de(123);
    assert_eq!(registers.get_de(), 123);

    let mut registers = Registers::default();
    registers.set_de(0x12ab);
    assert_eq!(registers.get_d(), 0x12);
    assert_eq!(registers.get_e(), 0xab);

    registers.set_d(0x34);
    assert_eq!(registers.get_d(), 0x34);
    assert_eq!(registers.get_e(), 0xab);
    assert_eq!(registers.get_de(), 0x34ab);

    registers.set_e(0xcd);
    assert_eq!(registers.get_d(), 0x34);
    assert_eq!(registers.get_e(), 0xcd);
    assert_eq!(registers.get_de(), 0x34cd);
}

#[test]
fn test_hl() {
    let mut registers = Registers::default();
    registers.set_hl(123);
    assert_eq!(registers.get_hl(), 123);

    let mut registers = Registers::default();
    registers.set_hl(0x12ab);
    assert_eq!(registers.get_h(), 0x12);
    assert_eq!(registers.get_l(), 0xab);

    registers.set_h(0x34);
    assert_eq!(registers.get_h(), 0x34);
    assert_eq!(registers.get_l(), 0xab);
    assert_eq!(registers.get_hl(), 0x34ab);

    registers.set_l(0xcd);
    assert_eq!(registers.get_h(), 0x34);
    assert_eq!(registers.get_l(), 0xcd);
    assert_eq!(registers.get_hl(), 0x34cd);
}

#[test]
fn test_sp() {
    let mut registers = Registers::default();
    registers.set_sp(123);
    assert_eq!(registers.get_sp(), 123);

    assert_eq!(registers.post_increment_sp(), 123);
    assert_eq!(registers.get_sp(), 124);

    registers.set_sp(65_535);
    assert_eq!(registers.post_increment_sp(), 65_535);
    assert_eq!(registers.get_sp(), 0);

    registers.set_sp(123);
    assert_eq!(registers.pre_decrement_sp(), 122);

    registers.set_sp(0);
    assert_eq!(registers.pre_decrement_sp(), 65_535);
}

#[test]
fn test_pc() {
    let mut registers = Registers::default();
    registers.set_pc(123);
    assert_eq!(registers.get_pc(), 123);

    assert_eq!(registers.post_increment_pc(), 123);
    assert_eq!(registers.get_pc(), 124);

    registers.set_pc(65_535);
    assert_eq!(registers.post_increment_pc(), 65_535);
    assert_eq!(registers.get_pc(), 0);
}

#[test]
fn test_cf() {
    let mut registers = Registers::default();
    registers.set_f(0b10100101);
    assert!(!registers.get_cf());
    registers.set_cf(true);
    assert_eq!(registers.get_f(), 0b10110101);
}

#[test]
fn test_hf() {
    let mut registers = Registers::default();
    registers.set_f(0b10100101);
    assert!(registers.get_hf());
    registers.set_hf(false);
    assert_eq!(registers.get_f(), 0b10000101);
}

#[test]
fn test_nf() {
    let mut registers = Registers::default();
    registers.set_f(0b10100101);
    assert!(!registers.get_nf());
    registers.set_nf(true);
    assert_eq!(registers.get_f(), 0b11100101);
}

#[test]
fn test_zf() {
    let mut registers = Registers::default();
    registers.set_f(0b10100101);
    assert!(registers.get_zf());
    registers.set_zf(false);
    assert_eq!(registers.get_f(), 0b00100101);
}
