use std::{
    iter::Peekable,
    ops::{Add, Div, Mul, Sub},
    str::FromStr,
};

trait Numeric:
    Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Copy
    + FromStr
    + Default
{
}
impl Numeric for isize {}
impl Numeric for i64 {}
impl Numeric for i32 {}
impl Numeric for f32 {}
impl Numeric for f64 {}

#[derive(Debug, Clone)]
enum Op<T>
where
    T: Numeric,
{
    Seg(Vec<Op<T>>),
    Val(T),
    Add,
    Sub,
    Mul,
    Div,
}

fn main() {
    let mut math = "(12 + 2) - 3".chars().peekable();
    dbg!(parse_segment::<f32, _>(&mut math, &mut 0));
}

fn parse_segment<T, I: Iterator<Item = char>>(
    s: &mut Peekable<I>,
    bracket_count: &mut usize,
) -> Vec<Op<T>>
where
    T: Numeric,
{
    let mut res = vec![];
    while let Some(c) = s.next() {
        if c.is_whitespace() {
            continue;
        }
        match c {
            '(' => {
                *bracket_count += 1;
                res.push(Op::Seg(parse_segment(s, bracket_count)))
            }
            ')' => {
                assert!(*bracket_count > 0, "cannot close the bracket");
                *bracket_count -= 1;
                return res;
            }
            '+' => res.push(Op::Add),
            '-' => res.push(Op::Sub),
            '*' => res.push(Op::Mul),
            '/' => res.push(Op::Div),
            a => {
                let mut x = String::new();
                x.push(a);
                while let Some(c) = s.next_if(|x| x.is_ascii_digit() || *x == '.') {
                    x.push(c);
                }
                let Ok(parsed) = x.parse() else {
                    panic!("failed to parse {x}");
                };
                res.push(Op::Val(parsed))
            }
        }
    }
    assert!(*bracket_count == 0, "not enough closing brackets");
    res
}
