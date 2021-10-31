use ares::node::input::button::Button;
use ares::node::input::Input;

#[derive(Clone, Debug)]
pub enum InputNode {
    Button(Button),
}

impl Default for InputNode {
    fn default() -> InputNode {
        InputNode::Button(Button::default())
    }
}

impl InputNode {
    pub fn as_button(self) -> Button {
        match self {
            InputNode::Button(b) => b,
        }
    }

    pub fn as_button_ref(&self) -> &Button {
        match self {
            InputNode::Button(ref b) => b,
        }
    }

    pub fn as_button_mut(&mut self) -> &mut Button {
        match self {
            InputNode::Button(ref mut b) => b,
        }
    }
}

#[derive(Clone, Debug)]
pub enum Node {
    Input(Input),
}

pub mod input;
pub mod object;
