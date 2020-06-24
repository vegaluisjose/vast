use crate::pretty::PrettyPrinter;
use pretty::RcDoc;
use std::fmt;
use std::rc::Rc;

pub type Id = String;

#[derive(Clone, Debug)]
pub enum Ty {
    Int,
    Width(u64),
}

impl Ty {
    pub fn width(&self) -> u64 {
        match self {
            Ty::Width(w) => w.clone(),
            _ => panic!("Error: type does not support width"),
        }
    }
}

impl PrettyPrinter for Ty {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            Ty::Int => RcDoc::text("int"),
            Ty::Width(w) => match w {
                0 => panic!("Error: width must be greater than zero"),
                1 => RcDoc::nil(),
                n => RcDoc::text("[")
                    .append(RcDoc::as_string(n - 1))
                    .append(RcDoc::text(":"))
                    .append(RcDoc::text("0"))
                    .append(RcDoc::text("]")),
            },
        }
    }
}

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

impl PrettyPrinter for Rop {
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

impl PrettyPrinter for Binop {
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

impl PrettyPrinter for Expr {
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
    pub ports: Vec<GenericPort<T>>,
    pub stmt: Vec<GenericStmt<T, U>>,
}
