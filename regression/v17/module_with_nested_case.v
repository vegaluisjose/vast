module module_with_nested_case (
    input logic opcode,
    input logic id
);
    always_comb begin
        case (opcode)
            1'd0 : begin
                case (id)
                    1'd0 : begin
                        $display("id 0");
                    end
                    1'd1 : begin
                        $display("id 1");
                    end
                endcase
            end
            1'd1 : begin
                $display("invalid");
            end
        endcase
    end
endmodule
