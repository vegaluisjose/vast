module module_with_case (
    input logic [4:0] opcode
);
    always_comb begin
        case (opcode)
            5'd0 : begin
                $display("nop");
            end
            5'd1 : begin
                $display("add");
            end
            5'd2 : begin
                $display("sub");
            end
            default : begin
                $display("invalid");
            end
        endcase
    end
endmodule
