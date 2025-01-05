#[derive(Debug)]
pub struct LUT
{
    pub table: Vec<TFX>
}

impl LUT
{
    pub fn new(size: u8) -> LUT
    {
        LUT{table: Vec::with_capacity(2_usize.pow(size as u32))}
    }
}

#[derive(Debug)]
pub enum TFX
{
    One,
    Zero,
    X
}

