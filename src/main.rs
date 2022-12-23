use rust_math_parser::Op;

fn main() {
    let math = "2 - 3 + 4 * 3 / 3";
    let math: Op<f32> = math.parse().unwrap();
    println!("{math} = {}", math.eval());
}
