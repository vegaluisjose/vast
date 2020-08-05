use std::collections::HashMap;
use std::rc::Rc;

pub type Id = String;
pub type Map = HashMap<Id, Expr>;

// Reduce ops
#[derive(Clone, Debug)]
pub enum Rop {
    LogNot,
    Not,
    And,
    Nand,
    Or,
    Nor,
    Xor,
    Xnor,
}

#[derive(Clone, Debug)]
pub enum Binop {
    Add,
    Mul,
    Lt,
    Equal,
    NotEqual,
}

#[derive(Clone, Debug)]
pub enum Radix {
    Dec,
    Bin,
    Hex,
}

#[derive(Clone, Debug)]
pub enum Expr {
    Ref(Id),
    Int(i32),
    ULit(u32, Radix, String),
    Str(String),
    Unop(Rop, Rc<Expr>),
    Binop(Binop, Rc<Expr>, Rc<Expr>),
}

#[derive(Clone, Debug)]
pub enum EventTy {
    Posedge,
    Negedge,
}

#[derive(Clone, Debug)]
pub struct Instance {
    pub id: Id,
    pub prim: Id,
    pub params: Map,
    pub ports: Map,
}

#[derive(Clone, Debug)]
pub enum AssignTy {
    Blocking,
    NonBlocking,
}

// T ~> Declaration type
#[derive(Clone, Debug)]
pub enum GenericPort<T> {
    Input(T),
    Output(T),
}

// T ~> Declaration type
// U ~> Sequential type
// V ~> Data Type
#[derive(Clone, Debug)]
pub struct GenericFunction<T, U, V> {
    pub name: Id,
    pub inputs: Vec<GenericPort<T>>,
    pub decls: Vec<T>,
    pub body: Vec<U>,
    pub ret: V,
}

// T ~> Declaration type
// U ~> Parallel type
#[derive(Clone, Debug)]
pub enum GenericStmt<T, U> {
    Decl(T),
    Parallel(U),
}

// T ~> Declaration type
// U ~> Parallel type
#[derive(Clone, Debug)]
pub struct GenericModule<T, U> {
    pub name: String,
    pub params: Vec<T>,
    pub ports: Vec<GenericPort<T>>,
    pub body: Vec<GenericStmt<T, U>>,
}
