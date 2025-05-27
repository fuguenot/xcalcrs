use std::collections::HashMap;

use crate::{node::Node, token::TokenType};

use super::{Interpreter, InterpreterError, InterpreterResult};

impl Interpreter {
    fn move_equation(&self, eq: &Node) -> InterpreterResult<Node> {
        if let Node::Equation { lhs, rhs } = eq {
            Ok(Node::Terms(vec![
                (TokenType::Plus, *lhs.clone()),
                (TokenType::Minus, *rhs.clone()),
            ]))
        } else {
            Err(InterpreterError::SolveError(String::from(
                "Not an equation",
            )))
        }
    }

    pub fn solve_system(
        &self,
        eq1: &Node,
        eq2: &Node,
        guess: (f64, f64),
    ) -> InterpreterResult<(f64, f64)> {
        let f1 = self.move_equation(eq1)?;
        let f2 = self.move_equation(eq2)?;
        let jacobian: ((Node, Node), (Node, Node)) = (
            (
                self.visit(&self.differentiate(&f1, 'x', None)?, None)?,
                self.visit(&self.differentiate(&f1, 'y', None)?, None)?,
            ),
            (
                self.visit(&self.differentiate(&f2, 'x', None)?, None)?,
                self.visit(&self.differentiate(&f2, 'y', None)?, None)?,
            ),
        );
        let det = self.visit(
            &Node::Terms(vec![
                (
                    TokenType::Plus,
                    Node::Factors(vec![
                        (TokenType::Mul, jacobian.0 .0.clone()),
                        (TokenType::Mul, jacobian.1 .1.clone()),
                    ]),
                ),
                (
                    TokenType::Minus,
                    Node::Factors(vec![
                        (TokenType::Mul, jacobian.0 .1.clone()),
                        (TokenType::Mul, jacobian.1 .0.clone()),
                    ]),
                ),
            ]),
            None,
        )?;
        let inv = (
            (
                self.visit(
                    &Node::Factors(vec![
                        (TokenType::Mul, jacobian.1 .1),
                        (TokenType::Div, det.clone()),
                    ]),
                    None,
                )?,
                self.visit(
                    &Node::Factors(vec![
                        (TokenType::Mul, Node::Num(-1f64)),
                        (TokenType::Mul, jacobian.0 .1),
                        (TokenType::Div, det.clone()),
                    ]),
                    None,
                )?,
            ),
            (
                self.visit(
                    &Node::Factors(vec![
                        (TokenType::Mul, Node::Num(-1f64)),
                        (TokenType::Mul, jacobian.1 .0),
                        (TokenType::Div, det.clone()),
                    ]),
                    None,
                )?,
                self.visit(
                    &Node::Factors(vec![(TokenType::Mul, jacobian.0 .0), (TokenType::Div, det)]),
                    None,
                )?,
            ),
        );
        let mut solution = guess;
        let mut step: Option<(f64, f64)> = None;
        let mut try_step: (Node, Node);
        let mut map = HashMap::new();
        while step.is_none() || step.unwrap().0.powi(2) + step.unwrap().1.powi(2) > 0.00000001 {
            map.insert('x', solution.0);
            map.insert('y', solution.1);
            let val: (Node, Node) = (self.visit(&f1, Some(&map))?, self.visit(&f2, Some(&map))?);
            try_step = (
                self.visit(
                    &Node::Terms(vec![
                        (
                            TokenType::Plus,
                            Node::Factors(vec![
                                (TokenType::Mul, Node::Num(-1f64)),
                                (TokenType::Mul, inv.0 .0.clone()),
                                (TokenType::Mul, val.0.clone()),
                            ]),
                        ),
                        (
                            TokenType::Plus,
                            Node::Factors(vec![
                                (TokenType::Mul, Node::Num(-1f64)),
                                (TokenType::Mul, inv.0 .1.clone()),
                                (TokenType::Mul, val.1.clone()),
                            ]),
                        ),
                    ]),
                    Some(&map),
                )?,
                self.visit(
                    &Node::Terms(vec![
                        (
                            TokenType::Plus,
                            Node::Factors(vec![
                                (TokenType::Mul, Node::Num(-1f64)),
                                (TokenType::Mul, inv.1 .0.clone()),
                                (TokenType::Mul, val.0.clone()),
                            ]),
                        ),
                        (
                            TokenType::Plus,
                            Node::Factors(vec![
                                (TokenType::Mul, Node::Num(-1f64)),
                                (TokenType::Mul, inv.1 .1.clone()),
                                (TokenType::Mul, val.1.clone()),
                            ]),
                        ),
                    ]),
                    Some(&map),
                )?,
            );
            if let Node::Num(x) = try_step.0 {
                if let Node::Num(y) = try_step.1 {
                    step = Some((x, y));
                } else {
                    return Err(InterpreterError::SolveError(String::from(
                        "Not substituted",
                    )));
                }
            } else {
                return Err(InterpreterError::SolveError(String::from(
                    "Not substituted",
                )));
            }
            solution = (solution.0 + step.unwrap().0, solution.1 + step.unwrap().1);
        }
        Ok(solution)
    }
}
