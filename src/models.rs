#[derive(Clone, Copy, PartialEq)]
pub struct Position {
    pub x: u16,
    pub y: u16,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Direction {
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Tile {
    Empty,
    Snake,
    Food,
}
