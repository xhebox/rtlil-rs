extern crate bitflags;

use super::*;
use bitflags::*;
use getset::*;

bitflags! {
    #[derive(Default)]
    pub struct CellFlag : u32 {
        const REAL = 0x1;
        const SIGNED = 0x2;
    }
}

#[derive(Debug, Getters, MutGetters)]
#[get = "pub"]
#[get_mut = "pub"]
pub struct CellParam {
    val: Const,
    flags: CellFlag,
}

impl CellParam {
    pub fn new(val: Const, flags: CellFlag) -> Self {
        Self { val, flags }
    }
}

impl fmt::Display for CellParam {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.flags.contains(CellFlag::SIGNED) {
            write!(f, "signed ")?;
        }
        if self.flags.contains(CellFlag::REAL) {
            write!(f, "real ")?;
        }
        write!(f, "{}", self.val)?;
        Ok(())
    }
}

#[derive(Debug)]
pub enum CellOption {
    Param((CellFlag, String, Const)),
    Connect((String, SigSpec)),
}

#[derive(Debug, Default, Getters, MutGetters)]
#[get = "pub"]
#[get_mut = "pub"]
pub struct Cell {
    i1: String,
    i2: String,
    params: HashMap<String, CellParam>,
    connects: HashMap<String, SigSpec>,
    attrs: HashMap<String, Const>,
}

impl Cell {
    pub fn new(i1: String, i2: String, o: Vec<CellOption>) -> Self {
        let mut r = Self {
            i1,
            i2,
            ..Self::default()
        };
        for opt in o {
            match opt {
                CellOption::Param((f, k, v)) => {
                    r.params.insert(k, CellParam::new(v, f));
                }
                CellOption::Connect((k, v)) => {
                    r.connects.insert(k, v);
                }
            }
        }
        r
    }
}

impl Visit for Cell {
    fn visit<F: Visitor>(&mut self, f: &mut F) -> Result<()> {
        f.enter(Node::Cell(self))?;
        f.leave(Node::Cell(self))?;
        Ok(())
    }
}
