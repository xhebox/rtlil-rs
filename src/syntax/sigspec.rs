use super::*;

#[derive(Debug, Clone)]
pub enum SigSpec {
    Const((Const, i64, i64)),
    Refer((String, i64, i64)),
    List(Vec<SigSpec>),
}

impl fmt::Display for SigSpec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SigSpec::Const((n, l, r)) => {
                write!(f, "{}", n)?;
                if *r != 0 {
                    write!(f, "[{}:{}]", l, r)?;
                } else if *l != 0 {
                    write!(f, "[{}]", l)?;
                }
            }
            SigSpec::Refer((n, l, r)) => {
                write!(f, "{}", n)?;
                if *r != 0 {
                    write!(f, "[{}:{}]", l, r)?;
                } else if *l != 0 {
                    write!(f, "[{}]", l)?;
                }
            }
            SigSpec::List(n) => {
                write!(f, "{{")?;
                for m in n.iter() {
                    write!(f, "{}", m)?;
                }
                write!(f, "}}")?;
            }
        };
        Ok(())
    }
}
