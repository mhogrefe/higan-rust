extern crate higan_rust;
extern crate malachite_base;

pub mod higan {
    pub mod gb {
        pub mod apu {
            pub mod noise;
            pub mod sequencer;
            pub mod square1;
            pub mod square2;
            pub mod wave;
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
