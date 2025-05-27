use std::fmt;

use crate::{
    node::Node,
    token::{Token, TokenType},
};

#[derive(Debug)]
pub enum ParserError {
    Expected { expected: TokenType, got: TokenType },
    Unexpected(TokenType),
    DifferentialEquation,
}
impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Expected { expected, got } => {
                f.write_fmt(format_args!("expected {expected:?}, got {got:?}"))
            }
            Self::Unexpected(unexpected) => f.write_fmt(format_args!("unexpected {unexpected:?}")),
            Self::DifferentialEquation => f.write_str("differential equations not supported yet"),
        }
    }
}

pub type ParserResult<T> = Result<T, ParserError>;

pub struct Parser<'a> {
    tokens: &'a [Token],
    curr: usize,
}
impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Self { tokens, curr: 0 }
    }

    fn rest(&self) -> &'a [Token] {
        self.tokens.get(self.curr..).unwrap()
    }

    fn peek(&self) -> Option<Token> {
        self.rest().iter().next().cloned()
    }

    fn advance(&mut self) {
        if self.curr < self.tokens.len() - 1 {
            self.curr += 1;
        }
    }

    fn retract(&mut self) {
        if self.curr > 0 {
            self.curr -= 1;
        }
    }

    fn retract_n(&mut self, n: usize) {
        for _ in 0..n {
            self.retract();
        }
    }

    fn accept(&mut self, token_type: &TokenType) -> Option<Token> {
        let token = self.peek()?;
        if &token == token_type {
            self.advance();
            Some(token)
        } else {
            None
        }
    }

    fn accept_token(&mut self, token: &Token) -> bool {
        if let Some(tok) = self.accept(&TokenType::from(token)) {
            if token == &tok {
                true
            } else {
                self.retract();
                false
            }
        } else {
            false
        }
    }

    fn expect(&mut self, token_type: &TokenType) -> ParserResult<Token> {
        self.accept(token_type).ok_or(ParserError::Expected {
            expected: *token_type,
            got: self.peek().unwrap().into(),
        })
    }

    fn parse_func(&mut self) -> ParserResult<Node> {
        if let Token::Func(func) = self.expect(&TokenType::Func)? {
            self.expect(&TokenType::LParen)?;
            let arg = Box::new(self.parse_expr()?);
            self.expect(&TokenType::RParen)?;
            Ok(Node::Func { func, arg })
        } else {
            unreachable!()
        }
    }

    fn parse_atom(&mut self) -> ParserResult<Node> {
        if let Some(Token::Var(var)) = self.accept(&TokenType::Var) {
            Ok(Node::Var(var))
        } else if self.accept(&TokenType::LParen).is_some() {
            let node = self.parse_expr()?;
            self.expect(&TokenType::RParen)?;
            Ok(node)
        } else if self.accept(&TokenType::Func).is_some() {
            self.retract();
            self.parse_func()
        } else {
            Err(ParserError::Unexpected(self.peek().unwrap().into()))
        }
    }

    fn parse_quantity(&mut self) -> ParserResult<Node> {
        if let Some(Token::Num(num)) = self.accept(&TokenType::Num) {
            Ok(Node::Num(num))
        } else {
            self.parse_atom()
        }
    }

    fn parse_factor(&mut self) -> ParserResult<Node> {
        let node = self.parse_quantity()?;
        Ok(if self.accept(&TokenType::Raise).is_some() {
            Node::Exponent {
                base: Box::new(node),
                exponent: Box::new(self.parse_factor()?),
            }
        } else {
            node
        })
    }

    fn parse_factors(&mut self) -> ParserResult<Node> {
        let mut factors = vec![];
        factors.push((TokenType::Mul, self.parse_factor()?));
        while self.accept(&TokenType::Var).is_some()
            || self.accept(&TokenType::Func).is_some()
            || self.accept(&TokenType::LParen).is_some()
        {
            self.retract();
            factors.push((TokenType::Mul, self.parse_atom()?));
        }
        Ok(Node::Factors(factors))
    }

    fn parse_term(&mut self) -> ParserResult<Node> {
        let mut factors = vec![];
        if let Node::Factors(mut factors_factors) = self.parse_factors()? {
            factors.append(&mut factors_factors);
        }
        loop {
            if self.accept(&TokenType::Mul).is_some() {
                if let Node::Factors(mut factors_factors) = self.parse_factors()? {
                    factors.append(&mut factors_factors);
                }
            } else if self.accept(&TokenType::Div).is_some() {
                factors.push((TokenType::Div, self.parse_factors()?));
            } else {
                break;
            }
        }
        Ok(Node::Factors(factors))
    }

    fn parse_expr(&mut self) -> ParserResult<Node> {
        let mut terms = vec![];

        let sign = self
            .accept(&TokenType::Minus)
            .map_or(TokenType::Plus, |_| TokenType::Minus);
        terms.push((sign, self.parse_term()?));
        loop {
            if self.accept(&TokenType::Plus).is_some() {
                terms.push((TokenType::Plus, self.parse_term()?));
            } else if self.accept(&TokenType::Minus).is_some() {
                terms.push((TokenType::Minus, self.parse_term()?));
            } else {
                break;
            }
        }
        Ok(Node::Terms(terms))
    }

    fn parse_derivative(&mut self) -> ParserResult<Node> {
        if self.accept_token(&Token::Var('d')) {
            if self.accept(&TokenType::Div).is_some() {
                if self.accept_token(&Token::Var('d')) {
                    if let Some(Token::Var(var)) = self.accept(&TokenType::Var) {
                        self.expect(&TokenType::LParen)?;
                        let derivative = Box::new(self.parse_derivative()?);
                        self.expect(&TokenType::RParen)?;
                        return Ok(Node::Derivative { derivative, var });
                    } else {
                        self.retract_n(3);
                    }
                } else {
                    self.retract_n(2);
                }
            } else {
                self.retract();
            }
        }
        self.parse_expr()
    }

    pub fn parse(&mut self) -> ParserResult<Node> {
        let mut node = self.parse_derivative()?;
        if self.accept(&TokenType::Equals).is_some() {
            if let Node::Derivative { .. } = node {
                return Err(ParserError::DifferentialEquation);
            } else {
                node = Node::Equation {
                    lhs: Box::new(node),
                    rhs: Box::new(self.parse_expr()?),
                };
            }
        }
        self.expect(&TokenType::Eof)?;
        Ok(node)
    }
}
