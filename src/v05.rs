use crate::common::{self, GenericModule, GenericStmt};
use crate::pretty::PrettyPrinter;
use pretty::RcDoc;
use std::fmt;
use std::rc::Rc;

pub use common::Id;
pub use common::Unop;
pub use common::Expr;
pub use common::Width;


#[derive(Clone, Debug)]
pub enum Decl {
    Wire(Id, Width),
    Reg(Id, Width),
}

impl PrettyPrinter for Decl {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            Decl::Wire(name, width) => RcDoc::text("wire")
                    .append(RcDoc::space())
                    .append(width.to_doc())
                    .append(RcDoc::space())
                    .append(RcDoc::as_string(name)),
            Decl::Reg(name, width) => RcDoc::text("reg"),
        }
    }
}

impl fmt::Display for Decl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_pretty())
    }
}

#[derive(Clone, Debug)]
pub enum Par {
    Assign,
    Always,
}

impl PrettyPrinter for Par {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            Par::Assign => RcDoc::text("assign"),
            Par::Always => RcDoc::text("always"),
        }
    }
}

impl fmt::Display for Par {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_pretty())
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
