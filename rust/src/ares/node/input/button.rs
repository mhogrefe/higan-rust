use ares::node::Node;

#[derive(Clone, Debug)]
pub struct Button {
    pub name: &'static str,
    pub nodes: Vec<Node>,
    pub value: bool,
}

impl Default for Button {
    fn default() -> Button {
        Button {
            name: "input.button",
            nodes: Vec::new(),
            value: false,
        }
    }
}
