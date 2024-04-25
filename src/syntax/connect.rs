use super::*;
use getset::*;

#[derive(Debug, Clone, Getters, MutGetters)]
#[get = "pub"]
#[get_mut = "pub"]
pub struct Connect {
    sig1: SigSpec,
    sig2: SigSpec,
}

impl Connect {
    pub fn new(sig1: SigSpec, sig2: SigSpec) -> Self {
        Self { sig1, sig2 }
    }
}

impl Visit for Connect {
    fn visit<F: Visitor>(&mut self, f: &mut F) -> Result<()> {
        f.enter(Node::Connect(self))?;
        f.leave(Node::Connect(self))?;
        Ok(())
    }
}
