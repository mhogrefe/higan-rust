extern crate fixed_width;
extern crate malachite_base;
extern crate rand;

pub mod ares {
    pub mod component {
        pub mod processor {
            pub mod sm83 {
                pub mod algorithms;
                pub mod instruction;
                pub mod instructions;
                pub mod memory;
                pub mod registers;
                pub mod sm83;
            }
        }
    }
    pub mod emulator {
        pub mod types;
    }
    pub mod gb {
        pub mod apu {
            pub mod apu;
            pub mod io;
            pub mod noise;
            pub mod sequencer;
            pub mod square_1;
            pub mod square_2;
            pub mod wave;
        }
        pub mod bus;
        pub mod cpu {
            pub mod cpu;
            pub mod memory;
            pub mod timing;
        }
    }
}
pub mod nall {
    pub mod random;
}
