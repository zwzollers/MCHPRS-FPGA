module top (i_clk, i_switches, i_buttons, o_LEDs, i_RX, o_TX);
	input        i_clk      /*synthesis chip_pin = "AF14"*/;
	input  [9:0] i_switches /*synthesis chip_pin = "AE12", "AD10", "AC9", "AE11", "AD12, AD11, AF10, AF9, AC12, AB12"*/;
	input  [3:0] i_buttons  /*synthesis chip_pin = "Y16", "W15", "AA15", "AA14"*/;
	output [9:0] o_LEDs     /*synthesis chip_pin = "Y21, W21, W20, Y19, W19, W17, V18, V17, W16, V16"*/;
	input 		 i_RX       /*synthesis chip_pin = "AJ16*/; // Pin 10
	output 		 o_TX			/*synthesis chip_pin = "AJ17*/; // Pin 9
	
	wire tps;
	wire tick;
	
	reg [511:0] inputs;
	reg [511:0] outputs;
	
	reg [7:0] tx_data = 8'hA5;
	
	uart_transmit #(4'd8) transmitter (.i_clk(), .i_rst(i_buttons[0]), .i_start(~i_buttons[2]), .tx_data(tx_data), .tx(o_TX)); 
	
	
	//clk_div #(24)   divider1 (.clk_in(i_clk), .clk_out(tps)); 
	//assign tick = (tps & i_buttons[0]) | (~i_buttons[1] & ~i_buttons[0]);

	//fclk fclk ( .refclk(i_clk), .rst(~i_buttons[0]), .outclk_0(tick));
	
	//redstone redstone (.tick(tick), .inputs(i_switches), .outputs(o_LEDs));
	
endmodule