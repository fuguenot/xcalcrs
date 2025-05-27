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

    pub fn solve_equation(&self, eq: &Node, guess: f64) -> InterpreterResult<f64> {
        let f = self.move_equation(eq)?;
        let derivative = self.visit(&self.differentiate(&f, 'x', None)?, None)?;
        let mut solution = guess;
        let mut error = None;
        let mut step;
        let mut map = HashMap::new();
        while error.is_none_or(|e| e > 0.0001) {
            map.insert('x', solution);
            step = self.visit(
                &Node::Factors(vec![
                    (TokenType::Mul, Node::Num(-1f64)),
                    (TokenType::Mul, self.visit(&f, Some(&map))?),
                    (TokenType::Div, self.visit(&derivative, Some(&map))?),
                ]),
                Some(&map),
            )?;
            if let Node::Num(h) = step {
                solution += h;
                error = Some(h);
            } else {
                return Err(InterpreterError::SolveError(String::from(
                    "Not substituted",
                )));
            }
        }

        Ok(solution)
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
        let mut error = None;
        let mut h;
        let mut map = HashMap::new();
        let mut val;
        while error.is_none_or(|e| e > 0.00000001) {
            map.insert('x', solution.0);
            map.insert('y', solution.1);
            val = (self.visit(&f1, Some(&map))?, self.visit(&f2, Some(&map))?);
            h = (
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
            if let Node::Num(hx) = h.0 {
                if let Node::Num(hy) = h.1 {
                    error = Some(hx.powi(2) + hy.powi(2));
                    solution = (solution.0 + hx, solution.1 + hy);
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
        }
        Ok(solution)
    }
}
