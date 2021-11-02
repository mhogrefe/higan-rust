use ares::gb::system::System;
use ares::platform::Platform;

#[derive(Clone, Debug, Default)]
pub struct Thread {
    pub unique_id: u32,
    pub frequency: u64,
    pub scalar: u64,
    pub clock: u64,
}

impl Thread {
    pub fn step(&mut self, clocks: u32) {
        self.clock += self.scalar * u64::from(clocks);
    }
}

impl<P: Platform> System<P> {
    pub fn apu_is_sync_needed(&self) -> bool {
        self.cpu_thread.clock < self.apu_thread.clock && !self.scheduler.synchronizing()
    }

    pub fn cpu_is_sync_needed(&self) -> bool {
        self.apu_thread.clock < self.cpu_thread.clock && !self.scheduler.synchronizing()
    }

    /*
        //ensure all threads are caught up to the current thread before proceeding.
    inline auto Thread::synchronize() -> void {
        //note: this will call Thread::synchronize(*this) at some point, but this is safe:
        //the comparison will always fail as the current thread can never be behind itself.
        for(auto thread : scheduler._threads) synchronize(*thread);
      }*/

    /*
    //ensure the specified thread(s) are caught up the current thread before proceeding.
    pub fn apu_synchronize_cpu(Thread& thread, P&&... p) -> void {
      //switching to another thread does not guarantee it will catch up before switching back.
      while(thread.clock() < clock()) {
        //disable synchronization for auxiliary threads during scheduler synchronization.
        //synchronization can begin inside of this while loop.
        if(scheduler.synchronizing()) break;
        co_switch(thread.handle());
      }
      //convenience: allow synchronizing multiple threads with one function call.
      if constexpr(sizeof...(p) > 0) synchronize(forward<P>(p)...);
    }*/
}
