use ares::node::Node;

#[derive(Clone, Debug)]
pub struct Input {
    name: &'static str,
    nodes: Vec<Node>,
}

impl Default for Input {
    fn default() -> Input {
        Input {
            name: "input",
            nodes: Vec::new(),
        }
    }
}

pub mod button;
