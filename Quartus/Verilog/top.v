module top (i_clk, i_switches, i_buttons, o_LEDs, i_RX, o_TX);
	input        i_clk      /*synthesis chip_pin = "AF14"*/;
	input  [9:0] i_switches /*synthesis chip_pin = "AE12", "AD10", "AC9", "AE11", "AD12, AD11, AF10, AF9, AC12, AB12"*/;
	input  [3:0] i_buttons  /*synthesis chip_pin = "Y16", "W15", "AA15", "AA14"*/;
	output [9:0] o_LEDs     /*synthesis chip_pin = "Y21, W21, W20, Y19, W19, W17, V18, V17, W16, V16"*/;
	input 		 i_RX       /*synthesis chip_pin = "AJ16*/; // Pin 10
	output 		 o_TX			/*synthesis chip_pin = "AJ17*/; // Pin 9
	
	`include "parameters.vh"
	
	wire [num_outputs-1:0] outputs;
	reg [num_outputs-1:0] outputs_reg;
	reg [num_inputs-1:0] inputs;
	
	parameter CMD_send_outputs = 8'h01;
	
	//wire refresh_outputs;
	
	//clk_div #(.limit(416667), .n(19)) refresh_clk (.ar(i_buttons[0]), .clk_in(i_clk), .clk_out(refresh_outputs));
	
	wire baud;
	
	
	clk_div #(.limit(9), .n(12)) baud_clk (.ar(i_buttons[0]), .clk_in(i_clk), .clk_out(baud));
	
	reg [7:0] tx_data;
	wire tx_done;
	reg send_data;
	wire ready;
	uart_transmit #(4'd8) transmitter (.i_clk(baud), .i_rst(i_buttons[0]), .i_start(send_data), .o_ready(ready), .o_done(tx_done), .tx_data(tx_data), .tx(o_TX)); 
	
	wire [7:0] rx_data;
	wire new_data;
	uart_receive #(4'd8) receiver (.i_clk(i_clk), .i_rst(i_buttons[0]), .o_done(new_data), .rx_data(rx_data), .rx(i_RX)); 
	
	reg [7:0] bytes_count;
	
	parameter s_idle = 2'b00, s_send_bytes = 2'b01, s_wait_send = 2'b10, s_end = 2'b11;
	reg [1:0] state = s_idle;
	
	
	always @(posedge baud or negedge i_buttons[0]) begin
		if (~i_buttons[0]) begin
			send_data = 1'b0;
			state = s_idle;
			bytes_count = 8'b0;
		end
		else begin
			case (state)
				s_idle: begin
					if (new_data) begin
						case (rx_data)
							CMD_send_outputs: begin
								bytes_count = num_bytes;
								outputs_reg = outputs;
								state = s_send_bytes;
							end
						endcase
					end
				end
				s_send_bytes: begin
					if (bytes_count > 0) begin
						if (ready) begin
							tx_data = (outputs_reg >> ((num_bytes - bytes_count) << 3)) & 8'hFF;
							send_data = 1'b1;
							bytes_count = bytes_count - 8'b1;
							state = s_wait_send;
						end
					end
					else begin
						state = s_end;
					end
				end
				s_wait_send: begin
					send_data = 1'b0;
					if (tx_done) begin
						state = s_send_bytes;
					end
				end
				s_end: begin
					if (~new_data) begin
						state = s_idle;
					end
				end
				default begin
					send_data = 1'b0;
					state = s_idle;
				end
			endcase 
		end
	end
	
	
	wire tick;
	
	//clk_div #(.limit(2500000), .n(22)) tps (.ar(i_buttons[0]), .clk_in(i_clk), .clk_out(tick)); 
	
	assign o_LEDs[7:0] = rx_data;
	
	fclk fclk ( .refclk(i_clk), .rst(~i_buttons[0]), .outclk_0(tick));
	
	redstone #(num_outputs, num_inputs) redstone (.tick(tick), .inputs(i_switches), .outputs(outputs));
	
endmodule