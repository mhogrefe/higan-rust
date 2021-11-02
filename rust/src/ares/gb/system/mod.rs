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
pub struct System<P: Platform> {
    pub platform: P,
    pub model: Model,
    pub controls: Controls,
    pub boot_rom: Vec<u8>,
    pub cpu: CPU,
    pub apu: APU,
    pub information: Information,

    pub cpu_return_to_sync: bool,
    pub cpu_resuming_after_sync: bool,
    pub cpu_sync_points: Vec<usize>,
    pub cpu_local_u8s: Vec<u8>,
    pub cpu_local_u32s: Vec<u32>,

    pub cpu_main_sync_point: usize,
    pub cpu_main_sp: u16,
    pub cpu_main_pc: u8,
    pub cpu_main_mask: u8,

    pub cpu_instruction_add_direct_relative_sync_point: usize,
    pub cpu_instruction_add_direct_relative_fresh_data: u8,

    pub cpu_instruction_call_condition_address_sync_point: usize,
    pub cpu_instruction_call_condition_address_address: u16,

    pub cpu_instruction_jp_condition_address_sync_point: usize,
    pub cpu_instruction_jp_condition_address_address: u16,

    pub cpu_instruction_jr_condition_relative_sync_point: usize,
    pub cpu_instruction_jr_condition_relative_data: u8,

    pub cpu_instruction_ld_direct_direct_relative_sync_point: usize,
    pub cpu_instruction_ld_direct_direct_relative_data: u8,

    pub cpu_instruction_ret_sync_point: usize,
    pub cpu_instruction_ret_address: u16,

    pub cpu_instruction_reti_sync_point: usize,
    pub cpu_instruction_reti_address: u16,
}

pub mod controls;
