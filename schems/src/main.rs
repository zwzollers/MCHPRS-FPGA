
use std::time::Instant;

mod items;
mod schem;
mod graph;

fn main() 
{
    let mut start = Instant::now();
    let schem = schem::SchemData::from_file("./test4.schem");

    println!("Loading Schem Took: {:?}", start.elapsed());
    
    start = Instant::now();
    let attr = items::ItemAttributes::from_file("./ItemAttributes.json");

    println!("Loading Attributes Took: {:?}", start.elapsed());

    start = Instant::now();
    let r_schem = schem::RedSchem::from_schem(schem, &attr);

    println!("Parsing Schem Took: {:?}", start.elapsed());

    println!("{r_schem:#?}");

    start = Instant::now();
    let nodes = graph::Graph::from_redschem(&r_schem);

    println!("Building Graph Took: {:?}", start.elapsed());

    println!("{nodes:#?}");
}