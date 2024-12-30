module top (i_clk, i_switches, i_buttons, o_LEDs);
	input        i_clk      /*synthesis chip_pin = "AF14"*/;
	input  [9:0] i_switches /*synthesis chip_pin = "AE12", "AD10", "AC9", "AE11", "AD12, AD11, AF10, AF9, AC12, AB12"*/;
	input  [3:0] i_buttons  /*synthesis chip_pin = "Y16", "W15", "AA15", "AA14"*/;
	output [9:0] o_LEDs     /*synthesis chip_pin = "Y21, W21, W20, Y19, W19, W17, V18, V17, W16, V16"*/;
	


	EBAB u0 (
		.memory_mem_a			(HPS_DDR3_ADDR),
		.memory_mem_ba			(HPS_DDR3_BA),
		.memory_mem_ck			(HPS_DDR3_CK_P),
		.memory_mem_ck_n		(HPS_DDR3_CK_N),
		.memory_mem_cke		(HPS_DDR3_CKE),
		.memory_mem_cs_n		(HPS_DDR3_CS_N),
		.memory_mem_ras_n		(HPS_DDR3_RAS_N),
		.memory_mem_cas_n		(HPS_DDR3_CAS_N),
		.memory_mem_we_n		(HPS_DDR3_WE_N),
		.memory_mem_reset_n	(HPS_DDR3_RESET_N),
		.memory_mem_dq			(HPS_DDR3_DQ),
		.memory_mem_dqs		(HPS_DDR3_DQS_P),
		.memory_mem_dqs_n		(HPS_DDR3_DQS_N),
		.memory_mem_odt		(HPS_DDR3_ODT),
		.memory_mem_dm			(HPS_DDR3_DM),
		.memory_oct_rzqin		(HPS_DDR3_RZQ),

		.system_pll_ref_clk_clk     (CLOCK_50),
		.system_pll_ref_reset_reset (1'b0),
		.sdram_clk_clk              (DRAM_CLK),
		.hps_io_reset_n             (<connected-to-hps_io_reset_n>),
		
		.ebab_address               (ebab_address),
		.ebab_byte_enable           (ebab_byte_enable),
		.ebab_read                  (ebab_read),
		.ebab_write                 (ebab_write),
		.ebab_write_data            (ebab_write_data),
		.ebab_acknowledge           (ebab_ack),
		.ebab_read_data             (ebab_read_data)
	);
	
	wire [29:0] ebab_address;
	wire [1:0]  ebab_byte_enable;
	wire    	   ebab_read;
	wire		   ebab_write;
	wire [15:0] ebab_write_data;
	wire 		   ebab_ack;
	wire [15:0] ebab_read_data;


	
	wire tps;
	wire tick;
	
	//clk_div #(24)   divider1 (.clk_in(i_clk), .clk_out(tps)); 
	//assign tick = (tps & i_buttons[0]) | (~i_buttons[1] & ~i_buttons[0]);

	fclk fclk ( .refclk(i_clk), .rst(~i_buttons[0]), .outclk_0(tick));
	
	redstone redstone (.tick(tick), .inputs(i_switches), .outputs(o_LEDs));
	
endmodule