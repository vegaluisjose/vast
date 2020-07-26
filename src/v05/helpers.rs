use crate::v05::ast::*;

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

    pub fn new_param_uint(name: &str, value: u32) -> Decl {
        Decl::Param(name.to_string(), Expr::new_ulit_dec(32, &value.to_string()))
    }

    pub fn new_param_str(name: &str, value: &str) -> Decl {
        Decl::Param(name.to_string(), Expr::new_str(value))
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

impl Instance {
    pub fn new(id: &str, prim: &str) -> Instance {
        Instance {
            id: id.to_string(),
            prim: prim.to_string(),
            params: PMap::new(),
            ports: PMap::new(),
        }
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

    pub fn id(&self) -> String {
        self.id.to_string()
    }

    pub fn prim(&self) -> String {
        self.prim.to_string()
    }

    pub fn param_map(&self) -> &PMap {
        &self.params
    }

    pub fn port_map(&self) -> &PMap {
        &self.ports
    }
}

impl Parallel {
    pub fn new_inst(inst: Instance) -> Parallel {
        Parallel::Instance(inst)
    }

    pub fn id(&self) -> String {
        match self {
            Parallel::Instance(inst) => inst.id(),
            Parallel::Assign(lexpr, _) => lexpr.id(),
            _ => panic!("Error: always do not support id"),
        }
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

impl Module {
    pub fn new_with_name(name: &str) -> Module {
        Module {
            name: name.to_string(),
            params: Vec::new(),
            ports: Vec::new(),
            body: Vec::new(),
        }
    }

    pub fn add_param_uint(&mut self, name: &str, value: u32) {
        self.params.push(Decl::new_param_uint(name, value));
    }

    pub fn add_param_str(&mut self, name: &str, value: &str) {
        self.params.push(Decl::new_param_str(name, value));
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
