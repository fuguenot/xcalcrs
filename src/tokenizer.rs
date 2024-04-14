use std::fmt;

use crate::token::{FuncType, Token};

pub enum TokenizerError {
    IllegalChar(char),
}
impl fmt::Display for TokenizerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IllegalChar(ch) => f.write_fmt(format_args!("illegal character: {ch}")),
        }
    }
}

pub type TokenizerResult<T> = Result<T, TokenizerError>;

pub struct Tokenizer<'a> {
    text: &'a str,
    curr: usize,
}
impl<'a> Tokenizer<'a> {
    pub fn new(text: &'a str) -> Self {
        Self { text, curr: 0 }
    }

    fn rest(&self) -> &'a str {
        &self.text[self.curr..]
    }

    fn peek(&self) -> Option<char> {
        self.rest().chars().next()
    }

    fn advance(&mut self) {
        if let Some(c) = self.peek() {
            self.curr += c.len_utf8();
        }
    }

    fn advance_n(&mut self, n: usize) {
        for _ in 0..n {
            if let Some(c) = self.peek() {
                self.curr += c.len_utf8();
            }
        }
    }

    fn take_while<P>(&mut self, mut pred: P) -> Option<&'a str>
    where
        P: FnMut(char) -> bool,
    {
        let start = self.curr;
        while let Some(ch) = self.peek() {
            if !pred(ch) {
                break;
            }
            self.advance();
        }
        let end = self.curr;
        if start != end {
            Some(&self.text[start..end])
        } else {
            None
        }
    }

    pub fn tokenize(&mut self) -> TokenizerResult<Vec<Token>> {
        let mut tokens = vec![];
        while let Some(ch) = self.peek() {
            match ch {
                s if s.is_whitespace() => self.advance(),
                '0'..='9' => {
                    let mut point = false;
                    tokens.push(Token::Num(
                        self.take_while(|c| match c {
                            '.' if point => {
                                point = true;
                                true
                            }
                            '0'..='9' => true,
                            _ => false,
                        })
                        .expect("one digit already seen")
                        .parse()
                        .expect("parsed string should be a number"),
                    ));
                }
                c if c.is_alphabetic() => {
                    let rest = self.rest();
                    if rest.starts_with("sin") {
                        self.advance_n(3);
                        tokens.push(Token::Func(FuncType::Sin));
                    } else if rest.starts_with("cos") {
                        self.advance_n(3);
                        tokens.push(Token::Func(FuncType::Cos));
                    } else if rest.starts_with("tan") {
                        self.advance_n(3);
                        tokens.push(Token::Func(FuncType::Tan));
                    } else if rest.starts_with("csc") {
                        self.advance_n(3);
                        tokens.push(Token::Func(FuncType::Csc));
                    } else if rest.starts_with("sec") {
                        self.advance_n(3);
                        tokens.push(Token::Func(FuncType::Sec));
                    } else if rest.starts_with("cot") {
                        self.advance_n(3);
                        tokens.push(Token::Func(FuncType::Cot));
                    } else if rest.starts_with("ln") {
                        self.advance_n(2);
                        tokens.push(Token::Func(FuncType::Ln));
                    } else if rest.starts_with("log") {
                        self.advance_n(3);
                        tokens.push(Token::Func(FuncType::Log));
                    } else {
                        self.advance();
                        tokens.push(Token::Var(c));
                    }
                }
                '+' => {
                    self.advance();
                    tokens.push(Token::Plus);
                }
                '-' => {
                    self.advance();
                    tokens.push(Token::Minus);
                }
                '*' => {
                    self.advance();
                    tokens.push(Token::Mul);
                }
                '/' => {
                    self.advance();
                    tokens.push(Token::Div);
                }
                '^' => {
                    self.advance();
                    tokens.push(Token::Raise);
                }
                '(' => {
                    self.advance();
                    tokens.push(Token::LParen);
                }
                ')' => {
                    self.advance();
                    tokens.push(Token::RParen);
                }
                '=' => {
                    self.advance();
                    tokens.push(Token::Equals);
                }
                o => return Err(TokenizerError::IllegalChar(o)),
            }
        }
        tokens.push(Token::Eof);
        Ok(tokens)
    }
}
