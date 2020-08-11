use crate::subset;
use std::rc::Rc;

pub type Id = subset::ast::Id;
pub type IPath = subset::ast::IPath;
pub type Expr = subset::ast::Expr;
pub type EventTy = subset::ast::EventTy;
pub type AssignTy = subset::ast::AssignTy;
pub type Instance = subset::ast::Instance;
pub type CaseBranch = subset::ast::GenericCaseBranch<Sequential>;
pub type Function = subset::ast::GenericFunction<Decl, Sequential, Ty>;
pub type Stmt = subset::ast::GenericStmt<Decl, Parallel>;
pub type Port = subset::ast::GenericPort<Decl>;
pub type Module = subset::ast::GenericModule<Decl, Parallel>;

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
    Func(Function),
    Param(Id, Ty, Expr),
}

#[derive(Clone, Debug)]
pub enum Sequential {
    Error(String),
    Return(Expr),
    Assign(Expr, Expr, AssignTy),
    Event(EventTy, Expr),
    If(Expr, Vec<Sequential>, Vec<Sequential>),
    Assert(Expr, Option<Rc<Sequential>>),
    // Case(Expr, CaseBranch, Option<Sequential>),
}

#[derive(Clone, Debug)]
pub enum Parallel {
    Inst(Instance),
    Assign,
    AlwaysComb(Vec<Sequential>),
    AlwaysFF(Sequential, Vec<Sequential>),
}
