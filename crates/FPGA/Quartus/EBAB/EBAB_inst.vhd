	component EBAB is
		port (
			clk_clk                    : in  std_logic                     := 'X';             -- clk
			reset_reset_n              : in  std_logic                     := 'X';             -- reset_n
			system_pll_ref_clk_clk     : in  std_logic                     := 'X';             -- clk
			system_pll_ref_reset_reset : in  std_logic                     := 'X';             -- reset
			sdram_clk_clk              : out std_logic;                                        -- clk
			hps_io_reset_n             : out std_logic;                                        -- reset_n
			ebab_address               : in  std_logic_vector(11 downto 0) := (others => 'X'); -- address
			ebab_byte_enable           : in  std_logic_vector(1 downto 0)  := (others => 'X'); -- byte_enable
			ebab_read                  : in  std_logic                     := 'X';             -- read
			ebab_write                 : in  std_logic                     := 'X';             -- write
			ebab_write_data            : in  std_logic_vector(15 downto 0) := (others => 'X'); -- write_data
			ebab_acknowledge           : out std_logic;                                        -- acknowledge
			ebab_read_data             : out std_logic_vector(15 downto 0)                     -- read_data
		);
	end component EBAB;

	u0 : component EBAB
		port map (
			clk_clk                    => CONNECTED_TO_clk_clk,                    --                  clk.clk
			reset_reset_n              => CONNECTED_TO_reset_reset_n,              --                reset.reset_n
			system_pll_ref_clk_clk     => CONNECTED_TO_system_pll_ref_clk_clk,     --   system_pll_ref_clk.clk
			system_pll_ref_reset_reset => CONNECTED_TO_system_pll_ref_reset_reset, -- system_pll_ref_reset.reset
			sdram_clk_clk              => CONNECTED_TO_sdram_clk_clk,              --            sdram_clk.clk
			hps_io_reset_n             => CONNECTED_TO_hps_io_reset_n,             --               hps_io.reset_n
			ebab_address               => CONNECTED_TO_ebab_address,               --                 ebab.address
			ebab_byte_enable           => CONNECTED_TO_ebab_byte_enable,           --                     .byte_enable
			ebab_read                  => CONNECTED_TO_ebab_read,                  --                     .read
			ebab_write                 => CONNECTED_TO_ebab_write,                 --                     .write
			ebab_write_data            => CONNECTED_TO_ebab_write_data,            --                     .write_data
			ebab_acknowledge           => CONNECTED_TO_ebab_acknowledge,           --                     .acknowledge
			ebab_read_data             => CONNECTED_TO_ebab_read_data              --                     .read_data
		);

