#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction
{
    Up,
    Down,
    Left,
    Right,
}

impl Direction
{
    pub fn from_wasd(c: char) -> Option<Direction>
    {
        match c.to_ascii_lowercase()
        {
            'w' => Some(Self::Up),
            'a' => Some(Self::Left),
            's' => Some(Self::Down),
            'd' => Some(Self::Right),
            _   => None,
        }
    }
}