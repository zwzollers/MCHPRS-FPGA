
use std::time::Instant;

mod items;
mod schem;
mod graph;
mod assembler;

fn main() 
{
    let mut start = Instant::now();
    let schem = schem::SchemData::from_file("./test_schems/test14_lock.schem");
    //let schem = schem::SchemData::from_file("./test_schems/c4AI1.schem");

    println!("Loading Schem Took: {:?}", start.elapsed());

    //println!("{schem:#?}");
    
    start = Instant::now();
    let attr = items::ItemAttributes::from_file("./ItemAttributes.json");

    println!("Loading Attributes Took: {:?}", start.elapsed());

    start = Instant::now();
    let r_schem = schem::RedSchem::from_schem(schem, &attr);

    println!("Parsing Schem Took: {:?}", start.elapsed());

    //println!("{r_schem:#?}");

    start = Instant::now();
    let graph = graph::Graph::from_redschem(&r_schem);

    println!("Building Graph Took: {:?}", start.elapsed());

    println!("{graph:#?}");

    start = Instant::now();
    assembler::generate_verilog(&graph, "./../Quartus/Verilog/redstone.v");

    println!("Generating Verilog Took: {:?}", start.elapsed());
}