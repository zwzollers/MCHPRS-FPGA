use crate::graph::*;
use std::fs::File;
use std::io::prelude::*;



pub fn generate_verilog(graph: &Graph, filename: &str)
{
    let mut input_count = 0;
    let mut output_count = 0;

    let mut verilog = "module redstone (tick, inputs, outputs);\ninput tick;\ninput [9:0] inputs;\noutput [9:0] outputs;\n".to_owned();
    for node in &graph.nodes
    {
        verilog.push_str(&format!("wire w{};\n", node.pos.to_string()));
        match &node.block
        {
            NBlock::Input { state } =>
            {
                verilog.push_str(&format!("assign w{} = inputs[{input_count}];\n", node.pos.to_string()));
                input_count += 1;
            }
            NBlock::Output =>
            {
                verilog.push_str(&format!("assign outputs[{output_count}] = ({});\n", get_inputs_str(node, graph)));
                output_count += 1;
            }
            NBlock::Repeater { delay, state } =>
            {
                verilog.push_str(&format!("repeater #(t = {}, state = 1'b{}) c{} (.i_clk(tick), .i_in({}), .o_out(w{}));\n", delay, if *state {1} else {0}, node.pos.to_string(), get_inputs_str(node, graph), node.pos.to_string()));
            }
            NBlock::Torch { state } =>
            {
                verilog.push_str(&format!("torch #(1'b{}) c{} (.i_clk(tick), .i_in({}), .o_out(w{}));\n", if *state {1} else {0}, node.pos.to_string(), get_inputs_str(node, graph), node.pos.to_string()));
            }
            NBlock::Comparator { mode, state } =>
            {
                verilog.push_str("\n");
            }
        }
    }
    verilog.push_str("endmodule");
    let mut file = File::create("/home/zwzollers/Documents/GIT/MCHPRS-FPGA/Quartus/Verilog/redstone.v").unwrap();
    file.write_all(verilog.as_bytes());

}

pub fn get_inputs_str (node: &Node, graph: &Graph) -> String
{
    let mut str = "".to_owned();
    for input in &node.inputs
    {
        str.push_str(&format!("w{}|",&graph.nodes[input.dest.0].pos.to_string()));
    }
    str.pop();
    str
}