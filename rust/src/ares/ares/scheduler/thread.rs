use ares::gb::system::System;
use ares::platform::Platform;

impl<P: Platform> System<P> {
    pub fn cpu_is_sync_needed(&self) -> bool {
        false
    }
}
