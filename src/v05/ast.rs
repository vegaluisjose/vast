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
    Param(Id, Expr),
}

#[derive(Clone, Debug)]
pub enum Sequential {
    Wildcard,
    Event(EventTy, Expr),
    If(Expr, Vec<Sequential>, Vec<Sequential>),
}

#[derive(Clone, Debug)]
pub struct Map {
    pub id: Id,
    pub value: Expr,
}

#[derive(Clone, Debug)]
pub struct Instance {
    pub id: Id,
    pub prim: Id,
    pub param_map: Vec<Map>,
    pub port_map: Vec<Map>,
}

#[derive(Clone, Debug)]
pub enum Parallel {
    Instance(Instance),
    Assign(Expr, Expr),
    Always,
}

pub type Stmt = subset::ast::GenericStmt<Decl, Parallel>;
pub type Port = subset::ast::GenericPort<Decl>;
pub type Module = subset::ast::GenericModule<Decl, Parallel>;
