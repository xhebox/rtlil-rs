use super::*;
use getset::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum State {
    S0,
    S1,
    Sx,
    Sz,
    Sa,
    Sm,
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            State::S0 => write!(f, "0"),
            State::S1 => write!(f, "1"),
            State::Sx => write!(f, "x"),
            State::Sz => write!(f, "z"),
            State::Sm => write!(f, "m"),
            _ => write!(f, "-"),
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Getters, MutGetters)]
#[get = "pub"]
#[get_mut = "pub"]
pub struct Signal {
    width: i64,
    bits: Vec<State>,
}

impl Signal {
    pub fn new(width: i64, bits: Vec<State>) -> Self {
        Signal {
            width,
            bits,
            ..Self::default()
        }
    }
}

impl fmt::Display for Signal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}'", self.width)?;
        for s in self.bits.iter() {
            write!(f, "{}", s)?;
        }
        Ok(())
    }
}
