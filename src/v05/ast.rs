use crate::subset;
use crate::subset::ast::{GenericModule, GenericPort, GenericStmt};

pub use subset::ast::EventTy;
pub use subset::ast::Expr;
pub use subset::ast::Id;

#[derive(Clone, Debug)]
pub enum Ty {
    Int,
    Width(u64),
}

impl Ty {
    pub fn width(&self) -> u64 {
        match self {
            Ty::Width(w) => w.clone(),
            _ => panic!("Error: type does not support width"),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Decl {
    Int(Id, Ty),
    Wire(Id, Ty),
    Reg(Id, Ty),
}

#[derive(Clone, Debug)]
pub enum Sequential {
    Wildcard,
    Event(EventTy, Expr),
    If(Expr, Vec<Sequential>, Vec<Sequential>),
}

#[derive(Clone, Debug)]
pub enum Parallel {
    Assign,
    Always,
}

pub type Stmt = GenericStmt<Decl, Parallel>;
pub type Port = GenericPort<Decl>;
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
