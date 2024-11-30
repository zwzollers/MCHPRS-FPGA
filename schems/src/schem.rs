use fastnbt::{ByteArray, IntArray};
use std::borrow::Cow;
use std::collections::HashSet;
use flate2::read::GzDecoder;
use serde::Deserialize;
use std::io::Read;
use std::collections::HashMap;

use crate::items::AttributeValueType;
use crate::items::ItemAttributes;
use crate::graph::CompMode;
use crate::graph::Link;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Direction
{
    North = 0,
    East = 1,
    South = 2,
    West = 3,
    Up = 4,
    Down = 5
}

impl Direction
{
    fn from_str(facing: &str, face: Option<&str>, reverse: bool) -> Self 
    {
        match face
        {
            Some("wall") | None =>
            {
                match facing
                {
                    "north" => if !reverse {Direction::North} else {Direction::South},
                    "east" => if !reverse {Direction::East} else {Direction::West},
                    "south" => if !reverse {Direction::South} else {Direction::North},
                    "west" => if !reverse {Direction::West} else {Direction::East},
                    _ => panic!("not a valid direction")
                }
            }
            Some(s) =>
            {
                match s
                {
                    "ceiling" => Direction::Up,
                    "floor" => Direction::Down,
                    _ => panic!("not a valid direction")
                }
            }
        }
    }
    pub fn from_offset(arr: [i32; 3]) -> Direction
    {
        match arr
        {
            _ if arr[0] == 1  => Direction::East,
            _ if arr[0] == -1 => Direction::West,
            _ if arr[2] == 1  => Direction::South,
            _ if arr[2] == -1 => Direction::North,
            _ if arr[1] == 1  => Direction::Up,
            _ if arr[1] == -1 => Direction::Down,
            _ => panic!("bad boy choose a better direction")

        }
    }
    pub fn to_offset(&self) -> [i32; 3]
    {
        match self
        {
            Direction::North => [0,0,-1],
            Direction::East =>  [1,0,0],
            Direction::South => [0,0,1],
            Direction::West =>  [-1,0,0],
            Direction::Up =>    [0,1,0],
            Direction::Down =>  [0,-1,0],
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum RBlock
{
    Air,
    Solid,
    Transparent,
    Target,
    SContainer{ss: u8},
    TContainer{ss: u8},
    Redstone,
    Repeater{delay: u8, dir: Direction, state: bool},
    Comparator{mode: CompMode,  dir: Direction, state: u8},
    Torch{dir: Direction, state: bool},
    Lever{dir: Direction, state: bool},
    Lamp
}

impl RBlock
{
    fn from_schem_data(palette: &Option<&HashMap<&str, AttributeValueType>>, entity: &Option<&HashMap<&str, AttributeValueType>>, attr: &ItemAttributes) -> RBlock
    {
        match palette
        {
            Some(p) => 
            {
                let blocktype = match p["id"]
                {
                    AttributeValueType::String(id) => attr.get_attr(id, "block_type"),
                    _ => AttributeValueType::None
                };

                let ty = match blocktype
                {
                    AttributeValueType::String(ty) => ty,
                    _ => "Air"
                };

                match ty
                {
                    "air" =>RBlock::Air,
                    "solid" =>RBlock::Solid,
                    "transparent" =>RBlock::Transparent,
                    "target" =>RBlock::Target,
                    "s_container" =>RBlock::SContainer {
                         ss: entity.unwrap()["ss"].as_number() as u8 },
                    "t_container" =>RBlock::TContainer {
                         ss: entity.unwrap()["ss"].as_number() as u8 },
                    "redstone" =>RBlock::Redstone,
                    "repeater" =>RBlock::Repeater { 
                        delay: p["delay"].as_number() as u8, 
                        dir: Direction::from_str(p["facing"].as_string(), None, true), 
                        state: p["powered"].as_string() == "true" },
                    "comparator" =>RBlock::Comparator { 
                        mode: if p["mode"].as_string() == "compare" {CompMode::Compare } else {CompMode::Subtract}, 
                        dir: Direction::from_str(p["facing"].as_string(), None, true), 
                        state: entity.unwrap()["state"].as_number() as u8 },
                    "torch" =>RBlock::Torch { 
                        dir: Direction::Up, 
                        state: p["lit"].as_string() == "false" },
                    "w_torch" =>RBlock::Torch { 
                        dir: Direction::from_str(p["facing"].as_string(), None, false), 
                        state: p["lit"].as_string() == "false" },
                    "lever" =>RBlock::Lever { 
                        dir: Direction::from_str(p["facing"].as_string(), Some(p["face"].as_string()), true),
                        state: p["powered"].as_string() == "true"},
                    "lamp" =>RBlock::Lamp,
                    _ => RBlock::Air
                }
            }
            None => RBlock::Air
        }
    }
    pub fn is_node(&self) -> bool
    {
        match self
        {
            RBlock::Comparator { mode:_, dir:_, state:_ } |
            RBlock::Lamp |
            RBlock::Lever { dir:_, state:_  } |
            RBlock::Repeater { delay:_, dir:_, state:_ } |
            RBlock::Torch { dir:_, state:_ } => true,
            _ => false
        }
    }
    pub fn is_solid(&self) -> bool
    {
        match self
        {
            RBlock::Solid |
            RBlock::SContainer { ss:_ } |
            RBlock::Lamp |
            RBlock::Target => true,
            _ => false
        }
    }
    pub fn is_transparent(&self) -> bool
    {
        match self
        {
            RBlock::Transparent |
            RBlock::TContainer { ss:_ } => true,
            _ => false
        }
    }
}

#[derive(Debug)]
pub struct Size
{
    pub len: u16,
    pub wid: u16,
    pub hth: u16
}

#[derive(Debug)]
pub struct RedSchem
{
    pub block_array: Vec<u16>,
    pub unified_palette: Vec<RBlock>,
    pub size: Size
}

impl RedSchem
{
    pub fn from_schem(data: SchemData, attr: &ItemAttributes) -> RedSchem
    {
        let palette = data.get_parsed_palette();
        let (entities, entities_data) = data.get_parsed_entities(attr);

        let mut relocation_table: HashMap<(u8,u16), u16>  = HashMap::new();
        let mut catalog: HashMap<RBlock, u16> = HashMap::new();

        let mut count: u16 = 0;
        let mut i = 0;
        
        let mut block_array: Vec<u16> = Vec::new();
        let mut unified_palette: Vec<RBlock> = Vec::new();

        let mut lower_byte: Option<&i8> = None;

        for id in data.block_data.into_iter()
        {
            if id < &0
            {
                lower_byte = Some(id);
                continue;
            }

            let palette_i = match lower_byte
            {
                Some(lb) => SchemData::extent_id(lb, id),
                None => *id as u16
            };

            let entity_i = entities_data[i];
            match relocation_table.get(&(entity_i, palette_i))
            {
                Some(index) =>
                {
                    block_array.push(*index);
                }
                None =>
                {
                    let r_block = RBlock::from_schem_data(&palette.get(&palette_i), &entities.get(&entity_i), attr);
                    match catalog.get(&r_block)
                    {
                        Some (j) => 
                        {
                            relocation_table.insert((entity_i, palette_i), *j);
                            block_array.push(*j);
                        }
                        None => 
                        {
                            catalog.insert(r_block.clone(), count);
                            unified_palette.push(r_block);
                            relocation_table.insert((entity_i, palette_i), count);
                            block_array.push(count);

                            count += 1;
                        }
                    };
                } 
            }
            i += 1;
            lower_byte = None;
        }
        RedSchem{block_array: block_array, unified_palette: unified_palette, size: Size{wid: data.width, len: data.length, hth: data.height}}
    }
    pub fn get_block(&self, index: Option<usize>) -> &RBlock
    {
        match index
        {
            Some(idx) => self.unified_palette.get(self.block_array[idx] as usize).unwrap(),
            None => &RBlock::Air
        }
        
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct SchemData<'a> {
    pub block_data: ByteArray,
    pub block_entities: Vec<Entity<'a>>,
    pub palette: HashMap<String, i32>,
    pub width: u16,
    pub length: u16,
    pub height: u16,

    #[serde(skip)]
    pub num_blocks: u32
}

impl<'a> SchemData<'a>
{
    fn get_parsed_palette(&self) -> HashMap<u16, HashMap<&str, AttributeValueType>>
    {
        let mut res: HashMap<u16, HashMap<&str, AttributeValueType>> = HashMap::new();

        for (data, id) in &self.palette
        {
            let mut pal_attr: HashMap<&str, AttributeValueType> = HashMap::new();
            let mut attr: &str = "\0";
            let mut start = 0;
            for (i, c) in data.chars().enumerate()
            {
                match c
                {
                    '[' => {pal_attr.insert("id", AttributeValueType::from_str(&data[0..i])); start = i+1},
                    '=' => {attr = &data[start..i]; start = i+1},
                    ',' | ']' => {pal_attr.insert(attr, AttributeValueType::from_str(&data[start..i])); start = i+1},
                    _ => ()
                }
            }
            if pal_attr.len() == 0
            {
                pal_attr.insert("id", AttributeValueType::from_str(data));
            }
            res.insert(*id as u16, pal_attr.clone());
        }
        res
    }

    fn get_parsed_entities(&self, attr: &ItemAttributes) -> (HashMap<u8, HashMap<&str, AttributeValueType>>, Vec<u8>)
    {
        let mut entity_palette: HashMap<u8, HashMap<&str, AttributeValueType>> = HashMap::new();
        let mut entity_data: Vec<u8> = vec![0; self.num_blocks as usize];

        let mut count: u8 = 1;

        for ent in &self.block_entities
        {
            let mut ent_attr: HashMap<&str, AttributeValueType> = HashMap::new();

            ent_attr.insert("id", AttributeValueType::String(&ent.id));
            match ent.output_signal
            {
                Some(ss) => {ent_attr.insert("state", AttributeValueType::Number(ss as u32));},
                None => ()
            };
            match ent.get_container_ss(attr)
            {
                Some(ss) => {ent_attr.insert("ss", AttributeValueType::Number(ss as u32));},
                None => ()
            };

            match entity_palette.iter().find_map(|(key, val)| if val == &ent_attr { Some(key) } else { None })
            {   
                Some(i) => entity_data[self.pos_to_index(&ent.pos) as usize] = *i,
                None =>
                {
                    entity_palette.insert(count, ent_attr);
                    entity_data[self.pos_to_index(&ent.pos) as usize] = count;
                    count += 1;
                }
            };
        }
        (entity_palette, entity_data)
    }

    fn pos_to_index (&self, pos: &IntArray) -> u32
    {
        (pos[1] as u32 * self.length as u32 + pos[2] as u32 as u32) * self.width as u32 + pos[0] as u32
    }

    fn extent_id(lb: &i8, ub: &i8) -> u16
    {
        (lb.to_be_bytes()[0] as u16 & 0x7F) | ((ub.to_be_bytes()[0] as u16) << 7)
    }

    pub fn from_file(filename: &str) -> SchemData
    {
        let file = std::fs::File::open(filename).unwrap();

        let mut decoder = GzDecoder::new(file);
        let mut data = vec![];
        decoder.read_to_end(&mut data).unwrap();

        let mut data:SchemData = fastnbt::from_bytes(&data).unwrap();
        data.num_blocks = data.length as u32 * data.width as u32 * data.height as u32;

        data
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Entity<'a>
{
    id: Cow<'a, str>,
    pos: IntArray,
    items: Option<Vec<Slot<'a>>>,
    output_signal: Option<i32> 
}

impl<'a> Entity<'a>
{
    fn get_container_ss (&self, attr: &ItemAttributes) -> Option<u8>
    {
        let mut fill_sum: u32 = 0;
        match &self.items
        {
            Some(slots) =>
            {
                for slot in slots
                {
                    fill_sum += slot.get_fill(attr) as u32;
                }
            }
            None => return None
        }

        let csize = attr.get_attr(&self.id, "container_size");
        match csize
        {
            AttributeValueType::Number(n) => 
            {
                if fill_sum != 0 {Some(((fill_sum * 14) / (64 * n) + 1) as u8)}
                else {Some(0)}
            }
            _ => None
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct Slot<'a>
{
    pub id: Cow<'a, str>,
    #[serde(rename = "Count")]
    pub count: u8
}

impl<'a> Slot<'a>
{
    fn get_fill(&self, attr: &ItemAttributes) -> u8 
    {
        let stack_size = attr.get_attr(&self.id, "stack_size");
        match stack_size
        {
            AttributeValueType::Number(n) => self.count * (64 / n as u8),
            _ => 0            
        }
    }
}