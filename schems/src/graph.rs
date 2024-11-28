use std::collections::{hash_set, vec_deque, HashMap, HashSet, VecDeque};

use crate::schem::*;

#[derive(Debug)]
pub struct Graph<'a>
{
    pub nodes: Vec<Node<'a>>,
    pub links: Vec<Link<'a>>
}

impl<'a> Graph<'a>
{
    // fn delete (&mut self, pos: usize)
    // {
    //     let removed: Node = self.nodes.swap_remove(pos);
    // }

    pub fn from_redschem (schem: &RedSchem) -> Graph
    {
        let mut graph: Graph = Graph{nodes: Vec::new(), links: Vec::new()};

        let mut nodes: Vec<Pos> = Vec::new();
        let mut i: usize = 0;

        for b in &schem.block_array
        {
            if schem.unified_palette.get(*b as usize).unwrap().is_node()
            {
                nodes.push(Pos::from_index(i, &schem.size));
            }
            i += 1;
        }
        for pos in nodes.into_iter()
        {
            let mut links: Vec<(u8, Pos)> = Node::get_links(&pos, schem);
            
        }
        graph
    }
}
#[derive(Debug)]
struct Node<'a>
{
    block: NBlock,

    pos: Pos,

    inputs: Vec<&'a Link<'a>>,
    outputs: Vec<&'a Link<'a>>
}

impl<'a> Node<'a>
{
    pub fn get_links (pos: &Pos, schem: &RedSchem) -> Vec<(u8, Pos)>
    {
        let mut links: Vec<(u8, Pos)> = Vec::new();
        let mut searched: HashSet<usize> = HashSet::new();
        let mut redstone: VecDeque<(u8, usize)> = VecDeque::new();

        const OFFSET_NONE: [[i32; 3]; 0] = [];
        const OFFSET_SIDES: [[i32; 3]; 6] = [[0,1,0],[0,-1,0],[1,0,0],[-1,0,0],[0,0,1],[0,0,-1]];
        const OFFSET_ALL: [[i32; 3]; 12] = [[1,0,0],[-1,0,0],[0,0,1],[0,0,-1],[1,1,0],[-1,1,0],[0,1,1],[0,1,-1],[1,-1,0],[-1,-1,0],[0,-1,1],[0,-1,-1]];
        const OFFSET_SOME: [[i32; 3]; 8] = [[1,0,0],[-1,0,0],[0,0,1],[0,0,-1],[1,1,0],[-1,1,0],[0,1,1],[0,1,-1]];

        let size = &schem.size;
        let ipos = pos.to_index(size);

        let (hp, sp) = match schem.get_block(Some(ipos))
        {
            RBlock::Comparator {dir:dir, ..} | RBlock::Repeater {dir:dir, ..} => (vec!(dir.to_offset()), OFFSET_NONE.to_vec()),
            RBlock::Torch { dir:dir, state:_ } => (vec![[0 as i32,1 as i32,0 as i32]], OFFSET_SIDES.to_vec()),
            RBlock::Lever { dir:dir, state:_ } => (vec!(dir.to_offset()), OFFSET_SIDES.to_vec()),
            _ => (vec![], vec![])
        };
        for offset in hp
        {
            let opos = Pos::relative_index(ipos, offset, size);
            if opos == None || searched.contains(&opos.unwrap()) {continue;}
            let oblock = schem.get_block(opos);

            match oblock
            {
                RBlock::Redstone => 
                {
                    redstone.push_front((0,opos.unwrap()));
                    searched.insert(opos.unwrap());
                }
                RBlock::Comparator {dir:dir, ..} | 
                RBlock::Repeater {dir:dir, ..} =>
                {
                    if Direction::from_offset(offset) == *dir
                    {
                        links.push((0, Pos::from_index(opos.unwrap(), size)));
                        searched.insert(opos.unwrap());
                    }
                }
                _ if oblock.is_solid() =>
                {
                    for offset in OFFSET_SIDES
                    {
                        let opos = Pos::relative_index(ipos, offset, size);
                        if opos == None || searched.contains(&opos.unwrap()) {continue;}

                        match schem.get_block(opos)
                        {
                            RBlock::Redstone => 
                            {
                                redstone.push_front((0,opos.unwrap()));
                                searched.insert(opos.unwrap());
                            }
                            RBlock::Comparator {dir:dir, ..} | 
                            RBlock::Repeater {dir:dir, ..} |
                            RBlock::Torch{dir:dir, ..} =>
                            {
                                if Direction::from_offset(offset) == *dir
                                {
                                    links.push((0, Pos::from_index(opos.unwrap(), size)));
                                    searched.insert(opos.unwrap());
                                }
                            }
                            _ => ()
                        }
                        
                    }
                }
                _ => ()
            }
        }

        for offset in sp
        {
            let opos = Pos::relative_index(ipos, offset, size);
            if opos == None || searched.contains(&opos.unwrap()) {continue;}

            match schem.get_block(opos)
            {
                RBlock::Redstone => 
                {
                    redstone.push_front((0,opos.unwrap()));
                    searched.insert(opos.unwrap());
                }
                RBlock::Comparator {dir:dir, ..} | 
                RBlock::Repeater {dir:dir, ..} =>
                {
                    if Direction::from_offset(offset) == *dir
                    {
                        links.push((0, Pos::from_index(opos.unwrap(), size)));
                        searched.insert(opos.unwrap());
                    }
                }

                _ => ()
            }
        }

        while redstone.len() != 0
        {
            let (weight, opos) = redstone.pop_back().unwrap();
            if searched.contains(&opos) {continue;}

            let offsets = match schem.get_block(Pos::relative_index(opos, [0,-1,0], size))
            {
                x if x.is_solid() => OFFSET_ALL.to_vec(),
                x if x.is_transparent() => OFFSET_SOME.to_vec(),
                _ => OFFSET_NONE.to_vec()
            };

            for offset in offsets
            {

            }
        }
        links
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Pos
{
    x: u16,
    y: u16,
    z: u16
}

impl Pos
{
    pub fn from_index(idx: usize, size: &Size) -> Pos
    {
        let area = size.len * size.wid;
        let y = idx as u16 / area;
        let left = idx as u16 % area;
        let z = left / size.wid;
        let x = left % size.wid;

        Pos{x: x, y: y, z: z}
    }
    pub fn to_index(&self, size: &Size) -> usize
    {
        (self.y as usize * size.len as usize + self.z as usize) * size.wid as usize + self.x as usize
    }
    pub fn get_relative_index(&self, offset: [i32; 3], size: &Size) -> Option<usize>
    {
        let x = self.x as i32 + offset[0];
        if x < 0 || x >= size.len as i32 {return None}
        let y = self.y as i32 + offset[1];
        if y < 0 || y >= size.hth as i32 {return None}
        let z = self.z as i32 + offset[2];
        if z < 0 || z >= size.wid as i32 {return None}

        Some((y as usize * size.len as usize + z as usize) * size.wid as usize + x as usize)
    }
    pub fn get_relative(&self, offset: [i32; 3], size: &Size) -> Option<Pos>
    {
        let x = self.x as i32 + offset[0];
        if x < 0 || x >= size.len as i32 {return None}
        let y = self.y as i32 + offset[1];
        if y < 0 || y >= size.hth as i32 {return None}
        let z = self.z as i32 + offset[2];
        if z < 0 || z >= size.wid as i32 {return None}
        
        Some(Pos{x: x as u16, y: y as u16, z: z as u16})
    }
    pub fn relative_index(idx: usize, offset: [i32; 3], size: &Size) -> Option<usize>
    {
        Pos::from_index(idx, size).get_relative_index(offset, size)
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