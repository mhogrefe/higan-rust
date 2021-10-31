use ares::gb::apu::APU;
use ares::gb::cpu::CPU;
use ares::node::InputNode;
use ares::platform::Platform;

#[derive(Clone, Debug, Default)]
pub struct Controls {
    pub up: InputNode,
    pub down: InputNode,
    pub left: InputNode,
    pub right: InputNode,
    pub b: InputNode,
    pub a: InputNode,
    pub select: InputNode,
    pub start: InputNode,

    pub y_hold: bool,
    pub up_latch: bool,
    pub down_latch: bool,
    pub x_hold: bool,
    pub left_latch: bool,
    pub right_latch: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Model {
    GameBoy,
    GameBoyColor,
    SuperGameBoy,
}

impl Default for Model {
    fn default() -> Model {
        Model::GameBoy
    }
}

#[derive(Clone, Debug, Default)]
pub struct System<P: Platform> {
    pub platform: P,
    pub model: Model,
    pub controls: Controls,
    pub boot_rom: Vec<u8>,
    pub cpu: CPU,
    pub apu: APU,
}

pub mod controls;
