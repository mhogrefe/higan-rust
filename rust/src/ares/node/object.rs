use ares::node::Node;

#[derive(Clone, Debug)]
pub struct Object {
    name: &'static str,
    nodes: Vec<Node>,
}

impl Default for Object {
    fn default() -> Object {
        Object {
            name: "Object",
            nodes: Vec::new(),
        }
    }
}
