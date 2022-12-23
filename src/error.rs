#[derive(Debug, Clone)]
pub enum MathParseError {
    Number(String),
    Bracket,
    Expression,
}

impl std::fmt::Display for MathParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(token) => write!(f, "failed to parse token as number {token}"),
            Self::Bracket => write!(f, "mismatched number of brackets"),
            Self::Expression => write!(f, "two consequtive values or operands"),
        }
    }
}
impl std::error::Error for MathParseError {}
