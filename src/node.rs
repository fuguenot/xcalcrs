use std::fmt::{self, Write};

use crate::token::{FuncType, TokenType};

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Num(f64),
    Var(char),
    Func {
        func: FuncType,
        arg: Box<Self>,
    },
    Exponent {
        base: Box<Self>,
        exponent: Box<Self>,
    },
    Factors(Vec<(TokenType, Self)>),
    Terms(Vec<(TokenType, Self)>),
    Derivative {
        derivative: Box<Self>,
        var: char,
    },
    Equation {
        lhs: Box<Self>,
        rhs: Box<Self>,
    },
}
impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Num(n) => write!(f, "{n}"),
            Self::Var(c) => write!(f, "{c}"),
            Self::Func { func, arg } => write!(f, "{func}({arg})"),
            Self::Exponent { base, exponent } => {
                match base.as_ref() {
                    Node::Factors(_) | Node::Terms(_) | Node::Derivative { .. } => {
                        write!(f, "({base})")?
                    }
                    _ => write!(f, "{base}")?,
                }
                f.write_char('^')?;
                match exponent.as_ref() {
                    Node::Factors(_) | Node::Terms(_) | Node::Derivative { .. } => {
                        write!(f, "({exponent})")
                    }
                    _ => write!(f, "{exponent}"),
                }
            }
            Self::Factors(factors) => {
                for (i, factor) in factors.iter().enumerate() {
                    if i > 0 {
                        f.write_char(match factor.0 {
                            TokenType::Mul => '*',
                            TokenType::Div => '/',
                            _ => unreachable!(),
                        })?;
                    }
                    match &factor.1 {
                        Node::Factors(_) | Node::Terms(_) | Node::Derivative { .. } => {
                            write!(f, "({})", factor.1)?
                        }
                        e => write!(f, "{e}")?,
                    }
                }
                Ok(())
            }
            Self::Terms(terms) => {
                for (i, term) in terms.iter().enumerate() {
                    if i > 0 || term.0 == TokenType::Minus {
                        f.write_char(match term.0 {
                            TokenType::Plus => '+',
                            TokenType::Minus => '-',
                            _ => unreachable!(),
                        })?;
                    }
                    match &term.1 {
                        Node::Factors(_) | Node::Terms(_) | Node::Derivative { .. } => {
                            write!(f, "({})", term.1)?
                        }
                        e => write!(f, "{e}")?,
                    }
                }
                Ok(())
            }
            Self::Derivative { derivative, var } => write!(f, "d/d{var}[{derivative}]"),
            Self::Equation { lhs, rhs } => write!(f, "{lhs} = {rhs}"),
        }
    }
}
