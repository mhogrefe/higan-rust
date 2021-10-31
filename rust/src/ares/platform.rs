use ares::node::InputNode;

pub trait Platform {
    fn input(&mut self, input: &mut InputNode);
}

#[derive(Clone, Copy, Debug, Default)]
pub struct NullPlatform;

impl Platform for NullPlatform {
    fn input(&mut self, _input: &mut InputNode) {}
}
