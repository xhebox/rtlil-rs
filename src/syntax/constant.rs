use super::*;

#[derive(Clone, Debug)]
pub enum Const {
    Empty,
    Sig(Signal),
    Str(String),
    Int(i64),
}

impl fmt::Display for Const {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Const::Empty => Ok(()),
            Const::Sig(n) => write!(f, "{}", n),
            Const::Str(n) => write!(f, "\"{}\"", n),
            Const::Int(n) => write!(f, "{}", n),
        }
    }
}
