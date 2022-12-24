use std::{
    ops::{Add, Div, Mul, Neg, Sub},
    str::FromStr,
};

mod error;
mod token;

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
    Add(Box<Op<T>>, Box<Op<T>>),
    Sub(Box<Op<T>>, Box<Op<T>>),
    Mul(Box<Op<T>>, Box<Op<T>>),
    Div(Box<Op<T>>, Box<Op<T>>),
}

impl<T: Numeric> Op<T> {
    pub fn eval(&self) -> T {
        match self {
            Self::Val(a) => *a,
            Self::Add(a, b) => a.eval() + b.eval(),
            Self::Sub(a, b) => a.eval() - b.eval(),
            Self::Mul(a, b) => a.eval() * b.eval(),
            Self::Div(a, b) => a.eval() / b.eval(),
        }
    }
    fn from_tokens(value: &Token<T>) -> Self {
        match value {
            &Token::Val(x) => Self::Val(x),
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
