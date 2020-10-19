module module_with_alwaysff ();
    alwaysff @(posedge clock) begin
        $display("hello sync world");
    end
endmodule
