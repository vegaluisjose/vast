use vast::util::file::read_to_string;
use vast::v17::ast::*;

#[test]
fn test_expr_str() {
    let expr = Expr::new_str("multiply");
    let exp = r#""multiply""#.to_string();
    let res = expr.to_string();
    assert_eq!(res, exp, "\n\nresult:\n{}\nexpected:\n{}\n\n", res, exp);
}

#[test]
fn test_expr_ulit_bin() {
    let expr = Expr::new_ulit_bin(4, "1000");
    let exp = "4'b1000".to_string();
    let res = expr.to_string();
    assert_eq!(res, exp, "\n\nresult:\n{}\nexpected:\n{}\n\n", res, exp);
}

#[test]
fn test_expr_ulit_hex() {
    let expr = Expr::new_ulit_hex(8, "ff");
    let exp = "8'hff".to_string();
    let res = expr.to_string();
    assert_eq!(res, exp, "\n\nresult:\n{}\nexpected:\n{}\n\n", res, exp);
}

#[test]
fn test_expr_ulit_dec() {
    let expr = Expr::new_ulit_dec(16, "78");
    let exp = "16'd78".to_string();
    let res = expr.to_string();
    assert_eq!(res, exp, "\n\nresult:\n{}\nexpected:\n{}\n\n", res, exp);
}

#[test]
fn test_expr_add() {
    let lhs = Expr::new_ref("a");
    let rhs = Expr::new_ulit_dec(8, "1");
    let add = Expr::new_add(lhs, rhs);
    let res = add.to_string();
    let exp = "a + 8'd1".to_string();
    assert_eq!(res, exp, "\n\nresult:\n{}\nexpected:\n{}\n\n", res, exp);
}

#[test]
fn test_expr_lt() {
    let lhs = Expr::new_ref("mask");
    let rhs = Expr::new_ulit_dec(32, "1");
    let lt = Expr::new_lt(lhs, rhs);
    let res = lt.to_string();
    let exp = "mask < 32'd1".to_string();
    assert_eq!(res, exp, "\n\nresult:\n{}\nexpected:\n{}\n\n", res, exp);
}

#[test]
fn test_expr_eq() {
    let lhs = Expr::new_ref("z");
    let rhs = Expr::new_ulit_dec(8, "1");
    let eq = Expr::new_eq(lhs, rhs);
    let res = eq.to_string();
    let exp = "z == 8'd1".to_string();
    assert_eq!(res, exp, "\n\nresult:\n{}\nexpected:\n{}\n\n", res, exp);
}

#[test]
fn test_expr_neq() {
    let lhs = Expr::new_ref("a");
    let rhs = Expr::new_ref("b");
    let neq = Expr::new_neq(lhs, rhs);
    let res = neq.to_string();
    let exp = "a != b".to_string();
    assert_eq!(res, exp, "\n\nresult:\n{}\nexpected:\n{}\n\n", res, exp);
}

#[test]
fn test_expr_mul() {
    let lhs = Expr::new_ref("a");
    let rhs = Expr::new_ref("b");
    let mul = Expr::new_mul(lhs, rhs);
    let res = mul.to_string();
    let exp = "a * b".to_string();
    assert_eq!(res, exp, "\n\nresult:\n{}\nexpected:\n{}\n\n", res, exp);
}

#[test]
fn test_expr_int() {
    let expr = Expr::Int(3);
    let res = expr.to_string();
    let exp = "3".to_string();
    assert_eq!(res, exp, "\n\nresult:\n{}\nexpected:\n{}\n\n", res, exp);
}

#[test]
fn test_expr_mux() {
    let cond = Expr::new_eq(Expr::new_ref("a"), Expr::new_ref("b"));
    let tru = Expr::new_ref("a");
    let fal = Expr::new_ref("b");
    let mux = Expr::new_mux(cond, tru, fal);
    let res = mux.to_string();
    let exp = "a == b ? a : b".to_string();
    assert_eq!(res, exp, "\n\nresult:\n{}\nexpected:\n{}\n\n", res, exp);
}

#[test]
fn test_expr_slice() {
    let hi = Expr::new_int(7);
    let lo = Expr::new_int(0);
    let slice = Expr::new_slice("a", hi, lo);
    let res = slice.to_string();
    let exp = "a[7:0]".to_string();
    assert_eq!(res, exp, "\n\nresult:\n{}\nexpected:\n{}\n\n", res, exp);
}

#[test]
fn test_expr_bit() {
    let bit = Expr::new_bit("a", 9);
    let res = bit.to_string();
    let exp = "a[9]".to_string();
    assert_eq!(res, exp, "\n\nresult:\n{}\nexpected:\n{}\n\n", res, exp);
}

#[test]
fn test_expr_ipath() {
    let mut ipath = IPath::default();
    ipath.add_inst("cpu");
    ipath.add_inst("alu");
    ipath.add_inst("a");
    let res = Expr::new_ipath(ipath).to_string();
    let exp = "cpu.alu.a".to_string();
    assert_eq!(res, exp, "\n\nresult:\n{}\nexpected:\n{}\n\n", res, exp);
}

#[test]
fn test_expr_return() {
    let ret = Sequential::new_return(Expr::new_ref("y"));
    let res = ret.to_string();
    let exp = "return y".to_string();
    assert_eq!(res, exp, "\n\nresult:\n{}\nexpected:\n{}\n\n", res, exp);
}

#[test]
fn test_decl_logic_width_32() {
    assert_eq!(
        "logic [31:0] foo".to_string(),
        Decl::Logic("foo".to_string(), Ty::Width(32)).to_string()
    );
}

#[test]
fn test_decl_logic_width_1() {
    assert_eq!(
        "logic foo".to_string(),
        Decl::Logic("foo".to_string(), Ty::Width(1)).to_string()
    );
}

#[test]
fn test_decl_int() {
    assert_eq!(
        "int a".to_string(),
        Decl::Int("a".to_string(), Ty::Int).to_string()
    );
}

#[test]
fn test_decl_param_uint() {
    assert_eq!(
        "parameter int width = 32'd3".to_string(),
        Decl::new_param_uint("width", 3).to_string(),
    );
}

#[test]
fn test_event_ty_posedge() {
    assert_eq!("posedge".to_string(), EventTy::Posedge.to_string(),);
}

#[test]
fn test_event_ty_negedge() {
    assert_eq!("negedge".to_string(), EventTy::Negedge.to_string(),);
}

#[test]
fn test_case_branch() {
    let mut branch = CaseBranch::new(Expr::new_ulit_dec(32, "0"));
    branch.add_stmt(Sequential::new_display("branch 0"));
    let res = branch.to_string();
    let exp = r#"32'd0 : $display("branch 0");"#;
    assert_eq!(res, exp, "\n\nresult:\n{}\nexpected:\n{}\n\n", res, exp);
}

#[test]
fn test_case_default() {
    let mut default = CaseDefault::default();
    default.add_stmt(Sequential::new_display("default branch"));
    let res = default.to_string();
    let exp = r#"default : $display("default branch");"#;
    assert_eq!(res, exp, "\n\nresult:\n{}\nexpected:\n{}\n\n", res, exp);
}

#[test]
fn test_seq_event_posedge_clock() {
    assert_eq!(
        "posedge clock".to_string(),
        Sequential::Event(EventTy::Posedge, Expr::Ref("clock".to_string())).to_string(),
    );
}

#[test]
fn test_seq_error() {
    let res = Sequential::new_error("this is an error").to_string();
    let exp = r#"$error("this is an error")"#;
    assert_eq!(res, exp, "\n\nresult:\n{}\nexpected:\n{}\n\n", res, exp);
}

#[test]
fn test_seq_display() {
    let res = Sequential::new_display("this is a message").to_string();
    let exp = r#"$display("this is a message");"#;
    assert_eq!(res, exp, "\n\nresult:\n{}\nexpected:\n{}\n\n", res, exp);
}

#[test]
fn test_seq_assert() {
    let lhs = Expr::new_ref("a");
    let rhs = Expr::new_ref("b");
    let expr = Expr::new_eq(lhs, rhs);
    let assert = Sequential::new_assert(expr);
    let res = assert.to_string();
    let exp = "assert(a == b)".to_string();
    assert_eq!(res, exp, "\n\nresult:\n{}\nexpected:\n{}\n\n", res, exp);
}

#[test]
fn test_seq_assert_with_error() {
    let lhs = Expr::new_ref("a");
    let rhs = Expr::new_ref("b");
    let expr = Expr::new_eq(lhs, rhs);
    let err = Sequential::new_error("some error");
    let assert = Sequential::new_assert_with_else(expr, err);
    let res = assert.to_string();
    let exp = r#"assert(a == b) else $error("some error")"#;
    assert_eq!(res, exp, "\n\nresult:\n{}\nexpected:\n{}\n\n", res, exp);
}

#[test]
fn test_seq_assign_blk_ref() {
    let lexpr = Expr::new_ref("a");
    let rexpr = Expr::new_ulit_bin(2, "10");
    let assign = Sequential::new_blk_assign(lexpr, rexpr);
    let res = assign.to_string();
    let exp = "a = 2'b10".to_string();
    assert_eq!(res, exp, "\n\nresult:\n{}\nexpected:\n{}\n\n", res, exp);
}

#[test]
fn test_seq_assign_non_blk_ref() {
    let lexpr = Expr::new_ref("y");
    let rexpr = Expr::new_ref("a");
    let assign = Sequential::new_non_blk_assign(lexpr, rexpr);
    let res = assign.to_string();
    let exp = "y <= a".to_string();
    assert_eq!(res, exp, "\n\nresult:\n{}\nexpected:\n{}\n\n", res, exp);
}

#[test]
fn test_port_input_width_1() {
    assert_eq!(
        "input logic foo".to_string(),
        Port::Input(Decl::Logic("foo".to_string(), Ty::Width(1))).to_string()
    );
}

#[test]
fn test_module_simple() {
    let mut module = Module::new_with_name("foo");
    module.add_input("a", 32);
    let res = module.to_string();
    let exp = r#"module foo (
    input logic [31:0] a
);
endmodule
"#;
    assert_eq!(res, exp, "\n\nresult:\n{}\nexpected:\n{}\n\n", res, exp);
}

#[test]
fn test_module_empty() {
    let exp = read_to_string("regression/v17/module_empty.v");
    let res = Module::new_with_name("empty").to_string();
    assert_eq!(exp, res);
}

#[test]
fn test_module_one_input() {
    let exp = read_to_string("regression/v17/module_one_input.v");
    let mut module = Module::new_with_name("one_input");
    module.add_input("a", 5);
    let res = module.to_string();
    assert_eq!(res, exp, "\n\nresult:\n{}\nexpected:\n{}\n\n", res, exp);
}

#[test]
fn test_module_four_inputs() {
    let exp = read_to_string("regression/v17/module_four_inputs.v");
    let mut module = Module::new_with_name("four_inputs");
    module.add_input("a", 2);
    module.add_input("b", 7);
    module.add_input("c", 4);
    module.add_input("d", 1);
    let res = module.to_string();
    assert_eq!(res, exp, "\n\nresult:\n{}\nexpected:\n{}\n\n", res, exp);
}

#[test]
fn test_module_with_instances() {
    let exp = read_to_string("regression/v17/module_with_instances.v");
    let mut i0 = Instance::new("i0", "prim");
    let mut i1 = Instance::new("i1", "prim");
    let mut i2 = Instance::new("i2", "prim");
    let e0 = Expr::new_ulit_hex(4, "0");
    let e1 = Expr::new_ulit_hex(4, "8");
    let e2 = Expr::new_ulit_hex(4, "f");
    i1.add_param_str("name", "multiply");
    i2.add_param_uint("WIDTH", 3);
    i0.connect("port_a", e0);
    i1.connect("port_a", e1);
    i2.connect("port_a", e2);
    let mut module = Module::new_with_name("module_with_instances");
    module.add_instance(i0);
    module.add_instance(i1);
    module.add_instance(i2);
    let res = module.to_string();
    assert_eq!(res, exp, "\n\nresult:\n{}\nexpected:\n{}\n\n", res, exp);
}

#[test]
fn test_module_with_function() {
    let exp = read_to_string("regression/v17/module_with_function.v");
    let lhs = Expr::new_ref("value");
    let rhs = Expr::new_ulit_hex(32, "badc0ffe");
    let expr = Expr::new_eq(lhs, rhs);
    let err = Sequential::new_error("good coffee");
    let assert = Sequential::new_assert_with_else(expr, err);
    let mut func = Function::new("check", Ty::Void);
    func.add_input("value", 32);
    func.add_stmt(assert);
    let mut module = Module::new_with_name("module_with_function");
    module.add_function(func);
    let res = module.to_string();
    assert_eq!(res, exp, "\n\nresult:\n{}\nexpected:\n{}\n\n", res, exp);
}

#[test]
fn test_module_with_function_add_one() {
    let exp = read_to_string("regression/v17/module_with_function_add_one.v");
    let var_res = Expr::new_ref("res");
    let var_val = Expr::new_ref("val");
    let con_one = Expr::new_ulit_bin(1, "1");
    let add_expr = Expr::new_add(var_val, con_one);
    let mut func = Function::new("add_one", Ty::Int);
    func.add_input("val", 32);
    func.add_logic("res", 32);
    func.add_stmt(Sequential::new_blk_assign(var_res.clone(), add_expr));
    func.add_stmt(Sequential::new_return(var_res));
    let mut module = Module::new_with_name("module_with_function_add_one");
    module.add_function(func);
    let res = module.to_string();
    assert_eq!(res, exp, "\n\nresult:\n{}\nexpected:\n{}\n\n", res, exp);
}

#[test]
fn test_module_with_always_comb() {
    let exp = read_to_string("regression/v17/module_with_always_comb.v");
    let mut always = AlwaysComb::default();
    always.add_stmt(Sequential::new_display("hello world"));
    let mut module = Module::new_with_name("module_with_always_comb");
    module.add_always_comb(always);
    let res = module.to_string();
    assert_eq!(res, exp, "\n\nresult:\n{}\nexpected:\n{}\n\n", res, exp);
}

#[test]
fn test_module_with_case() {
    let exp = read_to_string("regression/v17/module_with_case.v");
    let mut nop = CaseBranch::new(Expr::new_ulit_dec(5, "0"));
    nop.add_stmt(Sequential::new_display("nop"));
    let mut add = CaseBranch::new(Expr::new_ulit_dec(5, "1"));
    add.add_stmt(Sequential::new_display("add"));
    let mut sub = CaseBranch::new(Expr::new_ulit_dec(5, "2"));
    sub.add_stmt(Sequential::new_display("sub"));
    let mut invalid = CaseDefault::default();
    invalid.add_stmt(Sequential::new_display("invalid"));
    let mut case = Case::new(Expr::new_ref("opcode"));
    case.add_branch(nop);
    case.add_branch(add);
    case.add_branch(sub);
    case.set_default(invalid);
    let mut always = AlwaysComb::default();
    always.add_case(case);
    let mut module = Module::new_with_name("module_with_case");
    module.add_input("opcode", 5);
    module.add_always_comb(always);
    let res = module.to_string();
    assert_eq!(res, exp, "\n\nresult:\n{}\nexpected:\n{}\n\n", res, exp);
}
