use crate::v05::ast::*;
use std::rc::Rc;

impl Ty {
    pub fn new_int() -> Ty {
        Ty::Int
    }

    pub fn new_width(width: u64) -> Ty {
        assert!(width > 0, "Error: width must be greater than zero");
        Ty::Width(width)
    }

    pub fn width(&self) -> u64 {
        match self {
            Ty::Width(w) => *w,
            _ => panic!("Error: type does not support width"),
        }
    }
}

impl Decl {
    pub fn new_int(name: &str) -> Decl {
        Decl::Int(name.to_string(), Ty::new_int())
    }

    pub fn new_wire(name: &str, width: u64) -> Decl {
        Decl::Wire(name.to_string(), Ty::new_width(width))
    }

    pub fn new_reg(name: &str, width: u64) -> Decl {
        Decl::Reg(name.to_string(), Ty::new_width(width))
    }

    pub fn new_array(name: &str, width: u64, depth: u64) -> Decl {
        Decl::Array(name.to_string(), Ty::new_width(width), Ty::new_width(depth))
    }

    pub fn new_param_uint(name: &str, value: u32) -> Decl {
        Decl::Param(name.to_string(), Expr::new_ulit_dec(32, &value.to_string()))
    }

    pub fn new_param_str(name: &str, value: &str) -> Decl {
        Decl::Param(name.to_string(), Expr::new_str(value))
    }

    pub fn new_attribute_decl(attr: Attribute, decl: Decl) -> Decl {
        Decl::AttributeDecl(attr, Rc::new(decl))
    }
}

impl Port {
    pub fn new_input(name: &str, width: u64) -> Port {
        let ty = Ty::Width(width);
        let wire = Decl::Wire(name.to_string(), ty);
        Port::Input(wire)
    }

    pub fn new_output(name: &str, width: u64) -> Port {
        let ty = Ty::Width(width);
        let wire = Decl::Wire(name.to_string(), ty);
        Port::Output(wire)
    }

    pub fn new_output_reg(name: &str, width: u64) -> Port {
        let ty = Ty::Width(width);
        let reg = Decl::Reg(name.to_string(), ty);
        Port::Output(reg)
    }
}

impl SequentialIfElse {
    pub fn new<E>(cond: E) -> Self
    where
        E: Into<Expr>,
    {
        SequentialIfElse {
            cond: Some(cond.into()),
            body: Vec::new(),
            elsebr: None,
        }
    }

    pub fn cond(&self) -> Option<&Expr> {
        self.cond.as_ref()
    }

    pub fn body(&self) -> &Vec<Sequential> {
        &self.body
    }

    pub fn else_branch(&self) -> Option<&Sequential> {
        self.elsebr.as_deref()
    }

    pub fn add_seq<S>(&mut self, seq: S)
    where
        S: Into<Sequential>,
    {
        self.body.push(seq.into());
    }

    pub fn set_else<S>(&mut self, seq: S)
    where
        S: Into<Sequential>,
    {
        self.elsebr = Some(Rc::new(seq.into()));
    }
}

impl Sequential {
    pub fn new_posedge(name: &str) -> Self {
        Sequential::Event(EventTy::Posedge, name.into())
    }

    pub fn new_blk_assign<L, R>(lexpr: L, rexpr: R) -> Sequential
    where
        L: Into<Expr>,
        R: Into<Expr>,
    {
        Sequential::Assign(lexpr.into(), rexpr.into(), AssignTy::Blocking)
    }

    pub fn new_nonblk_assign<L, R>(lexpr: L, rexpr: R) -> Sequential
    where
        L: Into<Expr>,
        R: Into<Expr>,
    {
        Sequential::Assign(lexpr.into(), rexpr.into(), AssignTy::NonBlocking)
    }

    pub fn new_case(case: Case) -> Sequential {
        Sequential::SeqCase(case)
    }
}

impl ParallelProcess {
    pub fn new_always() -> Self {
        ParallelProcess {
            ty: ProcessTy::Always,
            event: None,
            body: Vec::new(),
        }
    }

    pub fn ty(&self) -> &ProcessTy {
        &self.ty
    }

    pub fn event(&self) -> Option<&Sequential> {
        self.event.as_ref()
    }

    pub fn body(&self) -> &Vec<Sequential> {
        &self.body
    }

    pub fn add_seq<S>(&mut self, seq: S) -> &mut Self
    where
        S: Into<Sequential>,
    {
        self.body.push(seq.into());
        self
    }

    pub fn set_event<S>(&mut self, seq: S)
    where
        S: Into<Sequential>,
    {
        self.event = Some(seq.into())
    }
}

impl Parallel {
    pub fn new_inst(inst: Instance) -> Parallel {
        Parallel::from(inst)
    }

    pub fn id(&self) -> String {
        match self {
            Parallel::Inst(inst) => inst.id(),
            Parallel::Assign(lexpr, _) => lexpr.id(),
            _ => panic!("Error: always do not support id"),
        }
    }
}

impl Stmt {
    pub fn new_parallel<P>(par: P) -> Stmt
    where
        P: Into<Parallel>,
    {
        Stmt::Parallel(par.into())
    }

    pub fn new_decl(decl: Decl) -> Stmt {
        Stmt::Decl(decl)
    }
}

impl Module {
    pub fn new(name: &str) -> Module {
        Module {
            name: name.to_string(),
            params: Vec::new(),
            ports: Vec::new(),
            body: Vec::new(),
            attr: Attribute::default(),
        }
    }

    pub fn name(&self) -> String {
        self.name.to_string()
    }

    pub fn body(&self) -> &Vec<Stmt> {
        &self.body
    }

    pub fn ports(&self) -> &Vec<Port> {
        &self.ports
    }

    pub fn params(&self) -> &Vec<Decl> {
        &self.params
    }

    pub fn attr(&self) -> &Attribute {
        &self.attr
    }

    pub fn add_param_uint(&mut self, name: &str, value: u32) {
        self.params.push(Decl::new_param_uint(name, value));
    }

    pub fn add_param_str(&mut self, name: &str, value: &str) {
        self.params.push(Decl::new_param_str(name, value));
    }

    pub fn add_port(&mut self, port: Port) {
        self.ports.push(port);
    }

    pub fn add_input(&mut self, name: &str, width: u64) {
        self.ports.push(Port::new_input(name, width));
    }

    pub fn add_output(&mut self, name: &str, width: u64) {
        self.ports.push(Port::new_output(name, width));
    }

    pub fn add_output_reg(&mut self, name: &str, width: u64) {
        self.ports.push(Port::new_output_reg(name, width));
    }

    pub fn add_instance(&mut self, inst: Instance) {
        self.body.push(Stmt::new_parallel(Parallel::new_inst(inst)));
    }

    pub fn add_decl(&mut self, decl: Decl) {
        self.body.push(Stmt::from(decl));
    }

    pub fn add_stmt<S>(&mut self, stmt: S)
    where
        S: Into<Stmt>,
    {
        self.body.push(stmt.into());
    }

    pub fn set_attr(&mut self, attr: Attribute) {
        self.attr = attr;
    }
}

impl CaseBranch {
    pub fn new<E>(cond: E) -> CaseBranch
    where
        E: Into<Expr>,
    {
        CaseBranch {
            cond: cond.into(),
            body: Vec::new(),
        }
    }

    pub fn add_seq<S>(&mut self, seq: S) -> &mut Self
    where
        S: Into<Sequential>,
    {
        self.body.push(seq.into());
        self
    }

    pub fn add_case(&mut self, case: Case) -> &mut Self {
        self.body.push(Sequential::new_case(case));
        self
    }

    pub fn body(&self) -> &Vec<Sequential> {
        &self.body
    }
}

impl CaseDefault {
    pub fn add_seq<S>(&mut self, seq: S) -> &mut Self
    where
        S: Into<Sequential>,
    {
        self.body.push(seq.into());
        self
    }

    pub fn body(&self) -> &Vec<Sequential> {
        &self.body
    }
}

impl Default for CaseDefault {
    fn default() -> CaseDefault {
        CaseDefault { body: Vec::new() }
    }
}

impl Case {
    pub fn new<E>(cond: E) -> Case
    where
        E: Into<Expr>,
    {
        Case {
            cond: cond.into(),
            branches: Vec::new(),
            default: None,
        }
    }

    pub fn add_branch(&mut self, branch: CaseBranch) -> &mut Self {
        self.branches.push(branch);
        self
    }

    pub fn set_default(&mut self, branch: CaseDefault) {
        self.default = Some(branch);
    }

    pub fn branches(&self) -> &Vec<CaseBranch> {
        &self.branches
    }

    pub fn default(&self) -> &CaseDefault {
        self.default
            .as_ref()
            .unwrap_or_else(|| panic!("Default branch has not been set"))
    }
}
