use std::collections::HashMap;

use crate::node::Node;

use super::{Interpreter, InterpreterError, InterpreterResult};

impl Interpreter {
    pub fn visit_exponent(
        &self,
        base: &Node,
        exponent: &Node,
        ext: Option<&HashMap<char, f64>>,
    ) -> InterpreterResult<Node> {
        Ok({
            let visited_base = self.visit(base, ext)?;
            let visited_exponent = self.visit(exponent, ext)?;
            if let Node::Num(base_num) = visited_base {
                if let Node::Num(exponent_num) = visited_exponent {
                    if exponent_num == 0f64 {
                        if base_num == 0f64 {
                            return Err(InterpreterError::Undefined);
                        } else {
                            Node::Num(1f64)
                        }
                    } else if exponent_num == 1f64 {
                        base.clone()
                    } else if base_num == 1f64 {
                        Node::Num(1f64)
                    } else {
                        Node::Num(base_num.powf(exponent_num))
                    }
                } else if base_num == 0f64 {
                    Node::Num(0f64)
                } else if base_num == 1f64 {
                    Node::Num(1f64)
                } else {
                    Node::Exponent {
                        base: Box::new(visited_base),
                        exponent: Box::new(visited_exponent),
                    }
                }
            } else if let Node::Num(exponent_num) = visited_exponent {
                if exponent_num == 0f64 {
                    Node::Num(1f64)
                } else if exponent_num == 1f64 {
                    visited_base
                } else {
                    Node::Exponent {
                        base: Box::new(visited_base),
                        exponent: Box::new(visited_exponent),
                    }
                }
            } else {
                Node::Exponent {
                    base: Box::new(visited_base),
                    exponent: Box::new(visited_exponent),
                }
            }
        })
    }
}
