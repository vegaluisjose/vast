module module_with_instances ();
    prim i0 (
        .port_a(4'h0)
    );
    prim # (
        .name("multiply")
    ) i1 (
        .port_a(4'h8)
    );
    prim # (
        .WIDTH(32'd3)
    ) i2 (
        .port_a(4'hf)
    );
endmodule
