use crate::subset;
use std::rc::Rc;

pub type Id = subset::ast::Id;
pub type InstancePath = subset::ast::InstancePath;
pub type Expr = subset::ast::Expr;
pub type EventTy = subset::ast::EventTy;
pub type Attribute = subset::ast::Attribute;
pub type AssignTy = subset::ast::AssignTy;
pub type Instance = subset::ast::Instance;
pub type CaseBranch = subset::ast::GenericCaseBranch<Sequential>;
pub type CaseDefault = subset::ast::GenericCaseDefault<Sequential>;
pub type Case = subset::ast::GenericCase<Sequential>;
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

#[derive(Clone, Debug, Default)]
pub struct SequentialIfElse {
    pub cond: Option<Expr>,
    pub body: Vec<Sequential>,
    pub else_branch: Option<Rc<Sequential>>,
}

#[derive(Clone, Debug)]
pub enum Sequential {
    Error(String),
    Display(String),
    Return(Expr),
    SeqAssign(Expr, Expr, AssignTy),
    SeqCase(Case),
    SeqCall(Expr),
    Event(EventTy, Expr),
    If(SequentialIfElse),
    Assert(Expr, Option<Rc<Sequential>>),
}

#[derive(Clone, Debug)]
pub enum ProcessTy {
    AlwaysComb,
    AlwaysFF,
    Initial,
    Final,
}

#[derive(Clone, Debug)]
pub struct ParallelProcess {
    pub ty: ProcessTy,
    pub event: Option<Sequential>,
    pub body: Vec<Sequential>,
}

#[derive(Clone, Debug)]
pub enum Parallel {
    Inst(Instance),
    ParAssign(Expr, Expr),
    Process(ParallelProcess),
}
