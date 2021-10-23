use ares::emulator::types::U3;

/// See higan-rust/cpp/ares/gb/apu/apu.hpp
#[derive(Clone, Debug, Default)]
pub struct Channel {
    pub left_enable: bool,
    pub right_enable: bool,
}

/// See higan-rust/cpp/ares/gb/apu/apu.hpp
#[derive(Clone, Debug, Default)]
pub struct Sequencer {
    pub left_enable: bool,
    pub left_volume: U3,
    pub right_enable: bool,
    pub right_volume: U3,

    pub square_1: Channel,
    pub square_2: Channel,
    pub wave: Channel,
    pub noise: Channel,

    pub enable: bool,

    pub center: i16,
    pub left: i16,
    pub right: i16,
}

impl Sequencer {
    /// See higan-rust/cpp/ares/gb/apu/sequencer.cpp
    pub fn power(&mut self) {
        *self = Sequencer::default();
    }
}
