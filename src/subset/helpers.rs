use crate::subset::ast::*;
use std::rc::Rc;

impl InstancePath {
    pub fn new(path: &str) -> InstancePath {
        let p: Vec<String> = path.split('.').map(|x| x.to_string()).collect();
        InstancePath { path: p }
    }

    pub fn path(&self) -> &Vec<Id> {
        &self.path
    }

    pub fn add_inst(&mut self, name: &str) {
        self.path.push(name.to_string());
    }
}

impl ExprConcat {
    pub fn exprs(&self) -> &Vec<Expr> {
        &self.exprs
    }

    pub fn add_expr(&mut self, expr: Expr) {
        self.exprs.push(expr);
    }
}

impl Expr {
    pub fn id(&self) -> String {
        match self {
            Expr::Ref(id) => id.to_string(),
            _ => panic!("Error: do not support id"),
        }
    }

    pub fn new_ref<S>(name: S) -> Expr
    where
        S: AsRef<str>,
    {
        Expr::Ref(name.as_ref().to_string())
    }

    pub fn new_signed_ref(name: &str) -> Expr {
        Expr::Signed(Rc::new(Expr::Ref(name.to_string())))
    }

    pub fn new_signed(expr: Expr) -> Expr {
        Expr::Signed(Rc::new(expr))
    }

    pub fn new_str(value: &str) -> Expr {
        Expr::Str(value.to_string())
    }

    pub fn new_ulit_dec(width: u32, value: &str) -> Expr {
        assert!(width > 0, "Error: width must be greater than zero");
        Expr::ULit(width, Radix::Dec, value.to_string())
    }

    pub fn new_ulit_hex(width: u32, value: &str) -> Expr {
        assert!(width > 0, "Error: width must be greater than zero");
        Expr::ULit(width, Radix::Hex, value.to_string())
    }

    pub fn new_ulit_bin(width: u32, value: &str) -> Expr {
        assert!(width > 0, "Error: width must be greater than zero");
        Expr::ULit(width, Radix::Bin, value.to_string())
    }

    pub fn new_bit_or(lhs: Expr, rhs: Expr) -> Expr {
        Expr::Binop(Binop::BitOr, Rc::new(lhs), Rc::new(rhs))
    }

    pub fn new_bit_and(lhs: Expr, rhs: Expr) -> Expr {
        Expr::Binop(Binop::BitAnd, Rc::new(lhs), Rc::new(rhs))
    }

    pub fn new_logical_or(lhs: Expr, rhs: Expr) -> Expr {
        Expr::Binop(Binop::LogOr, Rc::new(lhs), Rc::new(rhs))
    }

    pub fn new_logical_and(lhs: Expr, rhs: Expr) -> Expr {
        Expr::Binop(Binop::LogAnd, Rc::new(lhs), Rc::new(rhs))
    }

    pub fn new_add(lhs: Expr, rhs: Expr) -> Expr {
        Expr::Binop(Binop::Add, Rc::new(lhs), Rc::new(rhs))
    }

    pub fn new_gt(lhs: Expr, rhs: Expr) -> Expr {
        Expr::Binop(Binop::Gt, Rc::new(lhs), Rc::new(rhs))
    }

    pub fn new_lt(lhs: Expr, rhs: Expr) -> Expr {
        Expr::Binop(Binop::Lt, Rc::new(lhs), Rc::new(rhs))
    }

    pub fn new_geq(lhs: Expr, rhs: Expr) -> Expr {
        Expr::Binop(Binop::Geq, Rc::new(lhs), Rc::new(rhs))
    }

    pub fn new_leq(lhs: Expr, rhs: Expr) -> Expr {
        Expr::Binop(Binop::Leq, Rc::new(lhs), Rc::new(rhs))
    }

    pub fn new_eq(lhs: Expr, rhs: Expr) -> Expr {
        Expr::Binop(Binop::Equal, Rc::new(lhs), Rc::new(rhs))
    }

    pub fn new_neq(lhs: Expr, rhs: Expr) -> Expr {
        Expr::Binop(Binop::NotEqual, Rc::new(lhs), Rc::new(rhs))
    }

    pub fn new_mul(lhs: Expr, rhs: Expr) -> Expr {
        Expr::Binop(Binop::Mul, Rc::new(lhs), Rc::new(rhs))
    }

    pub fn new_mux(cond: Expr, tru: Expr, fal: Expr) -> Expr {
        Expr::Terop(Terop::Mux, Rc::new(cond), Rc::new(tru), Rc::new(fal))
    }

    pub fn new_not(exp: Expr) -> Expr {
        Expr::Unop(Unop::Not, Rc::new(exp))
    }

    pub fn new_slice(var: &str, hi: Expr, lo: Expr) -> Expr {
        Expr::Terop(
            Terop::Slice,
            Rc::new(Expr::new_ref(var)),
            Rc::new(hi),
            Rc::new(lo),
        )
    }

    pub fn new_index_slice(var: &str, lo: Expr, width: u32) -> Expr {
        Expr::Terop(
            Terop::IndexSlice,
            Rc::new(Expr::new_ref(var)),
            Rc::new(lo),
            Rc::new(Expr::new_int(width as i32)),
        )
    }

    pub fn new_index_bit(var: &str, bit: i32) -> Expr {
        Expr::Binop(
            Binop::IndexBit,
            Rc::new(Expr::new_ref(var)),
            Rc::new(Expr::new_int(bit)),
        )
    }

    pub fn new_int(value: i32) -> Expr {
        Expr::Int(value)
    }

    pub fn new_ipath(path: &str) -> Expr {
        Expr::IPath(InstancePath::new(path), None)
    }

    pub fn new_ipath_with_index(path: &str, index: &str) -> Expr {
        Expr::IPath(InstancePath::new(path), Some(Rc::new(Expr::new_ref(index))))
    }

    pub fn new_call(name: &str, params: Vec<Expr>) -> Expr {
        Expr::Call(name.to_string(), params)
    }
}

impl AttributeTy {
    pub fn new_val(value: &str) -> AttributeTy {
        AttributeTy::Val(value.to_string())
    }

    pub fn new_stmt(id: &str, value: &str) -> AttributeTy {
        AttributeTy::Stmt(id.to_string(), value.to_string())
    }
}

impl Attribute {
    pub fn attrs(&self) -> &Vec<AttributeTy> {
        &self.attrs
    }

    pub fn add_val(&mut self, value: &str) {
        self.attrs.push(AttributeTy::new_val(value));
    }

    pub fn add_stmt(&mut self, id: &str, value: &str) {
        self.attrs.push(AttributeTy::new_stmt(id, value));
    }

    pub fn add_attr(&mut self, attr: AttributeTy) {
        self.attrs.push(attr);
    }
}

impl Instance {
    pub fn new(id: &str, prim: &str) -> Instance {
        Instance {
            id: id.to_string(),
            prim: prim.to_string(),
            params: Map::new(),
            ports: Map::new(),
            attr: Attribute::default(),
        }
    }

    pub fn id(&self) -> String {
        self.id.to_string()
    }

    pub fn prim(&self) -> String {
        self.prim.to_string()
    }

    pub fn attr(&self) -> &Attribute {
        &self.attr
    }

    pub fn param_map(&self) -> &Map {
        &self.params
    }

    pub fn port_map(&self) -> &Map {
        &self.ports
    }

    pub fn set_id(&mut self, id: &str) {
        self.id = id.to_string();
    }

    pub fn set_prim(&mut self, prim: &str) {
        self.prim = prim.to_string();
    }

    pub fn set_attr(&mut self, attr: Attribute) {
        self.attr = attr;
    }

    pub fn add_param(&mut self, param: &str, value: Expr) {
        self.params.insert(param.to_string(), value);
    }

    pub fn add_param_uint(&mut self, param: &str, value: u32) {
        self.params.insert(
            param.to_string(),
            Expr::new_ulit_dec(32, &value.to_string()),
        );
    }

    pub fn add_param_str(&mut self, param: &str, value: &str) {
        self.params.insert(param.to_string(), Expr::new_str(value));
    }

    pub fn connect(&mut self, port: &str, expr: Expr) {
        self.ports.insert(port.to_string(), expr);
    }

    pub fn connect_ref(&mut self, port: &str, id: &str) {
        self.ports.insert(port.to_string(), Expr::new_ref(id));
    }
}
