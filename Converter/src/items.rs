use serde_json:: Value;
use std::fs;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AttributeValueType<'a>
{
    Number(u32),
    String(&'a str),
    None
}

impl<'a> AttributeValueType<'a>
{
    pub fn from_str(str: &str) -> AttributeValueType
    {
        match str.parse::<u32>()
        {
            _ if str.is_empty() => AttributeValueType::None,
            Ok(n) => AttributeValueType::Number(n),
            Err(_) => AttributeValueType::String(str)
        }
    }
    pub fn as_number(&self) -> u32
    {
        match self
        {
            AttributeValueType::Number(n) => *n,
            _ => panic!("not a number")
        }
    }
    pub fn as_string(&self) -> &str
    {
        match self
        {
            AttributeValueType::String(s) => *s,
            _ => panic!("not a number")
        }
    }
}

#[derive(Debug)]
pub struct ItemAttributes
{
    pub blocks: Value
}


impl ItemAttributes
{
    pub fn from_file(filename: &str) -> ItemAttributes
    {
        let data: String = fs::read_to_string(filename).unwrap();
        ItemAttributes{blocks:serde_json::from_str(&data).unwrap()}
    }

    pub fn get_attr(&self, block_id: &str, attr: &str) -> AttributeValueType
    {
        let res: &Value = &self.blocks[block_id][attr];
        if res.is_null()
        {
            return AttributeValueType::None
        }
        match res.as_number()
        {
            Some(n) => AttributeValueType::Number(n.as_u64().unwrap() as u32),
            None => AttributeValueType::String(res.as_str().unwrap())
        }
        
    }
}