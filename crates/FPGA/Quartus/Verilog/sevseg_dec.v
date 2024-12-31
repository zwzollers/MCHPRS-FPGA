
// Seven-segment decoder for hexadecimal values
// seven_seg_decoder.v
// Don M. Gruenbacher
// Jan. 23, 2006

module sevseg_dec(x_in, segs);
    input [3:0]   x_in;
	output [6:0] segs;

assign segs =
		(x_in == 4'h0) ? 7'b1000000 :
		(x_in == 4'h1) ? 7'b1111001 :
		(x_in == 4'h2) ? 7'b0100100 :
		(x_in == 4'h3) ? 7'b0110000 :
		(x_in == 4'h4) ? 7'b0011001 :
		(x_in == 4'h5) ? 7'b0010010 :
		(x_in == 4'h6) ? 7'b0000010 :  
		(x_in == 4'h7) ? 7'b1111000 :
		(x_in == 4'h8) ? 7'b0000000 :
		(x_in == 4'h9) ? 7'b0010000 :
		(x_in == 4'ha) ? 7'b0001000 :
		(x_in == 4'hb) ? 7'b0000011 :
		(x_in == 4'hc) ? 7'b1000110 :
		(x_in == 4'hd) ? 7'b0100001 :
		(x_in == 4'he) ? 7'b0000110 :
						7'b0001110 ;
		  
endmodule
