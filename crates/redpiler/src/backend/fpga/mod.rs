mod serial;

use super::JITBackend;
use crate::compile_graph::CompileGraph;
use crate::task_monitor::TaskMonitor;
use crate::{block_powered_mut, CompilerOptions};
use mchprs_blocks::block_entities::BlockEntity;
use mchprs_blocks::blocks::{Block, ComparatorMode, Instrument};
use mchprs_blocks::BlockPos;
use mchprs_redstone::{bool_to_ss, noteblock};
use mchprs_world::World;
use mchprs_world::{TickEntry, TickPriority};
use rustc_hash::FxHashMap;
use std::sync::Arc;
use std::{fmt, mem};
use tracing::{debug, warn};
use serial::{SerialConnection, SerialCommands};

#[derive(Default)]
pub struct FPGABackend {
    outputs: Vec<BlockPos>,
    inputs: Vec<BlockPos>,
    serial_conn: SerialConnection,
}

impl JITBackend for FPGABackend {
    fn inspect(&mut self, pos: BlockPos) {

    }

    fn reset<W: World>(&mut self, world: &mut W, io_only: bool) {

    }

    fn on_use_block(&mut self, pos: BlockPos) {
        println!("{pos:?}");
        
    }

    fn set_pressure_plate(&mut self, pos: BlockPos, powered: bool) {

    }

    fn tick(&mut self) {

    }

    fn flush<W: World>(&mut self, world: &mut W, io_only: bool) {
        
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
                            return
                        }
                    }
                }   
            }
            None => ()
        }
    }

    fn compile(
        &mut self,
        graph: CompileGraph,
        ticks: Vec<TickEntry>,
        options: &CompilerOptions,
        monitor: Arc<TaskMonitor>,
    ) {
        fpga_interface::convert_compile("counter3".to_string(), &mut self.outputs, &mut self.inputs);
        self.serial_conn = SerialConnection{port_name: "COM4".to_string(), baud_rate: 2_500_000, conn: None};
        println!("linking");
        self.serial_conn.start_conn();
    }
    fn has_pending_ticks(&self) -> bool {
        false
    }
}