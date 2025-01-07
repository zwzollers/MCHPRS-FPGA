mod serial;
mod LUT;
mod compile;
mod graph;
mod items;
mod schem;
mod assembler;

use super::JITBackend;
use crate::compile_graph::CompileGraph;
use crate::task_monitor::TaskMonitor;
use crate::{block_powered_mut, CompilerOptions};
use mchprs_blocks::block_entities::BlockEntity;
use mchprs_blocks::blocks::{self, Block, ComparatorMode, Instrument};
use mchprs_blocks::BlockPos;
use mchprs_redstone::{bool_to_ss, noteblock};
use mchprs_world::World;
use mchprs_world::{TickEntry, TickPriority};
use rustc_hash::FxHashMap;
use std::collections::HashMap;
use std::sync::Arc;
use std::{fmt, mem};
use tracing::{debug, warn};
use serial::{SerialConnection, SerialCommands};
use graph::Node;
use std::time::Instant;
use std::fs::File;
use std::io::prelude::*;
use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader, Error, ErrorKind};

#[derive(Default)]
pub struct FPGABackend {
    outputs: Vec<BlockPos>,
    inputs: HashMap<BlockPos, Input>,
    serial_conn: SerialConnection,
}

#[derive(Debug, Clone)]
pub struct Input {
    lever: blocks::Lever,
    id: usize,
    changed: bool,
}

impl JITBackend for FPGABackend {
    fn inspect(&mut self, pos: BlockPos) {

    }

    fn reset<W: World>(&mut self, world: &mut W, io_only: bool) {

    }

    fn on_use_block(&mut self, pos: BlockPos) {
        let node = self.inputs.get_mut(&pos);
        match node
        {
            Some(input) => 
            {
                self.serial_conn.transmit(0x02);
                self.serial_conn.transmit(input.id as u8);
                input.lever.powered = !input.lever.powered;
                let state = if input.lever.powered {1} else {0};
                self.serial_conn.transmit(state as u8);
                input.changed = true;
            }
            None => {}
        }
    }

    fn set_pressure_plate(&mut self, pos: BlockPos, powered: bool) 
    {

    }

    fn tick(&mut self) {

    }

    fn flush<W: World>(&mut self, world: &mut W, io_only: bool) 
    { 
        match self.serial_conn.read_serial(self.outputs.len())
        {
            Some(data) =>
            {
                let mut output_id = 0;
                for byte in data
                {
                    for i in 0..=7
                    {
                        mchprs_world::World::set_block(world, self.outputs[output_id], mchprs_blocks::blocks::Block::from_id(7418-(((byte >> i) as u32)&0x1_u32)));
                        output_id += 1;
                        if output_id >= self.outputs.len()
                        {
                            break
                        }
                    }
                }   
            }
            None => ()
        }

        for (pos, input) in &self.inputs
        {
            mchprs_world::World::set_block(world, *pos, Block::Lever{lever: input.lever});   
        }
    }

    fn compile(
        &mut self,
        graph: CompileGraph,
        ticks: Vec<TickEntry>,
        options: &CompilerOptions,
        monitor: Arc<TaskMonitor>,
    ) {
        let total = Instant::now();
        let mut start = Instant::now();
        let schem = schem::SchemData::from_file("schems/counter3.schem");
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
        let mut id = 0;
        for block in &r_schem.block_array
        {
            match &r_schem.unified_palette[*block as usize]
            {
                schem::RBlock::Lever { dir, state } => 
                {
                    let lever = blocks::Lever{face: blocks::LeverFace::Floor, facing: mchprs_blocks::BlockDirection::East, powered: *state};
                    let input: Input = Input{lever: lever, id: id, changed: false};
                    id += 1;
                    //self.inputs.insert(graph::Pos::from_index(*block as usize, &r_schem.size).to_BlockPos(), input);
                    self.inputs.insert(BlockPos{x:120,y:8,z:155}, input);
                }
                _ => ()
            }
        }

        start = Instant::now();
        let graph = graph::Graph::from_redschem(&r_schem);

        println!("Building Graph Took: {:?}", start.elapsed());

        //println!("{graph:#?}");

        let mut file = File::create("graph_out.txt").unwrap();
        let _ = file.write_all(format!("{graph:#?}").as_bytes());

        start = Instant::now();
        assembler::generate_verilog(&graph, "Quartus/Verilog/redstone.v", &r_schem.size,  self);

        println!("Generating Verilog Took: {:?}\n", start.elapsed());

        println!("Total Compilation Time: {:?}", total.elapsed());


        // let stdout = Command::new("cmd")
        //     .args(&["/C", "compile"])
        //     .stdout(Stdio::piped())
        //     .spawn().unwrap()
        //     .stdout
        //     .ok_or_else(|| Error::new(ErrorKind::Other,"Could not capture standard output.")).unwrap();

        // let reader = BufReader::new(stdout);
        
        // reader
        //     .lines()
        //     .filter_map(|line| line.ok())
        //     .for_each(|line| println!("{}", line));


        // let stdout = Command::new("cmd")
        //     .args(&["/C", "program"])
        //     .stdout(Stdio::piped())
        //     .spawn().unwrap()
        //     .stdout
        //     .ok_or_else(|| Error::new(ErrorKind::Other,"Could not capture standard output.")).unwrap();

        // let reader = BufReader::new(stdout);
        
        // reader
        //     .lines()
        //     .filter_map(|line| line.ok())
        //     .for_each(|line| println!("{}", line));


        self.serial_conn = SerialConnection{port_name: "COM4".to_string(), baud_rate: 2_500_000, conn: None};
        println!("linking");
        self.serial_conn.start_conn();
    }


    fn has_pending_ticks(&self) -> bool {
        false
    }
}