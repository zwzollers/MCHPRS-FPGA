use std::collections::{HashMap, HashSet, VecDeque};

use crate::schem::*;

#[derive(Debug)]
pub struct Graph
{
    pub nodes: Vec<Node>,
}

impl Graph
{
    pub fn from_redschem (schem: &RedSchem) -> Graph
    {
        let mut graph: Graph = Graph{nodes: Vec::new()};

        let mut node_pos: HashMap<Pos, NodeID> = HashMap::new();
        let mut i: usize = 0;
        let mut node_i: usize = 0;

        for b in &schem.block_array
        {
            let block = schem.unified_palette.get(*b as usize).unwrap();
            if block.is_node()
            {
                let pos = Pos::from_index(i, &schem.size);
                let node = Node{block: NBlock::from_rblock(block).unwrap(), pos:pos, inputs: Vec::new(), outputs: Vec::new()};
                graph.nodes.push(node);
                node_pos.insert(pos.clone(), NodeID(node_i));
                node_i += 1;
            }
            i += 1;
        }
        for node_id in 0..graph.nodes.len()
        {
            let links: Vec<(usize, u8)> = graph.nodes[node_id].get_links(schem);
            for (link_pos, link_w) in links
            {
                graph.nodes[node_id].outputs.push(Link{ty:LinkType::Normal, weight: link_w, dest: node_pos[&Pos::from_index(link_pos, &schem.size)]});
                graph.nodes[node_pos[&Pos::from_index(link_pos, &schem.size)].0].inputs.push(Link{ty:LinkType::Normal, weight: link_w, dest: NodeID(node_id)});
            }
        }
        graph
    }
}

#[derive(Debug, Clone, Copy)]
pub struct NodeID (pub usize);

#[derive(Debug)]
pub struct Node
{
    pub block: NBlock,
    pub pos: Pos,

    pub inputs: Vec<Link>,
    pub outputs: Vec<Link>
}

impl Node
{
    pub fn get_links (&self, schem: &RedSchem) -> Vec<(usize, u8)>
    {
        println!("POS: {:?}", self.pos);
        // position and weights of nodes connected to given node
        let mut links: Vec<(usize, u8)> = Vec::new();
        // list of searched positions (avoids researching and looping)
        let mut searched: HashSet<usize> = HashSet::new();
        // list of redstone dust not yet searched stored as (Pos_Index, Weight) 
        let mut redstone: VecDeque<(usize, u8)> = VecDeque::new();

        // constant offsets used for different search patterns
        const OFFSET_SIDES: [[i32; 3]; 6] = [[0,1,0],[0,-1,0],[1,0,0],[-1,0,0],[0,0,1],[0,0,-1]];
        const OFFSET_MIDDLE: [[i32; 3]; 4] = [[1,0,0],[-1,0,0],[0,0,1],[0,0,-1]];

        // size of the schematic
        let size = &schem.size;
        // current position in index form
        let node_pos = self.pos.to_index(size);
        // add node pos to list of searched positions 
        searched.insert(node_pos);

        // list of hardpower and softpower offsets depending on the node type and direction
        let (hp, sp) = match schem.get_block(Some(node_pos))
        {
            RBlock::Comparator {dir, ..} | 
            RBlock::Repeater {dir, ..} => (vec!(dir.to_offset()), vec![]),
            RBlock::Torch {..} => (vec![[0 as i32,1 as i32,0 as i32]], OFFSET_SIDES.to_vec()),
            RBlock::Lever { dir, ..} => (vec!(dir.to_offset()), OFFSET_SIDES.to_vec()),
            _ => (vec![], vec![])
        };

        for offset in hp
        {
            // get offset position index
            let offset_pos = Pos::relative_index(node_pos, offset, size);
            // contuine if position goes out of bounds or has already been searched
            if offset_pos == None || searched.contains(&offset_pos.unwrap()) {continue;}

            // match block at offset position
            match schem.get_block(offset_pos)
            {
                RBlock::Redstone => 
                {
                    redstone.push_front((offset_pos.unwrap(), 0)); 
                    searched.insert(offset_pos.unwrap());
                }
                RBlock::Comparator {dir, ..} | 
                RBlock::Repeater {dir, ..} =>
                {
                    // if node facing the same direction as the offset
                    if Direction::from_offset(offset) == *dir
                    {
                        links.push((offset_pos.unwrap(), 0));
                        searched.insert(offset_pos.unwrap());
                    }
                }
                RBlock::Lamp =>
                {
                    
                }
                x if x.is_solid() =>
                {
                    searched.insert(offset_pos.unwrap());

                    if *x == RBlock::Lamp
                    {
                        links.push((offset_pos.unwrap(), 0));
                    }

                    // searching all sides of a hardpowered block
                    for offset in OFFSET_SIDES
                    {
                        // get offset position index
                        let offset_pos = Pos::relative_index(offset_pos.unwrap(), offset, size);
                        // contuine if position goes out of bounds or has already been searched
                        if offset_pos == None || searched.contains(&offset_pos.unwrap()) {continue;}

                        match schem.get_block(offset_pos)
                        {
                            RBlock::Redstone => 
                            {
                                redstone.push_front((offset_pos.unwrap(),0));
                                searched.insert(offset_pos.unwrap());
                            }
                            RBlock::Comparator {dir, ..} | 
                            RBlock::Repeater {dir, ..} |
                            RBlock::Torch{dir, ..} =>
                            {
                                if Direction::from_offset(offset) == *dir
                                {
                                    links.push((offset_pos.unwrap(), 0));
                                    searched.insert(offset_pos.unwrap());
                                }
                            }
                            RBlock::Lamp =>
                            {
                                links.push((offset_pos.unwrap(), 0));
                                searched.insert(offset_pos.unwrap());
                            }
                            _ => ()
                        }
                    }
                }
                _ => ()
            }
        }
        // same thing for soft power except blocks cant be powered
        for offset in sp
        {
            // get offset position index
            let offset_pos = Pos::relative_index(node_pos, offset, size);
            // contuine if position goes out of bounds or has already been searched
            if offset_pos == None || searched.contains(&offset_pos.unwrap()) {continue;}

            match schem.get_block(offset_pos)
            {
                RBlock::Redstone => 
                {
                    redstone.push_front((offset_pos.unwrap(),0));
                    searched.insert(offset_pos.unwrap());
                }
                RBlock::Comparator {dir, ..} | 
                RBlock::Repeater {dir, ..} =>
                {
                    if Direction::from_offset(offset) == *dir
                    {
                        links.push((offset_pos.unwrap(), 0));
                        searched.insert(offset_pos.unwrap());
                    }
                }
                RBlock::Lamp =>
                {
                    links.push((offset_pos.unwrap(), 0));
                    searched.insert(offset_pos.unwrap());
                }
                _ => ()
            }
        }
        // at this point the redstone list has been initalized with all 0 weight redstone 
        // and all nodes that were directly connected to this node are accounted for

        // now we have to traverse the redstone dust 
        while redstone.len() != 0
        {
            // get next redstone data
            let (redstone_pos, weight) = redstone.pop_back().unwrap();
            // list of offsets that the redstone is redirected to
            let mut redstone_connections: Vec<[i32; 3]> = Vec::new();
            // list of solid blocks that the redstone is soft powering
            let mut connected_blocks: Vec<(usize, u8)> = Vec::new();

            println!("  red: {:?}", Pos::from_index(redstone_pos, size));

            // is the block above and below solid
            // this is going to be used to determine where possible connections will be
            let pos_below = Pos::relative_index(redstone_pos, [0,-1,0], size);
            let solid_below = schem.get_block(pos_below).is_solid();
            let pos_above = Pos::relative_index(redstone_pos, [0,1,0], size);
            let solid_above = schem.get_block(pos_above).is_solid();

            // if the block below is solid add it to the list of connected blocks
            
            if solid_below && !searched.contains(&pos_below.unwrap())
            {
                connected_blocks.push((pos_below.unwrap(), weight));
                searched.insert(pos_below.unwrap());
            }

            // get offsets for each side of the redstone
            for offset in OFFSET_MIDDLE
            {
                // get offset position index
                let offset_pos = Pos::relative_index(redstone_pos, offset, size);
                let cutoff = schem.get_block(offset_pos).is_solid();
                if offset_pos != None
                {
                    match schem.get_block(offset_pos)
                    {  
                        RBlock::Redstone =>
                        {
                            redstone_connections.push(offset);
                            if !searched.contains(&offset_pos.unwrap())
                            {
                                redstone.push_front((offset_pos.unwrap(), weight+1));
                                searched.insert(offset_pos.unwrap());
                            }
                            
                        }
                        RBlock::Target =>
                        {
                            redstone_connections.push(offset);
                            if !searched.contains(&offset_pos.unwrap())
                            {
                                connected_blocks.push((offset_pos.unwrap(), weight+1));
                                searched.insert(offset_pos.unwrap());
                            }
                        }
                        RBlock::Comparator {dir, ..} | 
                        RBlock::Repeater {dir, ..} =>
                        {
                            if Direction::from_offset(offset) == *dir
                            {
                                redstone_connections.push(offset);
                                if !searched.contains(&offset_pos.unwrap())
                                {
                                    links.push((offset_pos.unwrap(), weight));
                                    searched.insert(offset_pos.unwrap());
                                }
                            }
                        }
                        RBlock::Torch {..} =>
                        {
                            redstone_connections.push(offset);
                        }
                        RBlock::Lamp =>
                        {
                            links.push((offset_pos.unwrap(), weight));
                            searched.insert(offset_pos.unwrap());
                        }
                        _ => ()
                    } 
                }
                

                if !solid_above
                {
                    let offset_pos = Pos::relative_index(redstone_pos, [offset[0],1,offset[2]], size);
                    if offset_pos != None
                    {
                        match schem.get_block(offset_pos)
                        {  
                            RBlock::Redstone =>
                            {
                                redstone_connections.push(offset);
                                if !searched.contains(&offset_pos.unwrap())
                                {
                                    redstone.push_front((offset_pos.unwrap(), weight+1));
                                    searched.insert(offset_pos.unwrap());
                                }  
                            }
                            _ => ()
                        } 
                    }
                }

                if solid_below  && !cutoff
                {
                    let offset_pos = Pos::relative_index(redstone_pos, [offset[0],-1,offset[2]], size);
                    if offset_pos != None
                    {
                        match schem.get_block(offset_pos)
                        {  
                            RBlock::Redstone =>
                            {
                                redstone_connections.push(offset);
                                if !searched.contains(&offset_pos.unwrap())
                                {
                                    redstone.push_front((offset_pos.unwrap(), weight+1));
                                    searched.insert(offset_pos.unwrap());
                                }  
                            }
                            _ => ()
                        }
                    }
                }
            }

            if redstone_connections.len() == 0
            {
                for offset in OFFSET_MIDDLE
                {
                    let opos = Pos::relative_index(redstone_pos, offset, size);
                    if opos == None || searched.contains(&opos.unwrap()) || !schem.get_block(opos).is_solid() {continue;}
                    connected_blocks.push((opos.unwrap(),weight));
                    searched.insert(opos.unwrap());
                }
            }

            if redstone_connections.len() == 1
            {
                let opposite = [redstone_connections[0][0]*-1, 0, redstone_connections[0][2]*-1];
                let opos = Pos::relative_index(redstone_pos, opposite, size);
                if !(opos == None || searched.contains(&opos.unwrap()) || !schem.get_block(opos).is_solid())
                {
                    connected_blocks.push((opos.unwrap(),weight));
                    searched.insert(opos.unwrap());
                }
            }

            for conn in redstone_connections
            {
                let opos = Pos::relative_index(redstone_pos, conn, size);
                if opos == None || searched.contains(&opos.unwrap()) || !schem.get_block(opos).is_solid() {continue;}
                connected_blocks.push((opos.unwrap(),weight));
                searched.insert(opos.unwrap());
            }

            for (block_pos, weight) in connected_blocks
            {
                for offset in OFFSET_SIDES
                {
                    // get offset position index
                    let offset_pos = Pos::relative_index(block_pos, offset, size);
                    // contuine if position goes out of bounds or has already been searched
                    if offset_pos == None || searched.contains(&offset_pos.unwrap()) {continue;}

                    match schem.get_block(offset_pos)
                    {
                        RBlock::Comparator {dir, ..} | 
                        RBlock::Torch { dir, .. } |
                        RBlock::Repeater {dir, ..} =>
                        {
                            if Direction::from_offset(offset) == *dir
                            {
                                links.push((offset_pos.unwrap(), weight));
                                searched.insert(offset_pos.unwrap());
                            }
                        }
                        RBlock::Lamp =>
                        {
                            links.push((offset_pos.unwrap(), weight));
                            searched.insert(offset_pos.unwrap());
                        }
                        _ => ()
                    }
                }
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
pub enum NBlock
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pos
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
        if x < 0 || x >= size.wid as i32 {return None}
        let y = self.y as i32 + offset[1];
        if y < 0 || y >= size.hth as i32 {return None}
        let z = self.z as i32 + offset[2];
        if z < 0 || z >= size.len as i32 {return None}

        Some((y as usize * size.len as usize + z as usize) * size.wid as usize + x as usize)
    }
    pub fn get_relative(&self, offset: [i32; 3], size: &Size) -> Option<Pos>
    {
        let x = self.x as i32 + offset[0];
        if x < 0 || x >= size.wid as i32 {return None}
        let y = self.y as i32 + offset[1];
        if y < 0 || y >= size.hth as i32 {return None}
        let z = self.z as i32 + offset[2];
        if z < 0 || z >= size.len as i32 {return None}
        
        Some(Pos{x: x as u16, y: y as u16, z: z as u16})
    }
    pub fn relative_index(idx: usize, offset: [i32; 3], size: &Size) -> Option<usize>
    {
        Pos::from_index(idx, size).get_relative_index(offset, size)
    }
    pub fn to_string(&self) -> String
    {
        format!("{}{}{}", self.x, self.y, self.z)
    }
}

#[derive(Debug)]
enum LinkType
{
    Normal,
    Side
}

#[derive(Debug)]
pub struct Link
{
    pub ty: LinkType,
    pub weight: u8,

    pub dest: NodeID
}