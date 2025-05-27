use std::collections::HashMap;

use crate::{node::Node, token::TokenType};

use super::{Interpreter, InterpreterError, InterpreterResult};

impl Interpreter {
    pub fn visit_factors(
        &self,
        factors: &[(TokenType, Node)],
        ext: Option<&HashMap<char, f64>>,
    ) -> InterpreterResult<Node> {
        Ok({
            let mut ans = 1f64;
            let mut unresolved_factors = vec![];
            if factors
                .iter()
                .any(|(t, f)| f == &Node::Num(0f64) && t != &TokenType::Div)
            {
                return Ok(Node::Num(0f64));
            }
            let mut visited_factors = InterpreterResult::<Vec<(TokenType, Node)>>::from_iter(
                factors.iter().map(|(t, f)| Ok((*t, self.visit(f, ext)?))),
            )?;
            for (i, factor) in visited_factors.clone().iter().enumerate() {
                if let (TokenType::Mul, Node::Factors(inner)) = factor {
                    visited_factors.swap_remove(i);
                    visited_factors.append(&mut inner.clone());
                }
            }
            for (op, factor) in visited_factors {
                if let Node::Num(num) = factor {
                    match op {
                        TokenType::Mul => ans *= num,
                        TokenType::Div => {
                            if num == 0f64 {
                                return Err(InterpreterError::Undefined);
                            } else {
                                ans /= num;
                            }
                        }
                        _ => unreachable!(),
                    }
                } else {
                    unresolved_factors.push((op, factor));
                }
            }
            if unresolved_factors.is_empty() {
                Node::Num(ans)
            } else {
                if ans != 1f64 {
                    unresolved_factors.push((TokenType::Mul, Node::Num(ans)));
                } else if unresolved_factors.len() == 1
                    && unresolved_factors.first().unwrap().0 == TokenType::Mul
                {
                    return Ok(unresolved_factors.remove(0).1);
                }
                Node::Factors(unresolved_factors)
            }
        })
    }
}
