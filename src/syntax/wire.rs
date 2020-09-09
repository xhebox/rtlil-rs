use super::*;
use getset::*;

#[derive(Debug)]
pub enum WireOption {
    Width(i64),
    Upto,
    Signed,
    Offset(i64),
    Input(i64),
    Output(i64),
    Inout(i64),
}

#[derive(Debug, Default, Getters, MutGetters)]
#[get = "pub"]
#[get_mut = "pub"]
pub struct Wire {
    id: String,
    width: i64,
    offset: i64,
    port: i64,
    input: bool,
    output: bool,
    upto: bool,
    signed: bool,
    attrs: HashMap<String, Const>,
}

impl Wire {
    pub fn new(i: String, o: Vec<WireOption>) -> Self {
        let mut r = Self {
            id: i,
            ..Self::default()
        };
        for opt in o {
            match opt {
                WireOption::Width(m) => r.width = m,
                WireOption::Upto => r.upto = true,
                WireOption::Signed => r.signed = true,
                WireOption::Offset(m) => r.offset = m,
                // TODO: handle duplicate inout check?
                WireOption::Input(m) => {
                    r.port = m;
                    r.input = true;
                    r.output = false;
                }
                WireOption::Output(m) => {
                    r.port = m;
                    r.input = false;
                    r.output = true;
                }
                WireOption::Inout(m) => {
                    r.port = m;
                    r.input = true;
                    r.output = true;
                }
            }
        }
        r
    }
}

impl Visit for Wire {
    fn visit<F: Visitor>(&mut self, f: &mut F) -> Result<()> {
        f.enter(Node::Wire(self))?;
        f.leave(Node::Wire(self))?;
        Ok(())
    }
}
