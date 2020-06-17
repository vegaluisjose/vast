use crate::common::{self, GenericModule, GenericStmt, GenericPort};
use crate::pretty::PrettyPrinter;
use pretty::RcDoc;
use std::fmt;

pub use common::Id;
pub use common::Width;
pub use common::Expr;

#[derive(Clone, Debug)]
pub enum Decl {
    Logic(Id, Width),
}

impl PrettyPrinter for Decl {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            Decl::Logic(name, width) => {
                let ty = match width {
                    1 => RcDoc::nil(),
                    _ => width.to_doc().append(RcDoc::space()),
                };
                RcDoc::text("logic")
                    .append(RcDoc::space())
                    .append(ty)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decl_logic_width_32() {
        assert_eq!("logic [31:0] foo".to_string(), Decl::Logic("foo".to_string(), 32).to_string());
    }

    #[test]
    fn test_decl_logic_width_1() {
        assert_eq!("logic foo".to_string(), Decl::Logic("foo".to_string(), 1).to_string());
    }

    #[test]
    fn test_port_input_width_1() {
        assert_eq!("input logic foo".to_string(), Port::Input(Decl::Logic("foo".to_string(), 1)).to_string());
    }
}
