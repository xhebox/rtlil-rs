use super::*;
use getset::*;

#[derive(Debug, Clone, Default, Getters, MutGetters)]
#[get = "pub"]
#[get_mut = "pub"]
pub struct ProcessSwitchCase {
    sigs: Vec<SigSpec>,
    assign: Vec<(SigSpec, SigSpec)>,
    switch: Vec<ProcessSwitch>,
    attrs: HashMap<String, Const>,
}

impl ProcessSwitchCase {
    pub fn new(sigs: Vec<SigSpec>, stmts: Vec<ProcessStmt>) -> Self {
        let mut r = Self {
            sigs,
            ..Self::default()
        };
        for stmt in stmts {
            match stmt {
                ProcessStmt::Assign(v) => {
                    r.assign.push(v);
                }
                ProcessStmt::Switch(v) => {
                    r.switch.push(v);
                }
                _ => (),
            }
        }
        r
    }
}

impl Visit for ProcessSwitchCase {
    fn visit<F: Visitor>(&mut self, f: &mut F) -> Result<()> {
        f.enter(Node::ProcessSwitchCase(self))?;
        for s in self.switch.iter_mut() {
            s.visit(f)?;
        }
        f.leave(Node::ProcessSwitchCase(self))?;
        Ok(())
    }
}

#[derive(Debug, Clone, Getters, MutGetters)]
#[get = "pub"]
#[get_mut = "pub"]
pub struct ProcessSwitch {
    sig: SigSpec,
    cases: Vec<ProcessSwitchCase>,
    attrs: HashMap<String, Const>,
}

impl ProcessSwitch {
    pub fn new(sig: SigSpec, cases: Vec<ProcessSwitchCase>) -> Self {
        Self {
            sig,
            cases,
            attrs: HashMap::new(),
        }
    }
}

impl Visit for ProcessSwitch {
    fn visit<F: Visitor>(&mut self, f: &mut F) -> Result<()> {
        f.enter(Node::ProcessSwitch(self))?;
        for c in self.cases.iter_mut() {
            c.visit(f)?;
        }
        f.leave(Node::ProcessSwitch(self))?;
        Ok(())
    }
}

pub enum ProcessStmt {
    Empty,
    Assign((SigSpec, SigSpec)),
    Switch(ProcessSwitch),
}

#[derive(Debug, Clone)]
pub enum ProcessSyncType {
    Always,
    Global,
    Init,
    Low(SigSpec),
    High(SigSpec),
    Posedge(SigSpec),
    Negedge(SigSpec),
    Edge(SigSpec),
}

#[derive(Debug, Clone, Getters, MutGetters)]
#[get = "pub"]
#[get_mut = "pub"]
pub struct ProcessSync {
    tp: ProcessSyncType,
    updates: Vec<(SigSpec, SigSpec)>,
    attrs: HashMap<String, Const>,
}

impl ProcessSync {
    pub fn new(tp: ProcessSyncType, updates: Vec<(SigSpec, SigSpec)>) -> Self {
        Self {
            tp,
            updates,
            attrs: HashMap::new(),
        }
    }
}

impl Visit for ProcessSync {
    fn visit<F: Visitor>(&mut self, f: &mut F) -> Result<()> {
        f.enter(Node::ProcessSync(self))?;
        f.leave(Node::ProcessSync(self))?;
        Ok(())
    }
}

#[derive(Debug, Clone, Default, Getters, MutGetters)]
#[get = "pub"]
#[get_mut = "pub"]
pub struct Process {
    id: String,
    assign: Vec<(SigSpec, SigSpec)>,
    switch: Vec<ProcessSwitch>,
    syncs: Vec<ProcessSync>,
    attrs: HashMap<String, Const>,
}

impl Process {
    pub fn new(id: String, stmts: Vec<ProcessStmt>, syncs: Vec<ProcessSync>) -> Self {
        let mut r = Self {
            id,
            syncs,
            ..Self::default()
        };
        for stmt in stmts {
            match stmt {
                ProcessStmt::Assign(v) => {
                    r.assign.push(v);
                }
                ProcessStmt::Switch(v) => {
                    r.switch.push(v);
                }
                _ => (),
            }
        }
        r
    }
}

impl Visit for Process {
    fn visit<F: Visitor>(&mut self, f: &mut F) -> Result<()> {
        f.enter(Node::Process(self))?;
        for s in self.switch.iter_mut() {
            s.visit(f)?;
        }
        for s in self.syncs.iter_mut() {
            s.visit(f)?;
        }
        f.leave(Node::Process(self))?;
        Ok(())
    }
}
