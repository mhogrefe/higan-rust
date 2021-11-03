use ares::ares::scheduler::scheduler::Scheduler;
use ares::ares::scheduler::thread::Thread;
use ares::emulator::types::U3;
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

#[derive(Clone, Debug)]
pub struct Information {
    pub name: &'static str,
    pub model: Model,
    pub clocks_executed: u32,
}

impl Default for Information {
    fn default() -> Information {
        Information {
            name: "Game Boy",
            model: Model::GameBoy,
            clocks_executed: 0,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct SmallStack<T: Copy + Default> {
    i: usize,
    xs: [T; 10],
}

impl<T: Copy + Default> SmallStack<T> {
    pub fn push(&mut self, x: T) {
        self.xs[self.i] = x;
        self.i += 1;
    }

    pub fn pop(&mut self) -> T {
        self.i -= 1;
        self.xs[self.i]
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
    pub information: Information,

    pub scheduler: Scheduler,
    pub cpu_thread: Thread,
    pub apu_thread: Thread,

    pub cpu_return_to_sync: bool,
    pub cpu_resuming_after_sync: bool,
    pub cpu_sync_points: SmallStack<usize>,
    pub cpu_local_u3s: SmallStack<U3>,
    pub cpu_local_u8s: SmallStack<u8>,
    pub cpu_local_u16s: SmallStack<u16>,
    pub cpu_local_u32s: SmallStack<u32>,

    pub apu_return_to_sync: bool,
}

pub mod controls;
