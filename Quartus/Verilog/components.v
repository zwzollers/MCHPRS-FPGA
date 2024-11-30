module repeater (i_clk, i_in, o_out);

	input  i_clk;
	input  i_in;
	output o_out;

	parameter t = 1,
				 state = 1'b0;

	reg [t-1:0] buffer = {t{state}};

	assign o_out = buffer[t-1];
	
	generate
		if (t == 1)
			always @(posedge i_clk) begin
				buffer = i_in;
			end
		else
			always @(posedge i_clk) begin
				buffer = {buffer[t-2:0] | {t-1{buffer[t-1] & i_in}}, i_in | (~buffer[t-1] & buffer[0])};
			end
		// if (t == 1)
		// 	always @(posedge i_clk) begin
		// 		l_out = l_in;
		// 	end
		// else
		// 	always @(posedge i_clk) begin
		// 		{l_out, buffer} = {h_out, (buffer[t-2:0] | l_in) & {t-1{~buffer[t-1]}}, (buffer[t-1] & ~h_out) | h_in };
		// 	end
		// if (t == 1)
		// 	always @(posedge i_clk) begin
		// 		o_out = i_in;
		// 	end
		// else if (t == 2)
		// 	always @(posedge i_clk) begin
		// 		{o_out, buffer} = {
		// 			(o_out & i_in) | (o_out ^ buffer[1]),
		// 			(buffer[0] | (i_in ^ o_out)) & ~buffer[1],
		// 			buffer[1] & ~o_out};
		// 	end
		// else if (t == 3)
		// 	always @(posedge i_clk) begin
		// 		{o_out, buffer} = {
		// 			(o_out & i_in) | (o_out ^ buffer[2]),
		// 			buffer[1] & ~buffer[3],
		// 			(buffer[0] | (i_in ^ o_out)) & ~buffer[2],
		// 			buffer[2] & ~o_out};
		// 	end
		// else if (t == 4)
		// 	always @(posedge i_clk) begin
		// 		{o_out, buffer} = {
		// 			(o_out & i_in) | (o_out ^ buffer[3]),
		// 			buffer[2] & ~buffer[3],
		// 			buffer[1] & ~buffer[3],
		// 			(buffer[0] | (i_in ^ o_out)) & ~buffer[3],
		// 			buffer[3] & ~o_out};
		// 	end
	endgenerate
	
endmodule

module torch (i_clk, i_in, o_out);

	input  i_clk;
	input  i_in;
	output o_out;
	
	parameter state = 1'b0;

	reg buffer = state;

	assign o_out = ~buffer;

	always @(posedge i_clk) begin
		buffer = i_in;
	end

endmodule