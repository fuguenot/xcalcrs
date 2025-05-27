use std::collections::HashMap;

use crate::{node::Node, token::TokenType};

use super::{Interpreter, InterpreterResult};

impl Interpreter {
    pub fn visit_terms(
        &self,
        terms: &[(TokenType, Node)],
        ext: Option<&HashMap<char, f64>>,
    ) -> InterpreterResult<Node> {
        Ok({
            let mut ans = 0f64;
            let mut unresolved_terms = vec![];
            for (op, term) in terms.iter().map(|(o, t)| (o, self.visit(t, ext))) {
                if let Node::Num(num) = term.clone()? {
                    match op {
                        TokenType::Plus => ans += num,
                        TokenType::Minus => ans -= num,
                        _ => unreachable!(),
                    }
                } else {
                    unresolved_terms.push((*op, term.clone()?));
                }
            }
            if unresolved_terms.is_empty() {
                Node::Num(ans)
            } else {
                if ans != 0f64 {
                    unresolved_terms.push((TokenType::Plus, Node::Num(ans)));
                } else if unresolved_terms.len() == 1
                    && unresolved_terms.first().unwrap().0 == TokenType::Plus
                {
                    return Ok(unresolved_terms.remove(0).1);
                }
                Node::Terms(unresolved_terms)
            }
        })
    }
}
