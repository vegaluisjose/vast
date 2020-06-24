use crate::common::{self, GenericModule, GenericPort, GenericStmt};
use crate::pretty::PrettyPrinter;
use pretty::RcDoc;
use std::fmt;

pub use common::Expr;
pub use common::Id;
pub use common::Ty;

#[derive(Clone, Debug)]
pub enum Decl {
    Int(Id, Ty),
    Wire(Id, Ty),
    Reg(Id, Ty),
}

impl PrettyPrinter for Decl {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            Decl::Int(name, ty) => ty
                .to_doc()
                .append(RcDoc::space())
                .append(RcDoc::as_string(name)),
            Decl::Wire(name, ty) => {
                let extra_space = match ty.width() {
                    1 => RcDoc::nil(),
                    _ => RcDoc::space(),
                };
                RcDoc::text("wire")
                    .append(RcDoc::space())
                    .append(ty.to_doc())
                    .append(extra_space)
                    .append(RcDoc::as_string(name))
            }
            Decl::Reg(name, ty) => {
                let extra_space = match ty.width() {
                    1 => RcDoc::nil(),
                    _ => RcDoc::space(),
                };
                RcDoc::text("reg")
                    .append(RcDoc::space())
                    .append(ty.to_doc())
                    .append(extra_space)
                    .append(RcDoc::as_string(name))
            }
        }
    }
}

impl fmt::Display for Decl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_pretty())
    }
}

#[derive(Clone, Debug)]
pub enum Parallel {
    Assign,
    Always,
}

impl PrettyPrinter for Parallel {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            Parallel::Assign => RcDoc::text("assign"),
            Parallel::Always => RcDoc::text("always"),
        }
    }
}

impl fmt::Display for Parallel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_pretty())
    }
}

pub type Stmt = GenericStmt<Decl, Parallel>;

impl PrettyPrinter for Stmt {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            Stmt::Decl(decl) => decl.to_doc(),
            Stmt::Parallel(par) => par.to_doc(),
        }
    }
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_pretty())
    }
}

pub type Port = GenericPort<Decl>;

impl PrettyPrinter for Port {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            Port::Input(decl) => RcDoc::text("input")
                .append(RcDoc::space())
                .append(decl.to_doc()),
            Port::Output(decl) => RcDoc::text("output")
                .append(RcDoc::space())
                .append(decl.to_doc()),
        }
    }
}

impl fmt::Display for Port {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_pretty())
    }
}

pub type Module = GenericModule<Decl, Parallel>;

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
