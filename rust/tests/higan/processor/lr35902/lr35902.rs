use higan_rust::higan::processor::lr35902::lr35902::Register;

#[test]
fn test_register() {
    let mut register = Register::default();
    register.set_word(123);
    assert_eq!(register.get_word(), 123);

    let mut register = Register::default();
    register.set_word(0x12ab);
    assert_eq!(register.get_hi(), 0x12);
    assert_eq!(register.get_lo(), 0xab);

    register.set_hi(0x34);
    assert_eq!(register.get_hi(), 0x34);
    assert_eq!(register.get_lo(), 0xab);
    assert_eq!(register.get_word(), 0x34ab);

    register.set_lo(0xcd);
    assert_eq!(register.get_hi(), 0x34);
    assert_eq!(register.get_lo(), 0xcd);
    assert_eq!(register.get_word(), 0x34cd);
}
