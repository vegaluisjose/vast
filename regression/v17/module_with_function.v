module module_with_function ();
    
    function void check;
        input logic [31:0] value;
        begin
        assert(value == 32'hbadc0ffe) else $error("good coffee");
        end
    endfunction
    
endmodule
