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

    pub fn add_expr<E>(&mut self, expr: E)
    where
        E: Into<Expr>,
    {
        self.exprs.push(expr.into());
    }
}

impl Expr {
    pub fn id(&self) -> String {
        match self {
            Expr::Ref(id) => id.to_string(),
            Expr::Signed(e) => e.id(),
            _ => panic!("Error: do not support id"),
        }
    }

    pub fn new_ref<S>(name: S) -> Expr
    where
        S: AsRef<str>,
    {
        Expr::Ref(name.as_ref().to_string())
    }

    pub fn new_signed_ref<S>(name: S) -> Expr
    where
        S: AsRef<str>,
    {
        Expr::Signed(Rc::new(Expr::Ref(name.as_ref().to_string())))
    }

    pub fn new_signed<E>(expr: E) -> Expr
    where
        E: Into<Expr>,
    {
        Expr::Signed(Rc::new(expr.into()))
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

    pub fn new_bit_or<L, R>(lhs: L, rhs: R) -> Expr
    where
        L: Into<Expr>,
        R: Into<Expr>,
    {
        Expr::Binop(Binop::BitOr, Rc::new(lhs.into()), Rc::new(rhs.into()))
    }

    pub fn new_bit_and<L, R>(lhs: L, rhs: R) -> Expr
    where
        L: Into<Expr>,
        R: Into<Expr>,
    {
        Expr::Binop(Binop::BitAnd, Rc::new(lhs.into()), Rc::new(rhs.into()))
    }

    pub fn new_logical_or<L, R>(lhs: L, rhs: R) -> Expr
    where
        L: Into<Expr>,
        R: Into<Expr>,
    {
        Expr::Binop(Binop::LogOr, Rc::new(lhs.into()), Rc::new(rhs.into()))
    }

    pub fn new_logical_and<L, R>(lhs: L, rhs: R) -> Expr
    where
        L: Into<Expr>,
        R: Into<Expr>,
    {
        Expr::Binop(Binop::LogAnd, Rc::new(lhs.into()), Rc::new(rhs.into()))
    }

    pub fn new_add<L, R>(lhs: L, rhs: R) -> Expr
    where
        L: Into<Expr>,
        R: Into<Expr>,
    {
        Expr::Binop(Binop::Add, Rc::new(lhs.into()), Rc::new(rhs.into()))
    }

    pub fn new_shift_left<L, R>(lhs: L, rhs: R) -> Expr
    where
        L: Into<Expr>,
        R: Into<Expr>,
    {
        Expr::Binop(Binop::ShiftLeft, Rc::new(lhs.into()), Rc::new(rhs.into()))
    }

    pub fn new_sub<L, R>(lhs: L, rhs: R) -> Expr
    where
        L: Into<Expr>,
        R: Into<Expr>,
    {
        Expr::Binop(Binop::Sub, Rc::new(lhs.into()), Rc::new(rhs.into()))
    }

    pub fn new_gt<L, R>(lhs: L, rhs: R) -> Expr
    where
        L: Into<Expr>,
        R: Into<Expr>,
    {
        Expr::Binop(Binop::Gt, Rc::new(lhs.into()), Rc::new(rhs.into()))
    }

    pub fn new_lt<L, R>(lhs: L, rhs: R) -> Expr
    where
        L: Into<Expr>,
        R: Into<Expr>,
    {
        Expr::Binop(Binop::Lt, Rc::new(lhs.into()), Rc::new(rhs.into()))
    }

    pub fn new_geq<L, R>(lhs: L, rhs: R) -> Expr
    where
        L: Into<Expr>,
        R: Into<Expr>,
    {
        Expr::Binop(Binop::Geq, Rc::new(lhs.into()), Rc::new(rhs.into()))
    }

    pub fn new_leq<L, R>(lhs: L, rhs: R) -> Expr
    where
        L: Into<Expr>,
        R: Into<Expr>,
    {
        Expr::Binop(Binop::Leq, Rc::new(lhs.into()), Rc::new(rhs.into()))
    }

    pub fn new_eq<L, R>(lhs: L, rhs: R) -> Expr
    where
        L: Into<Expr>,
        R: Into<Expr>,
    {
        Expr::Binop(Binop::Equal, Rc::new(lhs.into()), Rc::new(rhs.into()))
    }

    pub fn new_neq<L, R>(lhs: L, rhs: R) -> Expr
    where
        L: Into<Expr>,
        R: Into<Expr>,
    {
        Expr::Binop(Binop::NotEqual, Rc::new(lhs.into()), Rc::new(rhs.into()))
    }

    pub fn new_mul<L, R>(lhs: L, rhs: R) -> Expr
    where
        L: Into<Expr>,
        R: Into<Expr>,
    {
        Expr::Binop(Binop::Mul, Rc::new(lhs.into()), Rc::new(rhs.into()))
    }

    pub fn new_mux<C, T, F>(cond: C, tru: T, fal: F) -> Expr
    where
        C: Into<Expr>,
        T: Into<Expr>,
        F: Into<Expr>,
    {
        Expr::Terop(
            Terop::Mux,
            Rc::new(cond.into()),
            Rc::new(tru.into()),
            Rc::new(fal.into()),
        )
    }

    pub fn new_not<E>(exp: E) -> Expr
    where
        E: Into<Expr>,
    {
        Expr::Unop(Unop::Not, Rc::new(exp.into()))
    }

    pub fn new_slice<H, L>(var: &str, hi: H, lo: L) -> Expr
    where
        H: Into<Expr>,
        L: Into<Expr>,
    {
        Expr::Terop(
            Terop::Slice,
            Rc::new(Expr::new_ref(var)),
            Rc::new(hi.into()),
            Rc::new(lo.into()),
        )
    }

    pub fn new_index_slice<E>(var: &str, lo: E, width: u32) -> Expr
    where
        E: Into<Expr>,
    {
        Expr::Terop(
            Terop::IndexSlice,
            Rc::new(Expr::new_ref(var)),
            Rc::new(lo.into()),
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

    pub fn new_index_expr<E>(var: &str, expr: E) -> Expr
    where
        E: Into<Expr>,
    {
        Expr::Binop(
            Binop::IndexBit,
            Rc::new(Expr::new_ref(var)),
            Rc::new(expr.into()),
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

    pub fn new_repeat<E>(times: u64, expr: E) -> Expr
    where
        E: Into<Expr>,
    {
        Expr::Repeat(times, Rc::new(expr.into()))
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

    pub fn add_param<E>(&mut self, param: &str, value: E)
    where
        E: Into<Expr>,
    {
        self.params.insert(param.to_string(), value.into());
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

    pub fn connect<E>(&mut self, port: &str, expr: E)
    where
        E: Into<Expr>,
    {
        self.ports.insert(port.to_string(), expr.into());
    }

    pub fn connect_ref(&mut self, port: &str, id: &str) {
        self.ports.insert(port.to_string(), Expr::new_ref(id));
    }
}
