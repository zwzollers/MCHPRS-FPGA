module top (i_clk, i_ar, i_switches, o_seg);
	input        i_clk      /*synthesis chip_pin = "AF14"*/; 					                     // 50 MHz clock
	input        i_ar       /*synthesis chip_pin = "AA14"*/;					                     // asynchronous reset (active low)
	input  [5:0] i_switches /*synthesis chip_pin = "AD12, AD11, AF10, AF9, AC12, AB12"*/; 		 // Input word for dip switches
	output [6:0] o_seg     /*synthesis chip_pin = "AH28, AG28, AF28, AG27, AE28, AE27, AE26"*/;
	
	wire		 clk_1;	
	wire 		 clk_500m;
	
	reg [3:0]	seg_bits;  	
	
	fclk fclk ( .refclk(i_clk), .rst(~i_ar), .outclk_0(clk_500m));
	
	clk_div #(28)   divider (.clk_in(clk_500m), .clk_out(clk_1)); 

	always @(negedge i_ar or posedge clk_1)
	if(~i_ar)
	begin
		seg_bits = 4'b0;
	end
	else
	begin
		seg_bits = seg_bits + 1;
	end
	
	sevseg_dec segs (.x_in(seg_bits), .segs(o_seg));	
	
endmodule