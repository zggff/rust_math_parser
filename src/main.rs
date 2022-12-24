use std::collections::HashMap;

use rust_math_parser::{Functions, Op, Variables};

fn main() {
    let math = "-abs(12 - 13 * 4) + 4";
    let mut variables: Variables<f32> = HashMap::new();
    variables.insert("A", 32.0);

    let mut functions: Functions<f32> = HashMap::new();
    functions.insert("abs", |x| x.abs());

    let math: Op<f32> = math.parse().unwrap();
    dbg!(&math);
    println!(
        "{math} = {:?}",
        math.eval(Some(&variables), Some(&functions))
    )
}
