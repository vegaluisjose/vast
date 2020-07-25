use std::rc::Rc;

pub type Id = String;

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
}

#[derive(Clone, Debug)]
pub enum Expr {
    Ref(Id),
    Const(i32),
    Unop(Rop, Rc<Expr>),
    Binop(Binop, Rc<Expr>, Rc<Expr>),
}

#[derive(Clone, Debug)]
pub enum EventTy {
    Posedge,
    Negedge,
}

#[derive(Clone, Debug)]
pub enum GenericPort<T> {
    Input(T),
    Output(T),
}

#[derive(Clone, Debug)]
pub enum GenericStmt<T, U> {
    Decl(T),
    Parallel(U),
}

#[derive(Clone, Debug)]
pub struct GenericModule<T, U> {
    pub name: String,
    pub ports: Vec<GenericPort<T>>,
    pub body: Vec<GenericStmt<T, U>>,
}
