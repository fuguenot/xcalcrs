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

#[derive(Debug, Clone, PartialEq, Eq)]
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
