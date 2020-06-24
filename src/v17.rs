use crate::common::{self, GenericModule, GenericPort, GenericStmt};
use crate::pretty::{PrettyPrinter, PRETTY_INDENT};
use pretty::RcDoc;
use std::fmt;

pub use common::EventTy;
pub use common::Expr;
pub use common::Id;
pub use common::Sequential;
pub use common::Ty;

#[derive(Clone, Debug)]
pub enum Decl {
    Int(Id, Ty),
    Logic(Id, Ty),
}

impl PrettyPrinter for Decl {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            Decl::Int(name, ty) => ty
                .to_doc()
                .append(RcDoc::space())
                .append(RcDoc::as_string(name)),
            Decl::Logic(name, ty) => {
                let extra_space = match ty.width() {
                    1 => RcDoc::nil(),
                    _ => RcDoc::space(),
                };
                RcDoc::text("logic")
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
    AlwaysComb,
    AlwaysFF,
}

impl PrettyPrinter for Parallel {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            Parallel::Assign => RcDoc::text("assign"),
            Parallel::AlwaysComb => RcDoc::text("always_comb"),
            Parallel::AlwaysFF => RcDoc::text("always_ff"),
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

impl Module {
    pub fn new_with_name(name: &str) -> Module {
        Module {
            name: name.to_string(),
            ports: Vec::new(),
            body: Vec::new(),
        }
    }
}

impl PrettyPrinter for Module {
    fn to_doc(&self) -> RcDoc<()> {
        let mut body_doc = RcDoc::nil();
        for decl in self.body.iter() {
            body_doc = body_doc
                .append(RcDoc::hardline())
                .append(decl.to_doc())
                .append(RcDoc::text(";"));
        }
        body_doc = body_doc.nest(PRETTY_INDENT).group();
        RcDoc::text("module")
            .append(RcDoc::space())
            .append(RcDoc::as_string(&self.name))
            .append(RcDoc::space())
            .append(RcDoc::text("("))
            .append(RcDoc::text(")"))
            .append(RcDoc::text(";"))
            .append(body_doc)
            .append(RcDoc::hardline())
            .append(RcDoc::text("endmodule"))
    }
}

impl fmt::Display for Module {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_pretty())
    }
}
