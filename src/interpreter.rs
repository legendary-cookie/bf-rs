use std::io::Read;
use std::num::Wrapping;

use crate::parser::ast::Node;

fn overflow_add(lhs: u8, rhs: u8) -> u8 {
    let x = Wrapping(lhs);
    let y = Wrapping(rhs);

    (x + y).0
}

fn read_byte() -> Option<u8> {
    std::io::stdin()
        .bytes()
        .next()
        .and_then(|result| result.ok())
}


pub struct Interpreter {
    memory: [u8; 30000],
    position: u16,
}

impl Interpreter {
    pub fn new() -> Self {
        return Self {
            memory: [0; 30000],
            position: 0,
        };
    }

    pub fn run(&mut self, nodes: &Vec<Node>) {
        for node in nodes {
            match node {
                Node::Forward => self.position += 1,
                Node::Backward => self.position -= 1,
                Node::Increment => {
                    self.memory[self.position as usize] =
                        overflow_add(self.memory[self.position as usize], 1)
                }
                Node::Decrement => {
                    self.memory[self.position as usize] =
                        overflow_add(self.memory[self.position as usize], 255)
                }
                Node::Output => {
                    print!("{}", self.memory[self.position as usize] as char)
                }
                Node::Loop { children } => {
                    self.run(&children);
                    while self.memory[self.position as usize] != 0 {
                        self.run(&children)
                    }
                }
                Node::Input => match read_byte() {
                    Some(byte) => self.memory[self.position as usize] = byte,
                    None => {}
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::ast::Node;

    use super::Interpreter;

    #[test]
    fn test_increment() {
        let program = vec![Node::Increment];
        let mut interpreter = Interpreter::new();
        interpreter.run(&program);
        assert!(interpreter.memory[0] == 1)
    }

    #[test]
    fn test_decrement() {
        let program = vec![Node::Decrement];
        let mut interpreter = Interpreter::new();

        interpreter.memory[0] = 1;

        interpreter.run(&program);
        assert!(interpreter.memory[0] == 0);
    }

    #[test]
    fn test_forward() {
        let program = vec![Node::Forward];
        let mut interpreter = Interpreter::new();
        interpreter.run(&program);
        assert!(interpreter.position == 1);
    }

    #[test]
    fn test_backward() {
        let program = vec![Node::Backward];

        let mut interpreter = Interpreter::new();
        interpreter.position = 1;

        interpreter.run(&program);
        assert!(interpreter.position == 0);
    }

    #[test]
    fn test_loop() {
        let program = vec![
            Node::Loop {
                children: vec![Node::Increment],
            },
            Node::Forward,
            Node::Decrement,
        ];

        let mut interpreter = Interpreter::new();

        interpreter.run(&program);
        assert_eq!(interpreter.position, 1);
        assert_eq!(interpreter.memory[0], 0);
        assert_eq!(interpreter.memory[1], 255);
    }
}
