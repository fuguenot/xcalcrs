use std::{collections::HashMap, fmt};

use crate::{
    node::Node,
    token::{FuncType, TokenType},
};

use super::{Interpreter, InterpreterError, InterpreterResult};

#[derive(Debug, Clone)]
pub enum DifferentiatorError {
    Equation,
}
impl fmt::Display for DifferentiatorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Equation => "cannot perform differentiation on an equation",
        })
    }
}

impl Interpreter {
    pub fn differentiate(
        &self,
        node: &Node,
        var: char,
        ext: Option<&HashMap<char, f64>>,
    ) -> InterpreterResult<Node> {
        Ok(match node {
            Node::Num(_) => Node::Num(0f64),
            Node::Var(ch) => {
                if ch == &var {
                    Node::Num(1f64)
                } else {
                    Node::Num(0f64)
                }
            }
            Node::Func { func, arg } => {
                let visited_arg = self.visit(arg, ext)?;
                self.visit(
                    &Node::Factors(vec![
                        (TokenType::Mul, self.differentiate(&visited_arg, var, ext)?),
                        (
                            TokenType::Mul,
                            match func {
                                FuncType::Sin => Node::Func {
                                    func: FuncType::Cos,
                                    arg: Box::new(visited_arg),
                                },
                                FuncType::Cos => Node::Factors(vec![
                                    (TokenType::Mul, Node::Num(-1f64)),
                                    (
                                        TokenType::Mul,
                                        Node::Func {
                                            func: FuncType::Sin,
                                            arg: Box::new(visited_arg),
                                        },
                                    ),
                                ]),
                                FuncType::Tan => Node::Exponent {
                                    base: Box::new(Node::Func {
                                        func: FuncType::Sec,
                                        arg: Box::new(visited_arg),
                                    }),
                                    exponent: Box::new(Node::Num(2f64)),
                                },
                                FuncType::Csc => Node::Factors(vec![
                                    (TokenType::Mul, Node::Num(-1f64)),
                                    (
                                        TokenType::Mul,
                                        Node::Func {
                                            func: FuncType::Csc,
                                            arg: Box::new(visited_arg.clone()),
                                        },
                                    ),
                                    (
                                        TokenType::Mul,
                                        Node::Func {
                                            func: FuncType::Cot,
                                            arg: Box::new(visited_arg),
                                        },
                                    ),
                                ]),
                                FuncType::Sec => Node::Factors(vec![
                                    (
                                        TokenType::Mul,
                                        Node::Func {
                                            func: FuncType::Sec,
                                            arg: Box::new(visited_arg.clone()),
                                        },
                                    ),
                                    (
                                        TokenType::Mul,
                                        Node::Func {
                                            func: FuncType::Tan,
                                            arg: Box::new(visited_arg),
                                        },
                                    ),
                                ]),
                                FuncType::Cot => Node::Factors(vec![
                                    (TokenType::Mul, Node::Num(-1f64)),
                                    (
                                        TokenType::Mul,
                                        Node::Exponent {
                                            base: Box::new(Node::Func {
                                                func: FuncType::Sec,
                                                arg: Box::new(visited_arg),
                                            }),
                                            exponent: Box::new(Node::Num(2f64)),
                                        },
                                    ),
                                ]),
                                FuncType::Ln => Node::Factors(vec![(TokenType::Div, visited_arg)]),
                                FuncType::Log => Node::Factors(vec![(
                                    TokenType::Div,
                                    Node::Factors(vec![
                                        (TokenType::Mul, visited_arg),
                                        (
                                            TokenType::Mul,
                                            Node::Func {
                                                func: FuncType::Ln,
                                                arg: Box::new(Node::Num(10f64)),
                                            },
                                        ),
                                    ]),
                                )]),
                            },
                        ),
                    ]),
                    ext,
                )?
            }
            Node::Exponent { base, exponent } => {
                let visited_base = self.visit(base, ext)?;
                let visited_exponent = self.visit(exponent, ext)?;
                if let Node::Num(num) = visited_exponent {
                    Node::Factors(vec![
                        (TokenType::Mul, self.differentiate(&visited_base, var, ext)?),
                        (TokenType::Mul, Node::Num(num)),
                        (
                            TokenType::Mul,
                            Node::Exponent {
                                base: Box::new(visited_base),
                                exponent: Box::new(Node::Num(num - 1f64)),
                            },
                        ),
                    ])
                } else if let Node::Num(num) = visited_base {
                    Node::Factors(vec![
                        (
                            TokenType::Mul,
                            self.differentiate(&visited_exponent, var, ext)?,
                        ),
                        (TokenType::Mul, node.clone()),
                        (
                            TokenType::Mul,
                            Node::Func {
                                func: FuncType::Ln,
                                arg: Box::new(Node::Num(num)),
                            },
                        ),
                    ])
                } else {
                    Node::Factors(vec![
                        (TokenType::Mul, node.clone()),
                        (
                            TokenType::Mul,
                            Node::Terms(vec![
                                (
                                    TokenType::Plus,
                                    Node::Factors(vec![
                                        (
                                            TokenType::Mul,
                                            self.differentiate(&visited_exponent, var, ext)?,
                                        ),
                                        (
                                            TokenType::Mul,
                                            Node::Func {
                                                func: FuncType::Ln,
                                                arg: Box::new(visited_base.clone()),
                                            },
                                        ),
                                    ]),
                                ),
                                (
                                    TokenType::Plus,
                                    Node::Factors(vec![
                                        (
                                            TokenType::Mul,
                                            self.differentiate(&visited_base, var, ext)?,
                                        ),
                                        (TokenType::Mul, visited_exponent),
                                        (TokenType::Div, visited_base),
                                    ]),
                                ),
                            ]),
                        ),
                    ])
                }
            }
            Node::Factors(factors) => {
                let visited_factors = InterpreterResult::<Vec<(TokenType, Node)>>::from_iter(
                    factors.iter().map(|(t, f)| Ok((*t, self.visit(f, ext)?))),
                )?;
                let (mul, div): (Vec<_>, Vec<_>) = visited_factors
                    .iter()
                    .partition(|(t, _)| t == &TokenType::Mul);
                let mut res = Node::Terms(InterpreterResult::<Vec<(TokenType, Node)>>::from_iter(
                    mul.iter().enumerate().map(|(i1, (_, factor1))| {
                        Ok((
                            TokenType::Plus,
                            Node::Factors(InterpreterResult::<Vec<(TokenType, Node)>>::from_iter(
                                mul.iter().enumerate().map(|(i2, (_, factor2))| {
                                    if i1 == i2 {
                                        self.differentiate(factor1, var, ext)
                                            .map(|r| (TokenType::Mul, r))
                                    } else {
                                        Ok((TokenType::Mul, factor2.clone()))
                                    }
                                }),
                            )?),
                        ))
                    }),
                )?);
                for (_, divisor) in div {
                    res = Node::Factors(vec![
                        (
                            TokenType::Mul,
                            Node::Terms(vec![
                                (
                                    TokenType::Plus,
                                    Node::Factors(vec![
                                        (TokenType::Mul, self.differentiate(&res, var, ext)?),
                                        (TokenType::Mul, divisor.clone()),
                                    ]),
                                ),
                                (
                                    TokenType::Minus,
                                    Node::Factors(vec![
                                        (TokenType::Mul, self.differentiate(divisor, var, ext)?),
                                        (TokenType::Mul, res.clone()),
                                    ]),
                                ),
                            ]),
                        ),
                        (
                            TokenType::Div,
                            Node::Exponent {
                                base: Box::new(divisor.clone()),
                                exponent: Box::new(Node::Num(2f64)),
                            },
                        ),
                    ]);
                }
                res
            }
            Node::Terms(terms) => Node::Terms(InterpreterResult::from_iter(terms.iter().map(
                |(t, term)| Ok((*t, self.differentiate(&self.visit(term, ext)?, var, ext)?)),
            ))?),
            Node::Derivative {
                derivative,
                var: var2,
            } => self.differentiate(
                &self.visit(&self.differentiate(derivative, *var2, ext)?, ext)?,
                var,
                ext,
            )?,
            Node::Equation { .. } => {
                return Err(InterpreterError::DifferentiatorError(
                    DifferentiatorError::Equation,
                ))
            }
        })
    }
}
