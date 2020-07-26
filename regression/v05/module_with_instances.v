module module_with_instances ();
    PRIM i0 ();
    PRIM # (
        .name("multiply"))
    i1 ();
    PRIM # (
        .WIDTH(32'd3))
    i2 (
        .port_a(signal_b));
endmodule