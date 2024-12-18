module redstone (tick, inputs, outputs);
	input tick;
	input [9:0] inputs;
	output [9:0] outputs;

	wire w300;
	assign w300 = inputs[0];
	wire w500;
	assign w500 = inputs[1];
	wire w311;
	torch #(1'b0) c311 (.i_clk(tick), .i_in(w300), .o_out(w311));
	wire w511;
	torch #(1'b0) c511 (.i_clk(tick), .i_in(w500), .o_out(w511));
	wire w322;
	repeater #(1, 1'b1, 1, 0) c322 (.i_clk(tick), .i_in(w311), .i_lock(), .o_out(w322));
	wire w622;
	assign outputs[0] = (w623);
	wire w023;
	assign w023 = inputs[2];
	wire w223;
	torch #(1'b0) c223 (.i_clk(tick), .i_in(w023), .o_out(w223));
	wire w323;
	repeater #(1, 1'b1, 0, 1) c323 (.i_clk(tick), .i_in(w223), .i_lock(w322), .o_out(w323));
	wire w623;
	torch #(1'b1) c623 (.i_clk(tick), .i_in(w511|w323), .o_out(w623));
	wire w342;
	repeater #(1, 1'b1, 1, 0) c342 (.i_clk(tick), .i_in(w311), .i_lock(), .o_out(w342));
	wire w642;
	assign outputs[1] = (w643);
	wire w043;
	assign w043 = inputs[3];
	wire w243;
	torch #(1'b0) c243 (.i_clk(tick), .i_in(w043), .o_out(w243));
	wire w343;
	repeater #(1, 1'b1, 0, 1) c343 (.i_clk(tick), .i_in(w243), .i_lock(w342), .o_out(w343));
	wire w643;
	torch #(1'b1) c643 (.i_clk(tick), .i_in(w511|w343), .o_out(w643));
	wire w362;
	repeater #(1, 1'b1, 1, 0) c362 (.i_clk(tick), .i_in(w311), .i_lock(), .o_out(w362));
	wire w662;
	assign outputs[2] = (w663);
	wire w063;
	assign w063 = inputs[4];
	wire w263;
	torch #(1'b0) c263 (.i_clk(tick), .i_in(w063), .o_out(w263));
	wire w363;
	repeater #(1, 1'b1, 0, 1) c363 (.i_clk(tick), .i_in(w263), .i_lock(w362), .o_out(w363));
	wire w663;
	torch #(1'b1) c663 (.i_clk(tick), .i_in(w511|w363), .o_out(w663));
	wire w382;
	repeater #(1, 1'b1, 1, 0) c382 (.i_clk(tick), .i_in(w311), .i_lock(), .o_out(w382));
	wire w682;
	assign outputs[3] = (w683);
	wire w083;
	assign w083 = inputs[5];
	wire w283;
	torch #(1'b0) c283 (.i_clk(tick), .i_in(w083), .o_out(w283));
	wire w383;
	repeater #(1, 1'b1, 0, 1) c383 (.i_clk(tick), .i_in(w283), .i_lock(w382), .o_out(w383));
	wire w683;
	torch #(1'b1) c683 (.i_clk(tick), .i_in(w511|w383), .o_out(w683));
	wire w3102;
	repeater #(1, 1'b1, 1, 0) c3102 (.i_clk(tick), .i_in(w311), .i_lock(), .o_out(w3102));
	wire w6102;
	assign outputs[4] = (w6103);
	wire w0103;
	assign w0103 = inputs[6];
	wire w2103;
	torch #(1'b0) c2103 (.i_clk(tick), .i_in(w0103), .o_out(w2103));
	wire w3103;
	repeater #(1, 1'b1, 0, 1) c3103 (.i_clk(tick), .i_in(w2103), .i_lock(w3102), .o_out(w3103));
	wire w6103;
	torch #(1'b1) c6103 (.i_clk(tick), .i_in(w511|w3103), .o_out(w6103));
	wire w3122;
	repeater #(1, 1'b1, 1, 0) c3122 (.i_clk(tick), .i_in(w311), .i_lock(), .o_out(w3122));
	wire w6122;
	assign outputs[5] = (w6123);
	wire w0123;
	assign w0123 = inputs[7];
	wire w2123;
	torch #(1'b0) c2123 (.i_clk(tick), .i_in(w0123), .o_out(w2123));
	wire w3123;
	repeater #(1, 1'b1, 0, 1) c3123 (.i_clk(tick), .i_in(w2123), .i_lock(w3122), .o_out(w3123));
	wire w6123;
	torch #(1'b1) c6123 (.i_clk(tick), .i_in(w511|w3123), .o_out(w6123));
	wire w3142;
	repeater #(1, 1'b1, 1, 0) c3142 (.i_clk(tick), .i_in(w311), .i_lock(), .o_out(w3142));
	wire w6142;
	assign outputs[6] = (w6143);
	wire w0143;
	assign w0143 = inputs[8];
	wire w2143;
	torch #(1'b0) c2143 (.i_clk(tick), .i_in(w0143), .o_out(w2143));
	wire w3143;
	repeater #(1, 1'b1, 0, 1) c3143 (.i_clk(tick), .i_in(w2143), .i_lock(w3142), .o_out(w3143));
	wire w6143;
	torch #(1'b1) c6143 (.i_clk(tick), .i_in(w511|w3143), .o_out(w6143));
	wire w3162;
	repeater #(1, 1'b1, 1, 0) c3162 (.i_clk(tick), .i_in(w311), .i_lock(), .o_out(w3162));
	wire w6162;
	assign outputs[7] = (w6163);
	wire w0163;
	assign w0163 = inputs[9];
	wire w2163;
	torch #(1'b0) c2163 (.i_clk(tick), .i_in(w0163), .o_out(w2163));
	wire w3163;
	repeater #(1, 1'b1, 0, 1) c3163 (.i_clk(tick), .i_in(w2163), .i_lock(w3162), .o_out(w3163));
	wire w6163;
	torch #(1'b1) c6163 (.i_clk(tick), .i_in(w511|w3163), .o_out(w6163));
endmodule