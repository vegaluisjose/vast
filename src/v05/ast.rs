use crate::subset;

pub type Id = subset::ast::Id;
pub type Expr = subset::ast::Expr;
pub type ExprConcat = subset::ast::ExprConcat;
pub type EventTy = subset::ast::EventTy;
pub type Attribute = subset::ast::Attribute;
pub type AssignTy = subset::ast::AssignTy;
pub type Instance = subset::ast::Instance;
pub type Stmt = subset::ast::GenericStmt<Decl, Parallel>;
pub type Port = subset::ast::GenericPort<Decl>;
pub type Module = subset::ast::GenericModule<Decl, Parallel>;

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
pub struct SequentialIfElse {
    pub cond: Expr,
    pub tru: Vec<Sequential>,
    pub fal: Vec<Sequential>,
}

#[derive(Clone, Debug)]
pub enum Sequential {
    Wildcard,
    Event(EventTy, Expr),
    Assign(Expr, Expr, AssignTy),
    IfElse(SequentialIfElse),
}

#[derive(Clone, Debug)]
pub struct ParallelAlways {
    pub event: Sequential,
    pub body: Vec<Sequential>,
}

#[derive(Clone, Debug)]
pub enum Parallel {
    Inst(Instance),
    Assign(Expr, Expr),
    Always(ParallelAlways),
}
