use crate::subset;

pub type EventTy = subset::ast::EventTy;
pub type Expr = subset::ast::Expr;
pub type Id = subset::ast::Id;

#[derive(Clone, Debug)]
pub enum Ty {
    Int,
    // maybe use nonzero64?
    Width(u64),
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

pub type Stmt = subset::ast::GenericStmt<Decl, Parallel>;
pub type Port = subset::ast::GenericPort<Decl>;
pub type Module = subset::ast::GenericModule<Decl, Parallel>;
