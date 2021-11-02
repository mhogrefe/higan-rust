use ares::platform::Event;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Mode {
    Run,
    Synchronize,
    SynchronizePrimary,
    SynchronizeAuxiliary,
}

impl Default for Mode {
    fn default() -> Mode {
        Mode::Run
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Scheduler {
    mode: Mode,
    event: Event,
    synchronize: bool,
}

impl Default for Scheduler {
    fn default() -> Scheduler {
        Scheduler {
            mode: Mode::Run,
            event: Event::Step,
            synchronize: false,
        }
    }
}

impl Scheduler {
    pub fn synchronizing(&self) -> bool {
        self.mode == Mode::SynchronizeAuxiliary
    }
}
