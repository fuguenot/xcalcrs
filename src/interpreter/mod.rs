use std::{collections::HashMap, f64, fmt};

use crate::node::Node;

use self::differentiator::DifferentiatorError;

mod differentiator;
mod newton;
mod visit_exponent;
mod visit_factors;
mod visit_func;
mod visit_terms;

#[derive(Debug, Clone)]
pub enum InterpreterError {
    Undefined,
    Infinity,
    NegInfinity,
    DifferentiatorError(DifferentiatorError),
    SolveError(String),
}
impl fmt::Display for InterpreterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Undefined => "undefined",
            Self::Infinity => "infinity",
            Self::NegInfinity => "-infinity",
            Self::DifferentiatorError(err) => return err.fmt(f),
            Self::SolveError(s) => s,
        })
    }
}

pub type InterpreterResult<T> = Result<T, InterpreterError>;

fn is_int(n: f64) -> bool {
    n == (n as u32) as f64
}

pub struct Interpreter {
    table: HashMap<char, f64>,
}
impl Interpreter {
    pub fn new() -> Self {
        let mut table = HashMap::new();
        table.insert('π', f64::consts::PI);
        table.insert('e', f64::consts::E);
        Self { table }
    }

    pub fn visit(&self, node: &Node, ext: Option<&HashMap<char, f64>>) -> InterpreterResult<Node> {
        Ok(match node {
            Node::Num(_) => node.clone(),
            Node::Var(var) => self
                .table
                .get(var)
                .or(ext.map(|t| t.get(var).unwrap()))
                .map_or_else(|| node.clone(), |val| Node::Num(*val)),
            Node::Func { func, arg } => self.visit_func(func, arg, ext)?,
            Node::Exponent { base, exponent } => self.visit_exponent(base, exponent, ext)?,
            Node::Factors(factors) => self.visit_factors(factors, ext)?,
            Node::Terms(terms) => self.visit_terms(terms, ext)?,
            Node::Derivative { derivative, var } => {
                self.visit(&self.differentiate(derivative, *var, ext)?, ext)?
            }
            Node::Equation { lhs, rhs } => Node::Equation {
                lhs: Box::new(self.visit(lhs, ext)?),
                rhs: Box::new(self.visit(rhs, ext)?),
            },
        })
    }
}
