module top (i_clk, i_switches, i_buttons, o_LEDs, o_seg);
	input        i_clk      /*synthesis chip_pin = "AF14"*/;
	input  [9:0] i_switches /*synthesis chip_pin = "AE12", "AD10", "AC9", "AE11", "AD12, AD11, AF10, AF9, AC12, AB12"*/;
	input  [3:0] i_buttons  /*synthesis chip_pin = "Y16", "W15", "AA15", "AA14"*/;
	output [9:0] o_LEDs     /*synthesis chip_pin = "Y21, W21, W20, Y19, W19, W17, V18, V17, W16, V16"*/;
	output [6:0] o_seg      /*synthesis chip_pin = "AH28, AG28, AF28, AG27, AE28, AE27, AE26"*/;
	
	wire tps;
	wire tick;
	
	//clk_div #(25)   divider1 (.clk_in(i_clk), .clk_out(tps)); 
	//assign tick = (tps & i_buttons[0]) | (~i_buttons[1] & ~i_buttons[0]);

	fclk fclk ( .refclk(i_clk), .rst(~i_buttons[0]), .outclk_0(tick));
	
	//assign o_LEDs[9] = tick;

	ramtest test(.address({6'b111111,i_switches}), .clock(tick), .data(i_switches), .wren(i_buttons[1]), .q(o_LEDs));

	// repeater #(1) r1 (.i_clk(tick), .i_in(i_switches[0]), .o_out(o_LEDs[0]));
	// repeater #(2) r2 (.i_clk(tick), .i_in(i_switches[1]), .o_out(o_LEDs[1]));
	// repeater #(3) r3 (.i_clk(tick), .i_in(i_switches[2]), .o_out(o_LEDs[2]));
	// repeater #(4) r4 (.i_clk(tick), .i_in(i_switches[3]), .o_out(o_LEDs[3]));
	// torch t1 (.i_clk(tick), .i_in(i_switches[4]), .o_out(o_LEDs[4]));

	// wire w2;
	// torch t2 (.i_clk(tick), .i_in(i_switches[5]), .o_out(w2));
	// wire w3;
	// torch t3 (.i_clk(tick), .i_in(i_switches[6]), .o_out(w3));
	// wire w6;
	// torch t6 (.i_clk(tick), .i_in(w2 | w3), .o_out(w6));
	// wire w4;
	// torch t4 (.i_clk(tick), .i_in(w2 | w6), .o_out(w4));
	// wire w5;
	// torch t5 (.i_clk(tick), .i_in(w3 | w6), .o_out(w5));

	// wire w7;
	// torch t7 (.i_clk(tick), .i_in(i_switches[7]), .o_out(w7));
	// wire w8;
	// torch t8 (.i_clk(tick), .i_in(w4 | w5), .o_out(w8));
	// wire w11;
	// torch t11 (.i_clk(tick), .i_in(w7 | w8), .o_out(w11));
	// wire w9;
	// torch t9 (.i_clk(tick), .i_in(w7 | w11), .o_out(w9));
	// wire w10;
	// torch t10 (.i_clk(tick), .i_in(w8 | w11), .o_out(w10));

	// assign o_LEDs[5] = w9 | w10;
	
	//repeater #(2) (.i_clk(tps), .i_in(w1|w2), .i_lock(w3), .o_out(o_LEDs[0]));
	
	// wire	clk_1;	
	// wire clk_500m;
	
	// reg [3:0]	seg_bits;  	
	
	// fclk fclk ( .refclk(i_clk), .rst(~i_ar), .outclk_0(clk_500m));
	
	// clk_div #(28)   divider (.clk_in(clk_500m), .clk_out(clk_1)); 

	// always @(negedge i_ar or posedge clk_1)
	// if(~i_ar)
	// begin
	// 	seg_bits = 4'b0;
	// end
	// else
	// begin
	// 	seg_bits = seg_bits + 4'b1;
	// end
	
	// sevseg_dec segs (.x_in(seg_bits), .segs(o_seg));	
	
endmodule