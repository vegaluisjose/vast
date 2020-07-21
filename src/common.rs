use crate::util::pretty_print::PrettyPrint;
use pretty::RcDoc;
use std::fmt;
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

impl PrettyPrint for Rop {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            Rop::LogNot => RcDoc::text("!"),
            Rop::Not => RcDoc::text("~"),
            Rop::And => RcDoc::text("&"),
            Rop::Nand => RcDoc::text("~&"),
            Rop::Or => RcDoc::text("|"),
            Rop::Nor => RcDoc::text("~|"),
            Rop::Xor => RcDoc::text("^"),
            Rop::Xnor => RcDoc::text("~^"),
        }
    }
}

impl fmt::Display for Rop {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_pretty())
    }
}

#[derive(Clone, Debug)]
pub enum Binop {
    Add,
}

impl PrettyPrint for Binop {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            Binop::Add => RcDoc::text("+"),
        }
    }
}

impl fmt::Display for Binop {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_pretty())
    }
}

#[derive(Clone, Debug)]
pub enum Expr {
    Ref(Id),
    Unop(Rop, Rc<Expr>),
    Binop(Binop, Rc<Expr>, Rc<Expr>),
}

impl PrettyPrint for Expr {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            Expr::Ref(name) => RcDoc::as_string(name),
            Expr::Unop(op, input) => op.to_doc().append(input.to_doc()),
            Expr::Binop(op, lhs, rhs) => lhs
                .to_doc()
                .append(RcDoc::space())
                .append(op.to_doc())
                .append(RcDoc::space())
                .append(rhs.to_doc()),
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_pretty())
    }
}

#[derive(Clone, Debug)]
pub enum EventTy {
    Posedge,
    Negedge,
}

impl PrettyPrint for EventTy {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            EventTy::Posedge => RcDoc::text("posedge"),
            EventTy::Negedge => RcDoc::text("negedge"),
        }
    }
}

impl fmt::Display for EventTy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_pretty())
    }
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
