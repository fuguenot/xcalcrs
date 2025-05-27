use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
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
impl fmt::Display for FuncType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Sin => "sin",
            Self::Cos => "cos",
            Self::Tan => "tan",
            Self::Csc => "csc",
            Self::Sec => "sec",
            Self::Cot => "cot",
            Self::Ln => "ln",
            Self::Log => "log",
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
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
impl PartialEq<TokenType> for Token {
    fn eq(&self, other: &TokenType) -> bool {
        other == &TokenType::from(self.clone())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    Eof,
    Num,
    Var,
    Func,
    LParen,
    RParen,
    Plus,
    Minus,
    Mul,
    Div,
    Raise,
    Equals,
}
impl From<Token> for TokenType {
    fn from(value: Token) -> Self {
        match value {
            Token::Eof => Self::Eof,
            Token::Num(_) => Self::Num,
            Token::Var(_) => Self::Var,
            Token::Func(_) => Self::Func,
            Token::LParen => Self::LParen,
            Token::RParen => Self::RParen,
            Token::Plus => Self::Plus,
            Token::Minus => Self::Minus,
            Token::Mul => Self::Mul,
            Token::Div => Self::Div,
            Token::Raise => Self::Raise,
            Token::Equals => Self::Equals,
        }
    }
}
impl From<&Token> for TokenType {
    fn from(value: &Token) -> Self {
        value.clone().into()
    }
}
