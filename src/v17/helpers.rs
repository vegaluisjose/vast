use crate::v17::ast::*;
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

impl Port {
    pub fn new_input(name: &str, width: u64) -> Port {
        let ty = Ty::Width(width);
        let logic = Decl::Logic(name.to_string(), ty);
        Port::Input(logic)
    }

    pub fn new_input_int(name: &str) -> Port {
        let inp = Decl::Int(name.to_string(), Ty::Int);
        Port::Input(inp)
    }

    pub fn new_output(name: &str, width: u64) -> Port {
        let ty = Ty::Width(width);
        let logic = Decl::Logic(name.to_string(), ty);
        Port::Output(logic)
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

impl Sequential {
    pub fn new_seqexpr<E>(expr: E) -> Sequential
    where
        E: Into<Expr>,
    {
        Sequential::SeqExpr(expr.into())
    }

    pub fn new_error(msg: &str) -> Sequential {
        Sequential::Error(msg.to_string())
    }

    pub fn new_display(msg: &str) -> Sequential {
        Sequential::Display(msg.to_string())
    }

    pub fn new_return<E>(expr: E) -> Sequential
    where
        E: Into<Expr>,
    {
        Sequential::Return(expr.into())
    }

    pub fn new_assert<E>(expr: E) -> Sequential
    where
        E: Into<Expr>,
    {
        Sequential::Assert(expr.into(), None)
    }

    pub fn new_assert_with_else<E, S>(expr: E, seq: S) -> Sequential
    where
        E: Into<Expr>,
        S: Into<Sequential>,
    {
        Sequential::Assert(expr.into(), Some(Rc::new(seq.into())))
    }

    pub fn new_blk_assign<L, R>(lexpr: L, rexpr: R) -> Sequential
    where
        L: Into<Expr>,
        R: Into<Expr>,
    {
        Sequential::SeqAssign(lexpr.into(), rexpr.into(), AssignTy::Blocking)
    }

    pub fn new_nonblk_assign<L, R>(lexpr: L, rexpr: R) -> Sequential
    where
        L: Into<Expr>,
        R: Into<Expr>,
    {
        Sequential::SeqAssign(lexpr.into(), rexpr.into(), AssignTy::NonBlocking)
    }

    pub fn new_case(case: Case) -> Sequential {
        Sequential::SeqCase(case)
    }

    pub fn new_call<E>(call: E) -> Sequential
    where
        E: Into<Expr>,
    {
        Sequential::Call(call.into())
    }
}

impl SequentialIfElse {
    pub fn new<E>(cond: E) -> Self
    where
        E: Into<Expr>,
    {
        SequentialIfElse {
            cond: Some(cond.into()),
            body: vec![],
            else_branch: None,
            unique: false,
        }
    }

    /// Marks the branches of this if-else chain with the `unique` annotation
    /// in Verilog.
    pub fn set_unique(&mut self) -> &mut Self {
        self.unique = true;
        self
    }

    /// Add a new statement in to the body of the true statement for this
    /// conditional block.
    pub fn add_seq<S>(&mut self, seq: S) -> &mut Self
    where
        S: Into<Sequential>,
    {
        self.body.push(seq.into());
        self
    }

    /// Set the alternate branch for this conditional.
    pub fn set_else<S>(&mut self, seq: S)
    where
        S: Into<Sequential>,
    {
        self.else_branch = Some(Rc::new(seq.into()));
    }
}

impl ParallelProcess {
    pub fn new_always_comb() -> Self {
        ParallelProcess {
            ty: ProcessTy::AlwaysComb,
            event: None,
            body: Vec::new(),
        }
    }

    pub fn new_always_ff() -> Self {
        ParallelProcess {
            ty: ProcessTy::AlwaysFF,
            event: None,
            body: Vec::new(),
        }
    }

    pub fn new_initial() -> Self {
        ParallelProcess {
            ty: ProcessTy::Initial,
            event: None,
            body: Vec::new(),
        }
    }

    pub fn new_final() -> Self {
        ParallelProcess {
            ty: ProcessTy::Final,
            event: None,
            body: Vec::new(),
        }
    }

    pub fn ty(&self) -> &ProcessTy {
        &self.ty
    }

    pub fn body(&self) -> &Vec<Sequential> {
        &self.body
    }

    pub fn event(&self) -> Option<&Sequential> {
        self.event.as_ref()
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

    pub fn set_event<S>(&mut self, seq: S)
    where
        S: Into<Sequential>,
    {
        self.event = Some(seq.into())
    }
}

impl Parallel {
    pub fn new_inst(inst: Instance) -> Parallel {
        Parallel::Inst(inst)
    }

    pub fn new_process(par_process: ParallelProcess) -> Parallel {
        Parallel::Process(par_process)
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

    pub fn new_rawstr(s: String) -> Stmt {
        Stmt::RawStr(s)
    }
}

impl Function {
    pub fn new(name: &str, ret: Ty) -> Function {
        Function {
            ty: FunctionTy::Default,
            name: name.to_string(),
            ports: Vec::new(),
            decls: Vec::new(),
            body: Vec::new(),
            ret,
        }
    }

    pub fn ports(&self) -> &Vec<Port> {
        &self.ports
    }

    pub fn decls(&self) -> &Vec<Decl> {
        &self.decls
    }

    pub fn body(&self) -> &Vec<Sequential> {
        &self.body
    }

    pub fn ty(&self) -> &FunctionTy {
        &self.ty
    }

    pub fn add_input(&mut self, name: &str, width: u64) -> &mut Self {
        self.ports.push(Port::new_input(name, width));
        self
    }

    pub fn add_input_int(&mut self, name: &str) -> &mut Self {
        self.ports.push(Port::new_input_int(name));
        self
    }

    pub fn add_output(&mut self, name: &str, width: u64) -> &mut Self {
        self.ports.push(Port::new_output(name, width));
        self
    }

    pub fn add_logic(&mut self, name: &str, width: u64) -> &mut Self {
        self.decls.push(Decl::new_logic(name, width));
        self
    }

    pub fn add_stmt<S>(&mut self, stmt: S) -> &mut Self
    where
        S: Into<Sequential>,
    {
        self.body.push(stmt.into());
        self
    }

    pub fn set_return_type(&mut self, ret: Ty) {
        self.ret = ret;
    }

    pub fn export(&mut self) {
        self.ty = FunctionTy::Export;
    }

    pub fn import(&mut self) {
        self.ty = FunctionTy::Import;
    }
}

impl Decl {
    pub fn new_param_uint(name: &str, value: u32) -> Decl {
        Decl::Param(
            name.to_string(),
            Ty::new_int(),
            Expr::new_ulit_dec(32, &value.to_string()),
        )
    }

    pub fn new_logic<S>(name: S, width: u64) -> Decl
    where
        S: AsRef<str>,
    {
        Decl::Logic(name.as_ref().to_string(), Ty::new_width(width))
    }

    pub fn new_int<S>(name: S) -> Decl
    where
        S: AsRef<str>,
    {
        Decl::Int(name.as_ref().to_string(), Ty::Int)
    }

    pub fn new_func(func: Function) -> Decl {
        Decl::Func(func)
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

    pub fn add_input(&mut self, name: &str, width: u64) -> &mut Self {
        self.ports.push(Port::new_input(name, width));
        self
    }

    pub fn add_output(&mut self, name: &str, width: u64) -> &mut Self {
        self.ports.push(Port::new_output(name, width));
        self
    }

    pub fn add_decl(&mut self, decl: Decl) -> &mut Self {
        self.body.push(Stmt::new_decl(decl));
        self
    }

    pub fn add_function(&mut self, func: Function) -> &mut Self {
        self.body.push(Stmt::new_decl(Decl::new_func(func)));
        self
    }

    pub fn add_instance(&mut self, inst: Instance) -> &mut Self {
        self.body.push(Stmt::new_parallel(Parallel::new_inst(inst)));
        self
    }

    pub fn add_process(&mut self, proc: ParallelProcess) -> &mut Self {
        self.body
            .push(Stmt::new_parallel(Parallel::new_process(proc)));
        self
    }

    pub fn add_stmt(&mut self, stmt: Stmt) -> &mut Self {
        self.body.push(stmt);
        self
    }

    pub fn set_attr(&mut self, attr: Attribute) {
        self.attr = attr;
    }
}
