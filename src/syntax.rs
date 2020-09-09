// Copyright (c) 2020 xhe

use anyhow::Result;
use std::collections::HashMap;
use std::fmt;
use std::sync::atomic::AtomicUsize;

pub static AUTOIDX: AtomicUsize = AtomicUsize::new(0);

mod design;
pub use design::*;

mod module;
pub use module::*;

mod wire;
pub use wire::*;

mod memory;
pub use memory::*;

mod cell;
pub use cell::*;

mod process;
pub use process::*;

mod connect;
pub use connect::*;

mod sigspec;
pub use sigspec::*;

mod constant;
pub use constant::*;

mod signal;
pub use signal::*;

macro_rules! define_type {
    ( $($x: ident),* ) => {
        #[derive(Debug)]
        pub enum Node<'a> {
            $($x(&'a mut $x),)*
        }
    };
}

define_type!(Design, Module, ModuleStmt, Wire, Memory, Cell, Process, ProcessSync, ProcessSwitch, ProcessSwitchCase, Connect, Signal);

pub trait Visitor {
	fn enter(&mut self, n: Node) -> Result<()>;
	fn leave(&mut self, n: Node) -> Result<()>;
}

pub trait Visit {
	fn visit<F: Visitor>(&mut self, f: &mut F) -> Result<()>;
}
