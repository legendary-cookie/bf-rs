#[derive(Debug)]
pub enum Node {
    Forward,
    Backward,
    Increment,
    Decrement,
    Output,
    Input,
    Loop { children: Vec<Node> },
}
