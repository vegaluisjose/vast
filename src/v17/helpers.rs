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

    pub fn new_output(name: &str, width: u64) -> Port {
        let ty = Ty::Width(width);
        let logic = Decl::Logic(name.to_string(), ty);
        Port::Output(logic)
    }
}

impl CaseBranch {
    pub fn new(cond: Expr) -> CaseBranch {
        CaseBranch {
            cond,
            body: Vec::new(),
        }
    }

    pub fn add_seq(&mut self, seq: Sequential) -> &mut Self {
        self.body.push(seq);
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
    pub fn add_seq(&mut self, seq: Sequential) -> &mut Self {
        self.body.push(seq);
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
    pub fn new(cond: Expr) -> Case {
        Case {
            cond,
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
        if let Some(default) = &self.default {
            &default
        } else {
            panic!("Default branch has not been set");
        }
    }
}

impl Sequential {
    pub fn new_seqexpr(expr: Expr) -> Sequential {
        Sequential::SeqExpr(expr)
    }

    pub fn new_error(msg: &str) -> Sequential {
        Sequential::Error(msg.to_string())
    }

    pub fn new_display(msg: &str) -> Sequential {
        Sequential::Display(msg.to_string())
    }

    pub fn new_return(expr: Expr) -> Sequential {
        Sequential::Return(expr)
    }

    pub fn new_assert(expr: Expr) -> Sequential {
        Sequential::Assert(expr, None)
    }

    pub fn new_assert_with_else(expr: Expr, seq: Sequential) -> Sequential {
        Sequential::Assert(expr, Some(Rc::new(seq)))
    }

    pub fn new_blk_assign(lexpr: Expr, rexpr: Expr) -> Sequential {
        Sequential::SeqAssign(lexpr, rexpr, AssignTy::Blocking)
    }

    pub fn new_nonblk_assign(lexpr: Expr, rexpr: Expr) -> Sequential {
        Sequential::SeqAssign(lexpr, rexpr, AssignTy::NonBlocking)
    }

    pub fn new_case(case: Case) -> Sequential {
        Sequential::SeqCase(case)
    }

    pub fn new_call(call: Expr) -> Sequential {
        Sequential::Call(call)
    }
}

impl SequentialIfElse {
    pub fn new(cond: Expr) -> Self {
        SequentialIfElse {
            cond: Some(cond),
            body: vec![],
            else_branch: None,
        }
    }

    pub fn add_seq(&mut self, seq: Sequential) -> &mut Self {
        self.body.push(seq);
        self
    }

    pub fn set_else(&mut self, seq: Sequential) {
        self.else_branch = Some(Rc::new(seq));
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

    pub fn add_seq(&mut self, seq: Sequential) -> &mut Self {
        self.body.push(seq);
        self
    }

    pub fn add_case(&mut self, case: Case) -> &mut Self {
        self.body.push(Sequential::new_case(case));
        self
    }

    pub fn set_event(&mut self, seq: Sequential) {
        self.event = Some(seq)
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
    pub fn new_parallel(par: Parallel) -> Stmt {
        Stmt::Parallel(par)
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
            name: name.to_string(),
            inputs: Vec::new(),
            decls: Vec::new(),
            body: Vec::new(),
            ret,
        }
    }

    pub fn inputs(&self) -> &Vec<Port> {
        &self.inputs
    }

    pub fn decls(&self) -> &Vec<Decl> {
        &self.decls
    }

    pub fn body(&self) -> &Vec<Sequential> {
        &self.body
    }

    pub fn add_input(&mut self, name: &str, width: u64) -> &mut Self {
        self.inputs.push(Port::new_input(name, width));
        self
    }

    pub fn add_logic(&mut self, name: &str, width: u64) -> &mut Self {
        self.decls.push(Decl::new_logic(name, width));
        self
    }

    pub fn add_stmt(&mut self, stmt: Sequential) -> &mut Self {
        self.body.push(stmt);
        self
    }

    pub fn set_return_type(&mut self, ret: Ty) {
        self.ret = ret;
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
