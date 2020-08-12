module module_with_case (
    input logic [4:0] opcode
);
    always_comb begin
        case (opcode)
            5'd0 : $display("nop");
            5'd1 : $display("add");
            5'd2 : $display("sub");
            default : $display("invalid");
        endcase
    end
endmodule
