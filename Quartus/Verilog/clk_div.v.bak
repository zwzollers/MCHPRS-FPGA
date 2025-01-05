
module    clk_div(clk_in, clk_out);
    input clk_in;         // 50 MHz for audio codec on DE2 board
    output clk_out;       // 
    
    //reg   clk_out;
    parameter n = 15;      // Bit width of counter and limit (log2 of limit)
    reg [n-1:0]   count;
	
	assign clk_out = count[n-1];
    
    always @(posedge clk_in)
		count = count + 1;
                                                                                                                                                                                                              
 endmodule       
          
                 
    
   