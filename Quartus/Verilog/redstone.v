module redstone (tick, inputs, outputs);
	input tick;
	input [9:0] inputs;
	output [9:0] outputs;

	wire w011;
	repeater #(1, 1'b0, 0, 1) c011 (.i_clk(tick), .i_in(w311), .i_lock(w111), .o_out(w011));
	wire w111;
	repeater #(2, 1'b0, 1, 0) c111 (.i_clk(tick), .i_in(w311), .i_lock(), .o_out(w111));
	wire w311;
	assign w311 = inputs[0];
	wire w411;
	repeater #(1, 1'b0, 0, 1) c411 (.i_clk(tick), .i_in(w711), .i_lock(w511), .o_out(w411));
	wire w511;
	repeater #(1, 1'b0, 1, 0) c511 (.i_clk(tick), .i_in(w711), .i_lock(), .o_out(w511));
	wire w711;
	assign w711 = inputs[1];
	wire w012;
	assign outputs[0] = (w011);
	wire w412;
	assign outputs[1] = (w411);
endmodule