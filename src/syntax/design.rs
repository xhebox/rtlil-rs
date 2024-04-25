use super::*;
use getset::*;

#[derive(Debug, Clone, Default, Getters, MutGetters)]
#[get = "pub"]
#[get_mut = "pub"]
pub struct Design {
    autoidx: usize,
    modules: Vec<Module>,
    attrs: HashMap<String, Const>,
}

impl Design {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Visit for Design {
    fn visit<F: Visitor>(&mut self, f: &mut F) -> Result<()> {
        f.enter(Node::Design(self))?;
        for n in self.modules.iter_mut() {
            n.visit(f)?;
        }
        f.leave(Node::Design(self))?;
        Ok(())
    }
}
