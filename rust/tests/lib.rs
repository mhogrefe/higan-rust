extern crate higan_rust;
extern crate malachite_base;

pub mod ares {
    pub mod component {
        pub mod processor {
            pub mod sm83 {
                pub mod algorithms;
                pub mod registers;
                pub mod sm83;
            }
        }
    }
    pub mod gb {
        pub mod apu {
            pub mod apu;
            pub mod io;
            pub mod noise;
            pub mod sequencer;
            pub mod square1;
            pub mod square2;
            pub mod wave;
        }
    }
}
