use crate::graph::*;
use std::fs::File;
use std::io::prelude::*;



pub fn generate_verilog(graph: &Graph, path: &str)
{
    let mut input_count = 0;
    let mut output_count = 0;

    let mut verilog = "module redstone (tick, inputs, outputs);\n\tinput tick;\n\tinput [9:0] inputs;\n\toutput [9:0] outputs;\n\n".to_owned();
    for node in &graph.nodes
    {
        verilog.push_str(&format!("\twire w{};\n", node.pos.to_string()));
        match &node.block
        {
            NBlock::Input { state } =>
            {
                verilog.push_str(&format!("\tassign w{} = inputs[{input_count}];\n", node.pos.to_string()));
                input_count += 1;
            }
            NBlock::Output =>
            {
                verilog.push_str(&format!("\tassign outputs[{output_count}] = ({});\n", get_inputs_str(node, graph, LinkType::Normal)));
                output_count += 1;
            }
            NBlock::Repeater { delay, state } =>
            {
                verilog.push_str(&format!("\trepeater #({}, 1'b{}, {}, {}) c{} (.i_clk(tick), .i_in({}), .i_lock({}), .o_out(w{}));\n",
                     delay,
                     if *state {1} else {0},
                     if node.outputs.len() == 1 && node.outputs[0].ty == LinkType::Side {1} else {0},
                     if is_locking(node) {1} else {0},
                     node.pos.to_string(),
                     get_inputs_str(node, graph, LinkType::Normal),
                     get_inputs_str(node, graph, LinkType::Side),
                     node.pos.to_string()));
            }
            NBlock::Torch { state } =>
            {
                verilog.push_str(&format!("\ttorch #(1'b{}) c{} (.i_clk(tick), .i_in({}), .o_out(w{}));\n", 
                    if *state {1} else {0},
                    node.pos.to_string(),
                    get_inputs_str(node, graph, LinkType::Normal),
                    node.pos.to_string()));
            }
            NBlock::Comparator { mode, state } =>
            {
                verilog.push_str("\n");
            }
        }
    }
    verilog.push_str("endmodule");
    let mut file = File::create(path).unwrap();
    file.write_all(verilog.as_bytes());

}

pub fn get_inputs_str (node: &Node, graph: &Graph, ty: LinkType) -> String
{
    let mut inputs = "".to_owned();
    for input in &node.inputs
    {
        if input.ty == ty
        {
            inputs.push_str(&format!("w{}|",&graph.nodes[input.dest.0].pos.to_string()));
        }
        
    }
    inputs.pop();
    inputs
}

pub fn is_locking (node: &Node) -> bool
{
    for link in &node.inputs
    {
        if link.ty == LinkType::Side {return true}
    }
    false
}