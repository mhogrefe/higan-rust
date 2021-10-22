extern crate fixed_width;
extern crate malachite_base;
extern crate rand;

pub mod ares {
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
        pub mod cpu {
            pub mod cpu;
            pub mod io;
            pub mod memory;
            pub mod timing;
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
