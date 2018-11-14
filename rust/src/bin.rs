extern crate higan_rust;
extern crate malachite_base;

use higan_rust::higan::emulator::types::U3;
use higan_rust::higan::gb::apu::square_1::Square1;
use malachite_base::num::PrimitiveInteger;

fn main() {
    let square = Square1::default();
    println!("{}", U3::WIDTH);
    println!("{:?}", square);
}
