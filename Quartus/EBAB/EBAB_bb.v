
module EBAB (
	clk_clk,
	reset_reset_n,
	system_pll_ref_clk_clk,
	system_pll_ref_reset_reset,
	sdram_clk_clk,
	hps_io_reset_n,
	ebab_address,
	ebab_byte_enable,
	ebab_read,
	ebab_write,
	ebab_write_data,
	ebab_acknowledge,
	ebab_read_data);	

	input		clk_clk;
	input		reset_reset_n;
	input		system_pll_ref_clk_clk;
	input		system_pll_ref_reset_reset;
	output		sdram_clk_clk;
	output		hps_io_reset_n;
	input	[11:0]	ebab_address;
	input	[1:0]	ebab_byte_enable;
	input		ebab_read;
	input		ebab_write;
	input	[15:0]	ebab_write_data;
	output		ebab_acknowledge;
	output	[15:0]	ebab_read_data;
endmodule
