use std::collections::HashMap;

use crate::{node::Node, token::FuncType};

use super::{is_int, Interpreter, InterpreterError, InterpreterResult};

impl Interpreter {
    pub fn visit_func(
        &self,
        func: &FuncType,
        arg: &Node,
        ext: Option<&HashMap<char, f64>>,
    ) -> InterpreterResult<Node> {
        Ok({
            let visited_arg = self.visit(arg, ext)?;
            if let Node::Num(num) = visited_arg {
                Node::Num(match func {
                    FuncType::Sin => num.sin(),
                    FuncType::Cos => num.cos(),
                    FuncType::Tan => num.tan(),
                    FuncType::Csc => {
                        if is_int(num / std::f64::consts::PI) {
                            return Err(InterpreterError::Undefined);
                        } else {
                            1f64 / num.sin()
                        }
                    }
                    FuncType::Sec => 1f64 / num.cos(),
                    FuncType::Cot => 1f64 / num.tan(),
                    FuncType::Ln => {
                        if num == 0f64 {
                            return Err(InterpreterError::NegInfinity);
                        } else {
                            num.ln()
                        }
                    }
                    FuncType::Log => {
                        if num == 0f64 {
                            return Err(InterpreterError::Infinity);
                        } else {
                            num.log10()
                        }
                    }
                })
            } else {
                Node::Func {
                    func: func.clone(),
                    arg: Box::new(visited_arg),
                }
            }
        })
    }
}
