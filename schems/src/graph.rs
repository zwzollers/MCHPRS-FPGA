use std::collections::HashMap;

use crate::schem::RBlock;
use crate::schem::RedSchem;
use crate::schem::SchemData;

#[derive(Debug)]
pub struct Graph<'a>
{
    pub nodes: Vec<Node<'a>>,
    pub links: Vec<Link<'a>>
}

impl<'a> Graph<'a>
{
    fn delete (&mut self, pos: usize)
    {
        let removed: Node = self.nodes.swap_remove(pos);

    }

    pub fn from_redschem (schem: &RedSchem) -> Graph
    {
        let mut graph: Graph = Graph{nodes: Vec::new(), links: Vec::new()};

        let mut node_positions: HashMap<usize, Node> = HashMap::new();
        let mut i: usize = 0;

        for b in &schem.block_array
        {
            match Node::from_block(*b, i, schem)
            {
                Some(n) => {node_positions.insert(i, n);}
                None => ()
            };
            i += 1;
        }
        for (pos, node) in node_positions.into_iter()
        {
            let mut links: Vec<(usize, u8)> = Vec::new();
            schem.search(pos, 0, &mut links);
        }
        graph
    }
}
#[derive(Debug)]
struct Node<'a>
{
    block: NBlock,

    inputs: Vec<&'a Link<'a>>,
    outputs: Vec<&'a Link<'a>>
}

impl<'a> Node<'a>
{
    fn from_block(block: u16, pos: usize, schem: &RedSchem) -> Option<Node<'a>>
    {
        match RedSchem::is_supported(pos, schem)
        {
            true =>
            {
                match NBlock::from_rblock(&schem.unified_palette[block as usize])
                {
                    Some(nblock) => Some(Node{block: nblock, inputs: Vec::new(), outputs: Vec::new()}),
                    None => None
                }
            }
            false => None
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum CompMode
{
    Compare = 0,
    Subtract = 1
}

#[derive(Debug)]
enum NBlock
{
    Repeater{delay: u8, state: bool},
    Comparator{mode: CompMode, state: u8},
    Torch{state: bool},
    Input{state: bool},
    Output
}

impl NBlock
{
    fn from_rblock(block: &RBlock) -> Option<NBlock>
    {
        match block
        {
            RBlock::Repeater { delay, dir:_, state } => Some(NBlock::Repeater { delay: *delay, state: *state }),
            RBlock::Comparator { mode, dir:_, state } => Some(NBlock::Comparator { mode: mode.clone(), state: *state }),
            RBlock::Torch { dir:_, state } => Some(NBlock::Torch { state: *state }),
            RBlock::Lever { dir:_, state } => Some(NBlock::Input { state: *state }),
            RBlock::Lamp => Some(NBlock::Output),
            _ => None
        }
    }
}

#[derive(Debug)]
enum LinkType
{
    Normal,
    Side
}

#[derive(Debug)]
pub struct Link<'a>
{
    ty: LinkType,

    weight: u8,

    src: &'a Node<'a>,
    dest: &'a Node<'a>
}