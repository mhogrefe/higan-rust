auto System::Controls::load(Node::Object parent) -> void {
  node = parent->append<Node::Object>("Controls");

  reset      = node->append<Node::Input::Button>("Reset");
  microphone = node->append<Node::Input::Button>("Microphone");
}

auto System::Controls::poll() -> void {
  platform->input(reset);
  platform->input(microphone);
}
