use crate::subset;
use crate::subset::ast::{GenericModule, GenericPort, GenericStmt};
use std::rc::Rc;

pub use subset::ast::EventTy;
pub use subset::ast::Expr;
pub use subset::ast::Id;

#[derive(Clone, Debug)]
pub enum Ty {
    Void,
    Int,
    Width(u64),
}

#[derive(Clone, Debug)]
pub enum Decl {
    Int(Id, Ty),
    Logic(Id, Ty),
    Function(Id, Ty, Vec<Port>, Vec<Decl>, Vec<Sequential>),
}

#[derive(Clone, Debug)]
pub enum Sequential {
    Event(EventTy, Expr),
    If(Expr, Vec<Sequential>, Vec<Sequential>),
    Assert(Expr, Option<Rc<Sequential>>),
}

#[derive(Clone, Debug)]
pub enum Parallel {
    Assign,
    AlwaysComb(Vec<Sequential>),
    AlwaysFF(Sequential, Vec<Sequential>),
}

pub type Stmt = GenericStmt<Decl, Parallel>;
pub type Port = GenericPort<Decl>;
pub type Module = GenericModule<Decl, Parallel>;
