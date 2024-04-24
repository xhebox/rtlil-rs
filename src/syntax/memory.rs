use super::*;
use getset::*;

#[derive(Debug, Clone)]
pub enum MemoryOption {
    Width(i64),
    Offset(i64),
    Size(i64),
}

#[derive(Debug, Clone, Default, Getters, MutGetters)]
#[get = "pub"]
#[get_mut = "pub"]
pub struct Memory {
    id: String,
    width: i64,
    offset: i64,
    size: i64,
    attrs: HashMap<String, Const>,
}

impl Memory {
    pub fn new(i: String, o: Vec<MemoryOption>) -> Self {
        let mut r = Self {
            id: i,
            ..Self::default()
        };
        for opt in o {
            match opt {
                MemoryOption::Width(m) => r.width = m,
                MemoryOption::Offset(m) => r.offset = m,
                MemoryOption::Size(m) => r.size = m,
            }
        }
        r
    }
}

impl Visit for Memory {
    fn visit<F: Visitor>(&mut self, f: &mut F) -> Result<()> {
        f.enter(Node::Memory(self))?;
        f.leave(Node::Memory(self))?;
        Ok(())
    }
}
