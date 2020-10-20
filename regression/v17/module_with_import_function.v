module main ();
    int x;
    import "DPI-C" function void foo(input int a,
    input int b);
    always_comb begin
        foo(2, x);
    end
endmodule
