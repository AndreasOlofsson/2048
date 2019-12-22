use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TileValue(usize);

impl TileValue
{
    pub fn new(value: usize) -> Option<TileValue>
    {
        if value.count_ones() == 1 && value >= 2
        { Some(TileValue(value)) }
        else
        { None }
    }

    pub fn next_value(&self) -> TileValue
    {
        TileValue(self.0 * 2)
    }
}

impl Into<usize> for TileValue
{
    fn into(self) -> usize
    { self.0 }
}

impl fmt::Display for TileValue
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result
    { <Self as Into<usize>>::into(*self).fmt(fmt) }
}