#[derive(Debug)]
pub enum FuncType {
    Sin,
    Cos,
    Tan,
    Csc,
    Sec,
    Cot,
    Ln,
    Log,
}

#[derive(Debug)]
pub enum Token {
    Eof,
    Num(f64),
    Var(char),
    Func(FuncType),
    LParen,
    RParen,
    Plus,
    Minus,
    Mul,
    Div,
    Raise,
    Equals,
}
