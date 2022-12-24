use crate::MathParseError;
use crate::Numeric;
use std::{iter::Peekable, str::FromStr};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token<T: Numeric> {
    Seg(Vec<Token<T>>),
    Val(T),
    Neg(Box<Token<T>>),
    Var(String),
    Fun(String, Box<Token<T>>),
    Add,
    Sub,
    Mul,
    Div,
}

fn tokenise<I: Iterator<Item = char>>(s: &mut Peekable<I>) -> Vec<String> {
    let mut res = vec![];
    let mut current = String::new();
    while let Some(c) = s.peek() {
        match c {
            ' ' => {
                s.next();
            }
            '+' | '-' | '/' | '*' | '(' | ')' => {
                if !current.is_empty() {
                    res.push(current.clone());
                    current.clear();
                }
                res.push(format!("{}", s.next().unwrap()));
            }
            a => {
                current.push(*a);
                s.next();
            }
        }
    }
    if !current.is_empty() {
        res.push(current);
    }
    res
}

impl<T: Numeric> Token<T> {
    fn fix_segment(self, splits: &[Token<T>]) -> Self {
        match self {
            Token::Seg(s) => Self::Seg(Self::fix_segments(s, splits)),
            Token::Fun(name, s) => Self::Fun(name, Box::new(Self::fix_segment(*s, splits))),
            Token::Neg(s) => Self::Neg(Box::new(Self::fix_segment(*s, splits))),
            Token::Var(name) => Self::Var(name),
            a => a,
        }
    }
    fn fix_segments(s: Vec<Token<T>>, splits: &[Token<T>]) -> Vec<Token<T>> {
        let mut s = s.into_iter();
        let mut result = Vec::new();
        while let Some(op) = s.next() {
            match op {
                Token::Seg(s) => result.push(Self::Seg(Self::fix_segments(s, splits))),
                Token::Fun(name, s) => {
                    result.push(Self::Fun(name, Box::new(Self::fix_segment(*s, splits))))
                }
                Token::Neg(s) => result.push(Self::Neg(Box::new(Self::fix_segment(*s, splits)))),
                Token::Var(name) => result.push(Self::Var(name)),
                op => {
                    if splits.contains(&op) {
                        let c = Token::Seg(vec![
                            result.pop().unwrap(),
                            op,
                            s.next().unwrap().fix_segment(splits),
                        ]);
                        result.push(c);
                    } else {
                        result.push(op)
                    }
                }
            }
        }
        result
    }

    fn validate_inner(s: &Vec<Token<T>>) -> Result<(), MathParseError> {
        let mut prev_is_value = false;
        for tok in s {
            match tok {
                Token::Val(_) | Token::Var(_) | Token::Neg(_) => {
                    if prev_is_value {
                        return Err(MathParseError::Expression);
                    } else {
                        prev_is_value = true;
                    }
                }

                Token::Seg(s) => {
                    Self::validate_inner(s)?;
                    if prev_is_value {
                        return Err(MathParseError::Expression);
                    } else {
                        prev_is_value = true;
                    }
                }
                Token::Fun(_, s) => {
                    Self::validate_inner(&vec![s.as_ref().clone()])?;
                    if prev_is_value {
                        return Err(MathParseError::Expression);
                    } else {
                        prev_is_value = true;
                    }
                }
                _ => {
                    if !prev_is_value {
                        return Err(MathParseError::Expression);
                    } else {
                        prev_is_value = false;
                    }
                }
            }
        }
        if prev_is_value {
            Ok(())
        } else {
            Err(MathParseError::Expression)
        }
    }
    fn parse_segment<I: Iterator<Item = String>>(
        c: String,
        s: &mut Peekable<I>,
        bracket_count: &mut usize,
        is_start: &mut bool,
    ) -> Result<Token<T>, MathParseError> {
        let res = match c.as_str() {
            "(" => {
                *bracket_count += 1;

                Token::Seg(Self::parse_inner(s, bracket_count)?)
            }
            ")" => {
                panic!("unreachable")
            }
            "+" => Token::Add,
            "-" => {
                if *is_start {
                    *is_start = false;
                    let a = Self::parse_segment(s.next().unwrap(), s, bracket_count, is_start)?;

                    Token::Neg(Box::new(a))
                } else {
                    Token::Sub
                }
            }
            "*" => Token::Mul,
            "/" => Token::Div,
            a => {
                let c = a.parse();

                if let Ok(c) = c {
                    Token::Val(c)
                } else if s.peek() == Some(&"(".to_string()) {
                    *bracket_count += 1;
                    s.next();
                    Token::Fun(
                        a.to_string(),
                        Box::new(Self::Seg(Self::parse_inner(s, bracket_count)?)),
                    )
                } else {
                    Token::Var(a.to_string())
                }
            }
        };
        Ok(res)
    }
    fn parse_inner<I: Iterator<Item = String>>(
        s: &mut Peekable<I>,
        bracket_count: &mut usize,
    ) -> Result<Vec<Token<T>>, MathParseError> {
        let mut res = vec![];
        let mut is_start = true;
        while let Some(c) = s.next() {
            if c == ")" {
                *bracket_count -= 1;
                break;
            }
            res.push(Self::parse_segment(c, s, bracket_count, &mut is_start)?);
            is_start = false;
        }
        if *bracket_count != 0 {
            return Err(MathParseError::Bracket);
        }
        Ok(res)
    }
    fn cleanup(self) -> Self {
        match self {
            Self::Seg(s) => match s.len() {
                1 => s[0].clone().cleanup(),
                3 => {
                    let s: Vec<Self> = s.into_iter().map(Self::cleanup).collect();
                    Self::Seg(s)
                }
                a => {
                    dbg!(a);
                    todo!("cleanup not implemented")
                }
            },
            Self::Neg(s) => Self::Neg(Box::new(s.cleanup())),
            Self::Fun(name, s) => Self::Fun(name, Box::new(s.cleanup())),
            a => a,
        }
    }
    fn parse(s: &str) -> Result<Token<T>, MathParseError> {
        let mut s = s.chars().peekable();
        let s = tokenise(&mut s);

        let mut s = s.into_iter().peekable();
        let s = Self::parse_inner(&mut s, &mut 0)?;

        Self::validate_inner(&s)?;
        let s = Self::fix_segments(s, &[Token::Mul, Token::Div]);
        let s = Self::fix_segments(s, &[Token::Add, Token::Sub]);
        let s = Token::Seg(s).cleanup();
        Ok(s)
    }
}

impl<T: Numeric> FromStr for Token<T> {
    type Err = MathParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}
