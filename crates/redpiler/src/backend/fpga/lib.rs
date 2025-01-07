
use std::time::Instant;
use std::fs::File;
use std::io::prelude::*;
use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader, Error, ErrorKind};
use mchprs_blocks::BlockPos;

mod items;
mod schem;
mod graph;
mod assembler;
mod LUT;

pub fn convert_compile(file: String, outputs: &mut Vec<BlockPos>, inputs: &mut Vec<BlockPos>) 
{
    let total = Instant::now();
    let mut start = Instant::now();

    let file_name = r"schems/counter3.schem";
    let schem = schem::SchemData::from_file(file_name);
    //let schem = schem::SchemData::from_file("./test_schems/c4AI1.schem");

    println!("Loading Schem Took: {:?}", start.elapsed());
 
    //println!("{schem:#?}");
    
    start = Instant::now();
    let attr = items::ItemAttributes::from_file("ItemAttributes.json");

    //println!("Loading Attributes Took: {:?}", start.elapsed());

    start = Instant::now();
    let r_schem = schem::RedSchem::from_schem(schem, &attr);

    println!("Parsing Schem Took: {:?}", start.elapsed());

    //println!("{r_schem:#?}");

    start = Instant::now();
    let graph = graph::Graph::from_redschem(&r_schem);

    println!("Building Graph Took: {:?}", start.elapsed());

    //println!("{graph:#?}");

    let mut file = File::create("graph_out.txt").unwrap();
    let _ = file.write_all(format!("{graph:#?}").as_bytes());

    start = Instant::now();
    assembler::generate_verilog(&graph, "../../Quartus/Verilog/redstone.v", &r_schem.size);

    println!("Generating Verilog Took: {:?}\n", start.elapsed());

    println!("Total Compilation Time: {:?}", total.elapsed());

    graph.get_context(outputs, inputs);

    let stdout = Command::new("cmd")
        .args(&["/C", "compile"])
        .stdout(Stdio::piped())
        .spawn().unwrap()
        .stdout
        .ok_or_else(|| Error::new(ErrorKind::Other,"Could not capture standard output.")).unwrap();

    let reader = BufReader::new(stdout);
    
    reader
        .lines()
        .filter_map(|line| line.ok())
        .for_each(|line| println!("{}", line));


    let stdout = Command::new("cmd")
        .args(&["/C", "program"])
        .stdout(Stdio::piped())
        .spawn().unwrap()
        .stdout
        .ok_or_else(|| Error::new(ErrorKind::Other,"Could not capture standard output.")).unwrap();

    let reader = BufReader::new(stdout);
    
    reader
        .lines()
        .filter_map(|line| line.ok())
        .for_each(|line| println!("{}", line));

}

