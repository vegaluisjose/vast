module module_with_function_add_one ();
    function int add_one;
        input logic [31:0] val;
        logic [31:0] res;
        begin
            res = val + 1'b1;
            return res;
        end
    endfunction
endmodule
