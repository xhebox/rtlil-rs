extern crate env_logger;
extern crate log;

use super::syntax::*;
use anyhow::Result;
use env_logger::*;
use log::*;
use std::io::Write;
use std::sync::{Arc, Mutex};

pub struct Dumper {
    p: Arc<Mutex<usize>>,
}

impl Dumper {
    pub fn new() -> Self {
        let r = Self {
            p: Arc::new(Mutex::new(0)),
        };
        let p = Arc::clone(&r.p);
        Builder::from_default_env()
            .format(move |buf, record| {
                writeln!(buf, "{0:1$}{2}", "", p.lock().unwrap(), record.args())
            })
            .filter(None, LevelFilter::Info)
            .init();
        r
    }

    fn indent(&mut self, i: i32) {
        if i < 0 {
            *self.p.lock().unwrap() -= -i as usize;
        } else {
            *self.p.lock().unwrap() += i as usize;
        }
    }
}

impl Visitor for Dumper {
    fn enter(&mut self, m: Node) -> Result<()> {
        match m {
            Node::Design(n) => {
                info!("autoidx {}", n.autoidx());
            }
            Node::Module(n) => {
                for (k, v) in n.attrs() {
                    info!("attribute {} {}", k, v);
                }
                info!("module {}", n.ident());
                self.indent(2);
                for (k, v) in n.params() {
                    info!("parameter {} {}", k, v);
                }
            }
            Node::Wire(n) => {
                for (k, v) in n.attrs() {
                    info!("attribute {} {}", k, v);
                }
                let mut s = "wire".to_string();
                if *n.width() != 0 {
                    s = format!("{} width {}", s, n.width());
                }
                if *n.upto() {
                    s = format!("{} upto", s);
                }
                if *n.signed() {
                    s = format!("{} signed", s);
                }
                if *n.offset() != 0 {
                    s = format!("{} offset {}", s, n.offset());
                }
                match (n.input(), n.output()) {
                    (true, false) => s = format!("{} input {}", s, n.port()),
                    (false, true) => s = format!("{} output {}", s, n.port()),
                    (true, true) => s = format!("{} inout {}", s, n.port()),
                    // TODO: handle this
                    _ => (),
                };
                if *n.offset() != 0 {}
                info!("{} {}", s, n.id());
            }
            Node::Cell(n) => {
                for (k, v) in n.attrs() {
                    info!("attribute {} {}", k, v);
                }
                info!("cell {} {}", n.i1(), n.i2());
                self.indent(2);
                for (k, v) in n.params() {
                    info!("parameter {} {}", k, v);
                }
                for (k, v) in n.connects() {
                    info!("connect {} {}", k, v);
                }
                self.indent(-2);
                info!("end");
            }
            Node::Memory(n) => {
                for (k, v) in n.attrs() {
                    info!("attribute {} {}", k, v);
                }
                let mut s = "memory".to_string();
                if *n.width() != 0 {
                    s = format!("{} width {}", s, n.width())
                }
                if *n.offset() != 0 {
                    s = format!("{} offset {}", s, n.offset())
                }
                if *n.size() != 0 {
                    s = format!("{} size {}", s, n.size())
                }
                info!("{} {}", s, n.id());
            }
            Node::Process(n) => {
                for (k, v) in n.attrs() {
                    info!("attribute {} {}", k, v);
                }
                info!("process {}", n.id());
                self.indent(2);
                for (k, v) in n.assign() {
                    info!("assign {} {}", k, v);
                }
            }
            Node::ProcessSwitch(n) => {
                for (k, v) in n.attrs() {
                    info!("attribute {} {}", k, v);
                }
                info!("switch {}", n.sig());
                self.indent(2);
            }
            Node::ProcessSwitchCase(n) => {
                for (k, v) in n.attrs() {
                    info!("attribute {} {}", k, v);
                }
                let mut s = "case".to_string();
                for (i, sig) in n.sigs().iter().enumerate() {
                    if i == 0 {
                        s = format!("{} {}", s, sig)
                    } else {
                        s = format!("{}, {}", s, sig)
                    }
                }
                info!("{}", s);
                self.indent(2);
                for (k, v) in n.assign() {
                    info!("assign {} {}", k, v);
                }
            }
            Node::ProcessSync(n) => {
                for (k, v) in n.attrs() {
                    info!("attribute {} {}", k, v);
                }
                match n.tp() {
                    ProcessSyncType::Always => info!("sync always"),
                    ProcessSyncType::Global => info!("sync global"),
                    ProcessSyncType::Init => info!("sync init"),
                    ProcessSyncType::Low(n) => info!("sync low {}", n),
                    ProcessSyncType::High(n) => info!("sync high {}", n),
                    ProcessSyncType::Posedge(n) => info!("sync posedge {}", n),
                    ProcessSyncType::Negedge(n) => info!("sync negedge {}", n),
                    ProcessSyncType::Edge(n) => info!("sync edge {}", n),
                }
                self.indent(2);
                for (k, v) in n.updates() {
                    info!("update {} {}", k, v);
                }
                self.indent(-2);
            }
            _ => (),
        }
        Ok(())
    }

    fn leave(&mut self, m: Node) -> Result<()> {
        match m {
            Node::Module(_) => {
                self.indent(-2);
                info!("end");
            }
            Node::Process(_) => {
                self.indent(-2);
                info!("end");
            }
            Node::ProcessSwitch(_) => {
                self.indent(-2);
                info!("end");
            }
            Node::ProcessSwitchCase(_) => {
                self.indent(-2);
            }
            _ => (),
        }
        Ok(())
    }
}
