use super::Pass;
use crate::blocks::Block;
use crate::plot::PlotWorld;
use crate::redpiler::compile_graph::{CompileGraph, CompileNode, NodeState, NodeType};
use crate::redpiler::{CompilerInput, CompilerOptions};
use crate::world::World;
use mchprs_blocks::block_entities::BlockEntity;
use mchprs_blocks::BlockPos;

pub struct IdentifyNodes;

impl Pass for IdentifyNodes {
    fn run_pass(
        &self,
        graph: &mut CompileGraph,
        options: &CompilerOptions,
        input: &CompilerInput<'_>,
    ) {
        let ignore_wires = options.optimize;
        let plot = input.plot;

        let (first_pos, second_pos) = plot.get_corners();

        let start_pos = first_pos.min(second_pos);
        let end_pos = first_pos.max(second_pos);
        for y in start_pos.y..=end_pos.y {
            for z in start_pos.z..=end_pos.z {
                for x in start_pos.x..=end_pos.x {
                    let pos = BlockPos::new(x, y, z);
                    for_pos(ignore_wires, plot, graph, pos);
                }
            }
        }
    }
}

fn for_pos(ignore_wires: bool, plot: &PlotWorld, graph: &mut CompileGraph, pos: BlockPos) {
    let id = plot.get_block_raw(pos);
    let block = Block::from_id(id);

    let Some((ty, state)) = identify_block(block, pos, plot) else {
        return;
    };

    let facing_diode = if let Block::RedstoneRepeater { repeater } = block {
        plot.get_block(pos.offset(repeater.facing.opposite().block_face()))
            .is_diode()
    } else if let Block::RedstoneComparator { comparator } = block {
        plot.get_block(pos.offset(comparator.facing.opposite().block_face()))
            .is_diode()
    } else {
        false
    };

    if ignore_wires && ty == NodeType::Wire {
        return;
    }

    graph.add_node(CompileNode {
        ty,
        block: Some((pos, id)),
        state,

        facing_diode,
        comparator_far_input: None,
    });
}

fn identify_block(block: Block, pos: BlockPos, world: &PlotWorld) -> Option<(NodeType, NodeState)> {
    let (ty, state) = match block {
        Block::RedstoneRepeater { repeater } => (
            NodeType::Repeater(repeater.delay),
            NodeState::repeater(repeater.powered, repeater.locked),
        ),
        Block::RedstoneComparator { comparator } => (
            NodeType::Comparator(comparator.mode),
            NodeState::comparator(
                comparator.powered,
                if let Some(BlockEntity::Comparator { output_strength }) =
                    world.get_block_entity(pos)
                {
                    *output_strength
                } else {
                    0
                },
            ),
        ),
        Block::RedstoneTorch { lit, .. } | Block::RedstoneWallTorch { lit, .. } => {
            (NodeType::Torch, NodeState::simple(lit))
        }
        Block::RedstoneWire { wire } => (NodeType::Wire, NodeState::ss(wire.power)),
        Block::StoneButton { button } => (NodeType::Button, NodeState::simple(button.powered)),
        Block::RedstoneLamp { lit } => (NodeType::Lamp, NodeState::simple(lit)),
        Block::Lever { lever } => (NodeType::Lever, NodeState::simple(lever.powered)),
        Block::StonePressurePlate { powered } => {
            (NodeType::PressurePlate, NodeState::simple(powered))
        }
        Block::IronTrapdoor { powered, .. } => (NodeType::Trapdoor, NodeState::simple(powered)),
        Block::RedstoneBlock {} => (NodeType::Constant, NodeState::ss(15)),
        block if block.has_comparator_override() => (
            NodeType::Constant,
            NodeState::ss(block.get_comparator_override(world, pos)),
        ),
        _ => return None,
    };
    Some((ty, state))
}
