# Copyright (C) 2024  Intel Corporation. All rights reserved.
# Your use of Intel Corporation's design tools, logic functions 
# and other software and tools, and any partner logic 
# functions, and any output files from any of the foregoing 
# (including device programming or simulation files), and any 
# associated documentation or information are expressly subject 
# to the terms and conditions of the Intel Program License 
# Subscription Agreement, the Intel Quartus Prime License Agreement,
# the Intel FPGA IP License Agreement, or other applicable license
# agreement, including, without limitation, that your use is for
# the sole purpose of programming logic devices manufactured by
# Intel and sold by Intel or its authorized distributors.  Please
# refer to the applicable agreement for further details, at
# https://fpgasoftware.intel.com/eula.

# Quartus Prime: Generate Tcl File for Project
# File: FPGA-MCHPRS.tcl
# Generated on: Sat Dec 28 00:18:22 2024

# Load Quartus Prime Tcl Project package
package require ::quartus::project

set need_to_close_project 0
set make_assignments 1

# Check that the right project is open
if {[is_project_open]} {
	if {[string compare $quartus(project) "FPGA-MCHPRS"]} {
		puts "Project FPGA-MCHPRS is not open"
		set make_assignments 0
	}
} else {
	# Only open if not already open
	if {[project_exists FPGA-MCHPRS]} {
		project_open -revision FPGA-MCHPRS FPGA-MCHPRS
	} else {
		project_new -revision FPGA-MCHPRS FPGA-MCHPRS
	}
	set need_to_close_project 1
}

# Make assignments
if {$make_assignments} {
	set_global_assignment -name FAMILY "Cyclone V"
	set_global_assignment -name DEVICE 5CSEMA5F31C6
	set_global_assignment -name TOP_LEVEL_ENTITY top
	set_global_assignment -name ORIGINAL_QUARTUS_VERSION 23.1STD.1
	set_global_assignment -name PROJECT_CREATION_TIME_DATE "17:45:12  NOVEMBER 06, 2024"
	set_global_assignment -name LAST_QUARTUS_VERSION "23.1std.1 Lite Edition"
	set_global_assignment -name MIN_CORE_JUNCTION_TEMP 0
	set_global_assignment -name MAX_CORE_JUNCTION_TEMP 85
	set_global_assignment -name POWER_PRESET_COOLING_SOLUTION "23 MM HEAT SINK WITH 200 LFPM AIRFLOW"
	set_global_assignment -name POWER_BOARD_THERMAL_MODEL "NONE (CONSERVATIVE)"
	set_global_assignment -name PARTITION_NETLIST_TYPE SOURCE -section_id Top
	set_global_assignment -name PARTITION_FITTER_PRESERVATION_LEVEL PLACEMENT_AND_ROUTING -section_id Top
	set_global_assignment -name PARTITION_COLOR 16764057 -section_id Top
	set_global_assignment -name OPTIMIZATION_TECHNIQUE AREA
	set_global_assignment -name PHYSICAL_SYNTHESIS_COMBO_LOGIC ON
	set_global_assignment -name ROUTER_LCELL_INSERTION_AND_LOGIC_DUPLICATION ON
	set_global_assignment -name ROUTER_TIMING_OPTIMIZATION_LEVEL MAXIMUM
	set_global_assignment -name SOURCE_FILE fclk.cmp
	set_global_assignment -name VERILOG_FILE Verilog/redstone.v
	set_global_assignment -name VERILOG_FILE Verilog/components.v
	set_global_assignment -name VERILOG_FILE Verilog/sevseg_dec.v
	set_global_assignment -name VERILOG_FILE Verilog/top.v
	set_global_assignment -name VERILOG_FILE Verilog/clk_div.v
	set_global_assignment -name QIP_FILE fclk.qip
	set_global_assignment -name SIP_FILE fclk.sip
	set_global_assignment -name QIP_FILE ramtest.qip
	set_global_assignment -name ENABLE_SIGNALTAP ON
	set_global_assignment -name USE_SIGNALTAP_FILE stp1.stp
	set_global_assignment -name SIGNALTAP_FILE stp1.stp
	set_global_assignment -name SLD_FILE db/stp1_auto_stripped.stp
	set_instance_assignment -name GLOBAL_SIGNAL GLOBAL_CLOCK -to "fclk:fclk|outclk_0"
	set_location_assignment PIN_Y16 -to i_buttons[3]
	set_location_assignment PIN_W15 -to i_buttons[2]
	set_location_assignment PIN_AA15 -to i_buttons[1]
	set_location_assignment PIN_AA14 -to i_buttons[0]
	set_location_assignment PIN_AE12 -to i_switches[9]
	set_location_assignment PIN_AD10 -to i_switches[8]
	set_location_assignment PIN_AC9 -to i_switches[7]
	set_location_assignment PIN_AE11 -to i_switches[6]
	set_location_assignment PIN_AD12 -to i_switches[5]
	set_location_assignment PIN_AD11 -to i_switches[4]
	set_location_assignment PIN_AB12 -to i_switches[0]
	set_location_assignment PIN_AC12 -to i_switches[1]
	set_location_assignment PIN_AF9 -to i_switches[2]
	set_location_assignment PIN_AF10 -to i_switches[3]
	set_instance_assignment -name PARTITION_HIERARCHY root_partition -to | -section_id Top

	# Commit assignments
	export_assignments

	# Close project
	if {$need_to_close_project} {
		project_close
	}
}
