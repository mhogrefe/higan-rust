use ares::emulator::types::{U10, U15, U2, U4, U6, U9};

#[derive(Clone, Debug, Default)]
pub struct PPU {
    pub color_emulation_dmg: &'static str,
    pub color_emulation_cgb: bool,
    pub interframe_blending: bool,
    pub vram: Vec<u8>,
    pub oam: Vec<u8>,
    pub bgp: Vec<U2>,
    pub obp: Vec<U2>,
    pub bgpd: Vec<u16>,
    pub obpd: Vec<u16>,

    pub status: Status,
    pub latch: Latch,
    pub history: History,
    pub bg: Pixel,
    pub ob: Pixel,
    pub sprite: [Sprite; 10],
    pub sprites: U4, //0-9
    pub px: u8,      //0-159
    pub background: Background,
    pub window: Background,
}

#[derive(Clone, Debug, Default)]
pub struct Status {
    pub irq: bool, //STAT IRQ line
    pub lx: U9,    //0~455

    //$ff40  LCDC
    pub bg_enable: bool, //DMG: BG enable; CGB: BG priority
    pub ob_enable: bool,
    pub ob_size: bool,
    pub bg_tilemap_select: bool,
    pub bg_tiledata_select: bool,
    pub window_display_enable: bool,
    pub window_tilemap_select: bool,
    pub display_enable: bool,

    //$ff41  STAT
    pub mode: U2, //0 = Hblank, 1 = Vblank, 2 = OAM search, 3 = LCD transfer
    pub interrupt_h_blank: bool,
    pub interrupt_v_blank: bool,
    pub interrupt_oam: bool,
    pub interrupt_lyc: bool,

    //$ff42  SCY
    pub scy: u8,

    //$ff43  SCX
    pub scx: u8,

    //$ff44  LY
    pub ly: u8,

    //$ff45  LYC
    pub lyc: u8,

    //$ff46  DMA
    pub dma_bank: u8,
    pub dma_active: bool,
    pub dma_clock: U10, //0~323 (DMG), 0~645 (CGB)

    //$ff4a  WY
    pub wy: u8,

    //$ff4b  WX
    pub wx: u8,

    //$ff4f  VBK
    pub vram_bank: bool,

    //$ff68  BGPI
    pub bgpi: U6,
    pub bgpi_increment: bool,

    //$ff6a  OBPI
    pub obpi: u8,
    pub obpi_increment: bool,
}

#[derive(Clone, Debug, Default)]
pub struct Latch {
    pub display_enable: bool,
    pub window_display_enable: bool,
    pub wx: u8,
    pub wy: u8,
}

#[derive(Clone, Debug, Default)]
pub struct History {
    pub mode: U10, //5 x 2-bit
}

#[derive(Clone, Debug, Default)]
pub struct Pixel {
    pub color: U15,
    pub palette: u8,
    pub priority: bool,
}

#[derive(Clone, Debug, Default)]
pub struct Sprite {
    pub x: i16,
    pub y: i16,
    pub tile: u8,
    pub attributes: u8,
    pub tiledata: u16,
}

#[derive(Clone, Debug, Default)]
pub struct Background {
    pub attributes: u8,
    pub tiledata: u16,
}

pub mod color;
