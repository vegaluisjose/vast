module module_with_always_ff (
    input logic clock
);
    always_ff @(posedge clock) begin
        $display("hello sync world");
    end
endmodule
