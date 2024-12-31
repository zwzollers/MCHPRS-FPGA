	EBAB u0 (
		.clk_clk                    (<connected-to-clk_clk>),                    //                  clk.clk
		.reset_reset_n              (<connected-to-reset_reset_n>),              //                reset.reset_n
		.system_pll_ref_clk_clk     (<connected-to-system_pll_ref_clk_clk>),     //   system_pll_ref_clk.clk
		.system_pll_ref_reset_reset (<connected-to-system_pll_ref_reset_reset>), // system_pll_ref_reset.reset
		.sdram_clk_clk              (<connected-to-sdram_clk_clk>),              //            sdram_clk.clk
		.hps_io_reset_n             (<connected-to-hps_io_reset_n>),             //               hps_io.reset_n
		.ebab_address               (<connected-to-ebab_address>),               //                 ebab.address
		.ebab_byte_enable           (<connected-to-ebab_byte_enable>),           //                     .byte_enable
		.ebab_read                  (<connected-to-ebab_read>),                  //                     .read
		.ebab_write                 (<connected-to-ebab_write>),                 //                     .write
		.ebab_write_data            (<connected-to-ebab_write_data>),            //                     .write_data
		.ebab_acknowledge           (<connected-to-ebab_acknowledge>),           //                     .acknowledge
		.ebab_read_data             (<connected-to-ebab_read_data>)              //                     .read_data
	);

