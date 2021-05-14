use std::collections::HashMap;
use std::rc::Rc;

pub type Id = String;
pub type Map = HashMap<Id, Expr>;

/// Unary reduction operators.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Unop {
    LogNot,
    Not,
    And,
    Nand,
    Or,
    Nor,
    Xor,
    Xnor,
}

/// Binary operators.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Binop {
    LogOr,
    LogAnd,
    Add,
    Sub,
    Mul,
    Gt,
    Lt,
    Geq,
    Leq,
    Equal,
    NotEqual,
    IndexBit,
    BitAnd,
    BitOr,
    ShiftLeft,
}

/// Ternaray operations
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Terop {
    Mux,
    Slice,
    IndexSlice,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Radix {
    Dec,
    Bin,
    Hex,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct InstancePath {
    pub path: Vec<Id>,
}

#[derive(Default, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ExprConcat {
    pub exprs: Vec<Expr>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Expr {
    Ref(Id),
    Int(i32),
    ULit(u32, Radix, String),
    Str(String),
    Signed(Rc<Expr>),
    IPath(InstancePath, Option<Rc<Expr>>),
    Unop(Unop, Rc<Expr>),
    Binop(Binop, Rc<Expr>, Rc<Expr>),
    Terop(Terop, Rc<Expr>, Rc<Expr>, Rc<Expr>),
    Concat(ExprConcat),
    Repeat(u64, Rc<Expr>),
    Call(Id, Vec<Expr>),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum AttributeTy {
    Val(String),
    Stmt(String, String),
}

/// Representation for attributes
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Attribute {
    pub attrs: Vec<AttributeTy>,
}

/// Event type for a task.
#[derive(Clone, Debug)]
pub enum EventTy {
    /// A positive edge triggered event.
    Posedge,
    /// A negative edge triggered event.
    Negedge,
}

/// Instance of a module.
#[derive(Clone, Debug)]
pub struct Instance {
    /// Name of this instance.
    pub id: Id,
    /// XXX
    pub prim: Id,
    /// Values for the parameters.
    pub params: Map,
    /// Wires for the ports.
    pub ports: Map,
    /// Attributes for the instance.
    pub attr: Attribute,
}

/// The type of assignment.
#[derive(Clone, Debug)]
pub enum AssignTy {
    /// A blocking assignment.
    Blocking,
    /// A non-blocking assignment.
    NonBlocking,
}

/// Representation for the case statement
// T ~> Sequential type
#[derive(Clone, Debug)]
pub struct GenericCaseBranch<T> {
    /// The conditional guard for this case.
    pub cond: Expr,
    /// The body for this case.
    pub body: Vec<T>,
}

/// Representation for the default case in a case statement.
// T ~> Sequential type
#[derive(Clone, Debug)]
pub struct GenericCaseDefault<T> {
    pub body: Vec<T>,
}

/// A case expression.
// T ~> Sequential type
#[derive(Clone, Debug)]
pub struct GenericCase<T> {
    /// The condition of the case expression.
    pub cond: Expr,
    /// Conditional branches for the case.
    pub branches: Vec<GenericCaseBranch<T>>,
    /// The default case for case statement.
    pub default: Option<GenericCaseDefault<T>>,
}

/// A port of a module or a function.
// T ~> Declaration type
#[derive(Clone, Debug)]
pub enum GenericPort<T> {
    /// An input port.
    Input(T),
    /// An output port.
    Output(T),
}

// F ~> Functiom type
// T ~> Declaration type
// U ~> Sequential type
// V ~> Data Type
#[derive(Clone, Debug)]
pub struct GenericFunction<F, T, U, V> {
    /// Function type (DPI-C).
    pub ty: F,
    /// Name of the function.
    pub name: Id,
    /// Ports of the function.
    pub ports: Vec<GenericPort<T>>,
    /// Declarations in this function.
    pub decls: Vec<T>,
    /// Body of this function.
    pub body: Vec<U>,
    /// Return value from this function.
    pub ret: V,
}

// T ~> Declaration type
// U ~> Parallel type
#[derive(Clone, Debug)]
pub enum GenericStmt<T, U> {
    /// A declaration parameterized on the verilog standard.
    Decl(T),
    /// A parallel task.
    Parallel(U),
    /// A raw string that represents a valid Verilog statement
    RawStr(String),
}

// T ~> Declaration type
// U ~> Parallel type
#[derive(Clone, Debug)]
pub struct GenericModule<T, U> {
    /// Name of the module.
    pub name: String,
    /// Parameters for the module.
    pub params: Vec<T>,
    /// Ports of this module.
    pub ports: Vec<GenericPort<T>>,
    /// Body of this module.
    pub body: Vec<GenericStmt<T, U>>,
    /// Attributes for this module.
    pub attr: Attribute,
}
