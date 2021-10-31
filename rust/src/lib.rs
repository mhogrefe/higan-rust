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
        pub mod apu;
        pub mod bus;
        pub mod cpu;
        pub mod system;
    }
    pub mod node;
    pub mod platform;
}
pub mod nall {
    pub mod random;
}
