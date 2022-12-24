use crate::MathParseError;
use crate::Numeric;
use std::{iter::Peekable, str::FromStr};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token<T: Numeric> {
    Seg(Vec<Token<T>>),
    Val(T),
    Add,
    Sub,
    Mul,
    Div,
}

impl<T: Numeric> Token<T> {
    fn fix_segment(self, splits: &[Token<T>]) -> Self {
        let Token::Seg(s) = self else {
        return self
    };
        let mut s = s.into_iter();
        let mut result = Vec::new();
        while let Some(op) = s.next() {
            match op {
                Token::Seg(s) => result.push(Token::Seg(s).fix_segment(splits)),
                op => {
                    if splits.contains(&op) {
                        let c = Token::Seg(vec![result.pop().unwrap(), op, s.next().unwrap()]);
                        result.push(c);
                    } else {
                        result.push(op)
                    }
                }
            }
        }
        if result.len() == 1 {
            return result[0].clone();
        }
        Token::Seg(result)
    }
    fn parse_number<I: Iterator<Item = char>>(s: &mut Peekable<I>) -> Result<T, MathParseError> {
        let mut x = String::new();
        while let Some(c) = s.next_if(|x| x.is_ascii_digit() || *x == '.') {
            x.push(c);
        }
        x.parse().map_err(|_| MathParseError::Number(x))
    }
    fn validate_inner(s: &Vec<Token<T>>) -> Result<(), MathParseError> {
        let mut prev_is_value = false;
        for tok in s {
            match tok {
                Token::Val(_) => {
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
    fn parse_inner<I: Iterator<Item = char>>(
        s: &mut Peekable<I>,
        bracket_count: &mut usize,
    ) -> Result<Token<T>, MathParseError> {
        let mut res = vec![];
        while let Some(c) = s.peek() {
            match c {
                ' ' => {
                    s.next();
                }
                '(' => {
                    *bracket_count += 1;
                    s.next();
                    res.push(Self::parse_inner(s, bracket_count)?)
                }
                ')' => {
                    if *bracket_count == 0 {
                        return Err(MathParseError::Bracket);
                    }
                    *bracket_count -= 1;
                    s.next();
                    return Ok(Token::Seg(res));
                }
                '+' => {
                    s.next();
                    res.push(Token::Add);
                }
                '-' => {
                    s.next();
                    if s.peek().unwrap().is_ascii_digit() {
                        let num = Self::parse_number(s)?;
                        res.push(Token::Val(-num))
                    } else {
                        res.push(Token::Sub)
                    }
                }
                '*' => {
                    s.next();
                    res.push(Token::Mul);
                }
                '/' => {
                    s.next();
                    res.push(Token::Div);
                }
                _ => res.push(Token::Val(Self::parse_number(s)?)),
            }
        }
        Self::validate_inner(&res)?;

        if *bracket_count != 0 {
            return Err(MathParseError::Bracket);
        }
        Ok(Token::Seg(res)
            .fix_segment(&[Token::Mul, Token::Div])
            .fix_segment(&[Token::Sub, Token::Add]))
    }
}

impl<T: Numeric> FromStr for Token<T> {
    type Err = MathParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.chars().peekable();
        Self::parse_inner(&mut s, &mut 0)
    }
}
