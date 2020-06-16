use crate::common::{self, GenericModule, GenericStmt};
use crate::pretty::PrettyPrinter;
use pretty::RcDoc;
use std::fmt;
use std::rc::Rc;

pub use common::Unop;
pub use common::Expr;

#[derive(Clone, Debug)]
pub enum Decl {
    Logic,
}

impl PrettyPrinter for Decl {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            Decl::Logic => RcDoc::text("logic"),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Par {
    Assign,
    AlwaysComb,
    AlwaysFF,
}

impl PrettyPrinter for Par {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            Par::Assign => RcDoc::text("assign"),
            Par::AlwaysComb => RcDoc::text("always_comb"),
            Par::AlwaysFF => RcDoc::text("always_ff"),
        }
    }
}

pub type Stmt = GenericStmt<Decl, Par>;

impl PrettyPrinter for Stmt {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            Stmt::Decl(decl) => decl.to_doc(),
            Stmt::Par(par) => par.to_doc(),
        }
    }
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_pretty())
    }
}

pub type Module = GenericModule<Decl, Par>;

impl PrettyPrinter for Module {
    fn to_doc(&self) -> RcDoc<()> {
        RcDoc::text("WIP")
    }
}

impl fmt::Display for Module {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_pretty())
    }
}
