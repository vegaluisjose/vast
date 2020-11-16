use vast::util::file::read_to_string;
use vast::v05::ast::*;

macro_rules! check {
    ($res:expr, $exp:expr) => {
        assert!(
            $res == $exp,
            "\n\nresult:\n{}\nexpected:\n{}\n\n",
            $res,
            $exp
        );
    };
}

#[test]
fn test_expr_str() {
    let expr = Expr::new_str("multiply");
    let exp = r#""multiply""#.to_string();
    let res = expr.to_string();
    check!(res, exp);
}

#[test]
fn test_expr_ulit_bin() {
    let expr = Expr::new_ulit_bin(4, "1000");
    let exp = "4'b1000".to_string();
    let res = expr.to_string();
    check!(res, exp);
}

#[test]
fn test_expr_ulit_hex() {
    let expr = Expr::new_ulit_hex(8, "ff");
    let exp = "8'hff".to_string();
    let res = expr.to_string();
    check!(res, exp);
}

#[test]
fn test_expr_ulit_dec() {
    let expr = Expr::new_ulit_dec(16, "78");
    let exp = "16'd78".to_string();
    let res = expr.to_string();
    check!(res, exp);
}

#[test]
fn test_expr_ref() {
    let expr = Expr::new_ref("a");
    let exp = "a".to_string();
    let res = expr.to_string();
    check!(res, exp);
}

#[test]
fn test_expr_signed() {
    let expr = Expr::new_signed_ref("a");
    let exp = "$signed(a)".to_string();
    let res = expr.to_string();
    check!(res, exp);
}

#[test]
fn test_expr_concat() {
    let mut concat = ExprConcat::default();
    concat.add_expr(Expr::new_ref("a"));
    concat.add_expr(Expr::new_ulit_bin(1, "0"));
    let expr = Expr::from(concat);
    let exp = "{1'b0, a}".to_string();
    let res = expr.to_string();
    check!(res, exp);
}

#[test]
fn test_expr_mux() {
    let cond = Expr::new_eq(Expr::new_ref("a"), Expr::new_ref("b"));
    let tru = Expr::new_ref("a");
    let fal = Expr::new_ref("b");
    let mux = Expr::new_mux(cond, tru, fal);
    let res = mux.to_string();
    let exp = "(a == b) ? a : b".to_string();
    check!(res, exp);
}

#[test]
fn test_expr_slice() {
    let hi = Expr::new_int(7);
    let lo = Expr::new_int(0);
    let slice = Expr::new_slice("a", hi, lo);
    let res = slice.to_string();
    let exp = "a[7:0]".to_string();
    check!(res, exp);
}

#[test]
fn test_expr_index_slice() {
    let lo = Expr::new_int(0);
    let slice = Expr::new_index_slice("a", lo, 8);
    let res = slice.to_string();
    let exp = "a[0 +: 8]".to_string();
    check!(res, exp);
}

#[test]
fn test_attr_val() {
    let mut attr = Attribute::default();
    attr.add_val("full_case");
    let exp = "(*full_case*)".to_string();
    let res = attr.to_string();
    check!(res, exp);
}

#[test]
fn test_attr_stmt() {
    let mut attr = Attribute::default();
    attr.add_stmt("x", "3");
    let exp = "(*x = \"3\"*)".to_string();
    let res = attr.to_string();
    check!(res, exp);
}

#[test]
fn test_decl_wire_width_32() {
    let wire = Decl::new_wire("foo", 32);
    let exp = "wire [31:0] foo".to_string();
    let res = wire.to_string();
    check!(res, exp);
}

#[test]
fn test_decl_wire_width_1() {
    let wire = Decl::new_wire("foo", 1);
    let exp = "wire foo".to_string();
    let res = wire.to_string();
    check!(res, exp);
}

#[test]
fn test_decl_reg_width_32() {
    let reg = Decl::new_reg("foo", 32);
    let exp = "reg [31:0] foo".to_string();
    let res = reg.to_string();
    check!(res, exp);
}

#[test]
fn test_decl_reg_width_1() {
    let reg = Decl::new_reg("foo", 1);
    let exp = "reg foo".to_string();
    let res = reg.to_string();
    check!(res, exp);
}

#[test]
fn test_decl_param_uint() {
    let param = Decl::new_param_uint("width", 3);
    let exp = "parameter width = 32'd3".to_string();
    let res = param.to_string();
    check!(res, exp);
}

#[test]
fn test_event_ty_posedge() {
    let event = EventTy::Posedge;
    let exp = "posedge".to_string();
    let res = event.to_string();
    check!(res, exp);
}

#[test]
fn test_event_ty_negedge() {
    let event = EventTy::Negedge;
    let exp = "negedge".to_string();
    let res = event.to_string();
    check!(res, exp);
}

#[test]
fn test_sequential_event_posedge_clock() {
    let seq = Sequential::Event(EventTy::Posedge, Expr::Ref("clock".to_string()));
    let exp = "posedge clock".to_string();
    let res = seq.to_string();
    check!(res, exp);
}

#[test]
fn test_sequential_if() {
    let cond = Expr::new_ref("reset");
    let y = Expr::new_ref("y");
    let a = Expr::new_ref("a");
    let seq = Sequential::new_nonblk_assign(y, a);
    let mut ifelse = SequentialIfElse::new(cond);
    ifelse.add_seq(seq);
    let exp = r#"if(reset) begin
    y <= a;
end"#;
    let res = ifelse.to_string();
    check!(res, exp);
}

#[test]
fn test_sequential_if_else() {
    let c0 = Expr::new_ref("reset");
    let y = Expr::new_ref("y");
    let a = Expr::new_ref("a");
    let val = Expr::new_int(0);
    let s0 = Sequential::new_nonblk_assign(y.clone(), val);
    let s1 = Sequential::new_nonblk_assign(y, a);
    let mut i0 = SequentialIfElse::new(c0);
    let mut i1 = SequentialIfElse::default();
    i0.add_seq(s0);
    i1.add_seq(s1);
    i0.set_else(i1.into());
    let exp = r#"if(reset) begin
    y <= 0;
end else begin
    y <= a;
end"#;
    let res = i0.to_string();
    check!(res, exp);
}

#[test]
fn test_sequential_if_else_if() {
    let c0 = Expr::new_ref("reset");
    let c1 = Expr::new_ref("en");
    let y = Expr::new_ref("y");
    let a = Expr::new_ref("a");
    let val = Expr::new_int(0);
    let s0 = Sequential::new_nonblk_assign(y.clone(), val);
    let s1 = Sequential::new_nonblk_assign(y, a);
    let mut i0 = SequentialIfElse::new(c0);
    let mut i1 = SequentialIfElse::new(c1);
    i0.add_seq(s0);
    i1.add_seq(s1);
    i0.set_else(i1.into());
    let exp = r#"if(reset) begin
    y <= 0;
end else if(en) begin
    y <= a;
end"#;
    let res = i0.to_string();
    check!(res, exp);
}

#[test]
fn test_parallel_assign() {
    let val = Expr::new_ulit_dec(32, "3");
    let var = Expr::new_ref("a");
    let par = Parallel::Assign(var, val);
    let exp = "assign a = 32'd3;".to_string();
    let res = par.to_string();
    check!(res, exp);
}

#[test]
fn test_parallel_always() {
    let event = Sequential::new_posedge("clock");
    let y = Expr::new_ref("y");
    let a = Expr::new_ref("a");
    let seq = Sequential::new_nonblk_assign(y, a);
    let mut always = ParallelAlways::new(event);
    always.add_seq(seq);
    let exp = r#"always @(posedge clock) begin
    y <= a;
end"#;
    let res = always.to_string();
    check!(res, exp);
}

#[test]
fn test_module_simple() {
    let mut module = Module::new("foo");
    module.add_input("a", 32);
    let res = module.to_string();
    let exp = r#"module foo (
    input wire [31:0] a
);
endmodule
"#;
    check!(res, exp);
}

#[test]
fn test_module_empty() {
    let exp = read_to_string("regression/v05/module_empty.v");
    let res = Module::new("empty").to_string();
    check!(res, exp);
}

#[test]
fn test_module_attribute() {
    let exp = read_to_string("regression/v05/module_attribute.v");
    let mut attr = Attribute::default();
    attr.add_stmt("use", "yes");
    let mut module = Module::new("attribute");
    module.set_attr(attr);
    let res = module.to_string();
    check!(res, exp);
}

#[test]
fn test_module_one_input() {
    let exp = read_to_string("regression/v05/module_one_input.v");
    let mut module = Module::new("one_input");
    module.add_input("a", 5);
    let res = module.to_string();
    check!(res, exp);
}

#[test]
fn test_module_three_inputs() {
    let exp = read_to_string("regression/v05/module_three_inputs.v");
    let mut module = Module::new("three_inputs");
    module.add_input("a", 5);
    module.add_input("b", 61);
    module.add_input("c", 1);
    let res = module.to_string();
    check!(res, exp);
}

#[test]
fn test_module_one_wire() {
    let exp = read_to_string("regression/v05/module_one_wire.v");
    let mut module = Module::new("one_wire");
    let wire = Decl::new_wire("one_wire", 8);
    module.add_stmt(Stmt::from(wire));
    let res = module.to_string();
    check!(res, exp);
}

#[test]
fn test_module_one_param() {
    let exp = read_to_string("regression/v05/module_one_param.v");
    let mut module = Module::new("one_param");
    module.add_param_uint("width", 32);
    module.add_input("data", 4);
    let res = module.to_string();
    check!(res, exp);
}

#[test]
fn test_module_two_params() {
    let exp = read_to_string("regression/v05/module_two_params.v");
    let mut module = Module::new("two_params");
    module.add_param_uint("width", 4);
    module.add_param_uint("length", 8);
    module.add_input("data", 4);
    let res = module.to_string();
    check!(res, exp);
}

#[test]
fn test_module_mix_params() {
    let exp = read_to_string("regression/v05/module_mix_params.v");
    let mut module = Module::new("mix_params");
    module.add_param_uint("width", 4);
    module.add_param_uint("length", 8);
    module.add_param_str("name", "foo");
    module.add_input("data", 4);
    let res = module.to_string();
    check!(res, exp);
}

#[test]
fn test_module_with_instances() {
    let exp = read_to_string("regression/v05/module_with_instances.v");
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
    let mut module = Module::new("module_with_instances");
    module.add_instance(i0);
    module.add_instance(i1);
    module.add_instance(i2);
    let res = module.to_string();
    check!(res, exp);
}

#[test]
fn test_module_with_instance_attribute() {
    let exp = read_to_string("regression/v05/module_with_instance_attribute.v");
    let mut attr = Attribute::default();
    attr.add_stmt("TYPE", "LUT6");
    attr.add_stmt("LOC", "X0Y0");
    let mut i0 = Instance::new("i0", "prim");
    let e0 = Expr::new_ulit_hex(4, "0");
    i0.connect("port_a", e0);
    i0.set_attr(attr);
    let mut module = Module::new("module_with_instance_attribute");
    module.add_instance(i0);
    let res = module.to_string();
    check!(res, exp);
}
