use std::{
    collections::HashMap,
    fmt::Debug,
    ops::{Add, Div, Mul, Neg, Sub},
    str::FromStr,
};

mod error;
mod token;

use error::MathEvalError;
pub use error::MathParseError;
use token::Token;

pub trait Numeric:
    Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Neg<Output = Self>
    + Copy
    + FromStr
    + Default
    + PartialEq
    + Debug
{
}
impl Numeric for isize {}
impl Numeric for i64 {}
impl Numeric for i32 {}
impl Numeric for f32 {}
impl Numeric for f64 {}

#[derive(Debug, Clone)]
pub enum Op<T: Numeric> {
    Val(T),
    Var(String),
    Fun(String, Box<Op<T>>),
    Neg(Box<Op<T>>),
    Add(Box<Op<T>>, Box<Op<T>>),
    Sub(Box<Op<T>>, Box<Op<T>>),
    Mul(Box<Op<T>>, Box<Op<T>>),
    Div(Box<Op<T>>, Box<Op<T>>),
}

pub type Function<T> = fn(T) -> T;
pub type Functions<'a, T> = HashMap<&'a str, Function<T>>;
pub type Variables<'a, T> = HashMap<&'a str, T>;

impl<T: Numeric> Op<T> {
    pub fn eval(
        &self,
        vars: Option<&Variables<T>>,
        funs: Option<&Functions<T>>,
    ) -> Result<T, MathEvalError> {
        let res = match self {
            Self::Val(a) => *a,
            Self::Add(a, b) => a.eval(vars, funs)? + b.eval(vars, funs)?,
            Self::Sub(a, b) => a.eval(vars, funs)? - b.eval(vars, funs)?,
            Self::Mul(a, b) => a.eval(vars, funs)? * b.eval(vars, funs)?,
            Self::Div(a, b) => a.eval(vars, funs)? / b.eval(vars, funs)?,
            Self::Neg(a) => -(a.eval(vars, funs)?),
            Self::Var(name) => {
                let c = vars
                    .and_then(|vars| vars.get(name.as_str()))
                    .ok_or(MathEvalError::Variable(name.clone()))?;
                *c
            }
            Self::Fun(name, a) => {
                let a = a.eval(vars, funs)?;
                let c = funs
                    .and_then(|vars| vars.get(name.as_str()))
                    .ok_or(MathEvalError::Function(name.clone()))?;
                (c)(a)
            }
        };
        Ok(res)
    }
    fn from_tokens(value: &Token<T>) -> Self {
        match value {
            &Token::Val(x) => Self::Val(x),
            Token::Var(x) => Self::Var(x.clone()),
            Token::Neg(x) => Self::Neg(Box::new(Self::from_tokens(x.as_ref()))),
            Token::Fun(name, s) => Self::Fun(name.clone(), Box::new(Self::from_tokens(s.as_ref()))),
            Token::Seg(s) => match s.get(1).unwrap() {
                Token::Add => Self::Add(
                    Box::new(Self::from_tokens(s.first().unwrap())),
                    Box::new(Self::from_tokens(s.last().unwrap())),
                ),
                Token::Sub => Self::Sub(
                    Box::new(Self::from_tokens(s.first().unwrap())),
                    Box::new(Self::from_tokens(s.last().unwrap())),
                ),
                Token::Mul => Self::Mul(
                    Box::new(Self::from_tokens(s.first().unwrap())),
                    Box::new(Self::from_tokens(s.last().unwrap())),
                ),
                Token::Div => Self::Div(
                    Box::new(Self::from_tokens(s.first().unwrap())),
                    Box::new(Self::from_tokens(s.last().unwrap())),
                ),

                _ => panic!("unreachable"),
            },
            _ => panic!("unreachable"),
        }
    }
}

impl<T: Numeric + std::fmt::Display> std::fmt::Display for Op<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Val(a) => write!(f, "{a}"),
            Self::Var(a) => write!(f, "{a}"),
            Self::Neg(a) => write!(f, "-({a})"),
            Self::Fun(name, a) => write!(f, "{name}({a})"),
            Self::Add(a, b) => write!(f, "({a} + {b})"),
            Self::Sub(a, b) => write!(f, "({a} - {b})"),
            Self::Mul(a, b) => write!(f, "({a} * {b})"),
            Self::Div(a, b) => write!(f, "({a} / {b})"),
        }
    }
}

impl<T: Numeric> FromStr for Op<T> {
    type Err = MathParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens = s.parse()?;
        Ok(Self::from_tokens(&tokens))
    }
}
