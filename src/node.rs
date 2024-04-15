use crate::token::{FuncType, TokenType};

#[derive(Debug, Clone)]
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
