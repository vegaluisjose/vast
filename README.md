# Verilog AST (VAST)

VAST is a Rust library for building and manipulating Verilog AST. The goal is to support features from two different versions of the standard 2005 and 2017, see [v05](https://github.com/vegaluisjose/vast/tree/master/src/v05) and [v17](https://github.com/vegaluisjose/vast/tree/master/src/v17) respectively. The [subset](https://github.com/vegaluisjose/vast/tree/master/src/subset) directory contains nodes that are common between the two.

## Using VAST

Add `vast` to your `Cargo.toml` like this:
```toml
[dependencies]
vast = "0.1.0"
```
