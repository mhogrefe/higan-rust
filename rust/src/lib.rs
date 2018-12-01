#[macro_use]
extern crate malachite_base;
extern crate rand;

pub mod higan {
    pub mod emulator {
        pub mod types;
    }
    pub mod gb {
        pub mod apu {
            pub mod apu;
            pub mod noise;
            pub mod sequencer;
            pub mod square_1;
            pub mod square_2;
            pub mod wave;
        }
        pub mod memory {
            pub mod memory;
        }
        pub mod system {
            pub mod system;
        }
    }
    pub mod processor {
        pub mod lr35902 {
            pub mod algorithms;
            pub mod lr35902;
            pub mod registers;
        }
    }
}
pub mod nall {
    pub mod random;
}
