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

    pub fn add_stmt(&mut self, stmt: Sequential) {
        self.body.push(stmt);
    }

    pub fn add_case(&mut self, case: Case) {
        self.body.push(Sequential::new_case(case));
    }

    pub fn body(&self) -> &Vec<Sequential> {
        &self.body
    }
}

impl CaseDefault {
    pub fn add_stmt(&mut self, stmt: Sequential) {
        self.body.push(stmt);
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

    pub fn add_branch(&mut self, branch: CaseBranch) {
        self.branches.push(branch);
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

    pub fn new_non_blk_assign(lexpr: Expr, rexpr: Expr) -> Sequential {
        Sequential::SeqAssign(lexpr, rexpr, AssignTy::NonBlocking)
    }

    pub fn new_case(case: Case) -> Sequential {
        Sequential::SeqCase(case)
    }

    pub fn new_call(call: Expr) -> Sequential {
        Sequential::SeqCall(call)
    }
}

impl Default for AlwaysComb {
    fn default() -> AlwaysComb {
        AlwaysComb { body: Vec::new() }
    }
}

impl AlwaysComb {
    pub fn add_stmt(&mut self, stmt: Sequential) {
        self.body.push(stmt);
    }

    pub fn add_case(&mut self, case: Case) {
        self.body.push(Sequential::new_case(case));
    }

    pub fn body(&self) -> &Vec<Sequential> {
        &self.body
    }
}

impl Parallel {
    pub fn new_inst(inst: Instance) -> Parallel {
        Parallel::Inst(inst)
    }

    pub fn new_always_comb(always: AlwaysComb) -> Parallel {
        Parallel::ParAlwaysComb(always)
    }
}

impl Stmt {
    pub fn new_parallel(par: Parallel) -> Stmt {
        Stmt::Parallel(par)
    }

    pub fn new_decl(decl: Decl) -> Stmt {
        Stmt::Decl(decl)
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

    pub fn add_input(&mut self, name: &str, width: u64) {
        self.inputs.push(Port::new_input(name, width));
    }

    pub fn add_logic(&mut self, name: &str, width: u64) {
        self.decls.push(Decl::new_logic(name, width));
    }

    pub fn add_stmt(&mut self, stmt: Sequential) {
        self.body.push(stmt);
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

    pub fn new_logic(name: &str, width: u64) -> Decl {
        Decl::Logic(name.to_string(), Ty::new_width(width))
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

    pub fn add_input(&mut self, name: &str, width: u64) {
        self.ports.push(Port::new_input(name, width));
    }

    pub fn add_output(&mut self, name: &str, width: u64) {
        self.ports.push(Port::new_output(name, width));
    }

    pub fn add_function(&mut self, func: Function) {
        self.body.push(Stmt::new_decl(Decl::new_func(func)));
    }

    pub fn add_instance(&mut self, inst: Instance) {
        self.body.push(Stmt::new_parallel(Parallel::new_inst(inst)));
    }

    pub fn add_always_comb(&mut self, always: AlwaysComb) {
        self.body
            .push(Stmt::new_parallel(Parallel::new_always_comb(always)));
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
}
