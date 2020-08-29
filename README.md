# Verilog AST (VAST)


[![Build Status](https://github.com/vegaluisjose/vast/workflows/Build%20and%20Test/badge.svg?branch=master)](https://github.com/vegaluisjose/vast/actions)
[![Crates.io](https://img.shields.io/crates/v/vast.svg)](https://crates.io/crates/vast)

VAST is a Rust library for building and manipulating Verilog ASTs. The goal is to support features from two different versions of the standard 2005 and 2017, [v05](https://github.com/vegaluisjose/vast/tree/master/src/v05) and [v17](https://github.com/vegaluisjose/vast/tree/master/src/v17) respectively. The [subset](https://github.com/vegaluisjose/vast/tree/master/src/subset) directory contains types that are common between the two.

## Using VAST

Add `vast` to your `Cargo.toml` like this:
```toml
[dependencies]
vast = "0.1.0"
```

## Creating a module in Verilog-2005

```rust
use vast::v05::ast::Module;

fn main() {
    let mut module = Module::new("foo");
    module.add_input("a", 32);
    let res = module.to_string();
    let exp = r#"module foo (
    input wire [31:0] a
);
endmodule
"#;
    assert_eq!(res, exp);
}
```

## Creating a module in SystemVerilog-2017

```rust
use vast::v17::ast::Module;

fn main() {
    let mut module = Module::new("foo");
    module.add_input("a", 32);
    let res = module.to_string();
    let exp = r#"module foo (
    input logic [31:0] a
);
endmodule
"#;
    assert_eq!(res, exp);
}
```
