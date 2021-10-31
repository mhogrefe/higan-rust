use ares::gb::system::System;
use ares::platform::Platform;

impl<P: Platform> System<P> {
    pub fn cpu_step(&mut self, _clocks: u32) {
        unimplemented!();
    }
}
