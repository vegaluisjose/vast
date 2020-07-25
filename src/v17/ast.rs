use crate::subset;
use std::rc::Rc;

pub type EventTy = subset::ast::EventTy;
pub type Expr = subset::ast::Expr;
pub type Id = subset::ast::Id;

#[derive(Clone, Debug)]
pub enum Ty {
    Void,
    Int,
    // maybe use nonzero64?
    Width(u64),
}

#[derive(Clone, Debug)]
pub enum Decl {
    Int(Id, Ty),
    Logic(Id, Ty),
    Function(Id, Ty, Vec<Port>, Vec<Decl>, Vec<Sequential>),
    Param(Id, Ty, Expr),
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

pub type Stmt = subset::ast::GenericStmt<Decl, Parallel>;
pub type Port = subset::ast::GenericPort<Decl>;
pub type Module = subset::ast::GenericModule<Decl, Parallel>;
