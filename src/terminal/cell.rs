use crate::common::colour::Colour;
use crate::common::vector2::Vector2;

pub struct CellDraw {
    pub cell: Cell,
    pub point: Vector2,
}

#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub struct Cell {
    pub content: char,
    pub fg: Colour,
    pub bg: Colour,
}

impl Default for Cell {
    fn default() -> Cell {
        Cell {
            content: ' ',
            fg: Colour::Gray,
            bg: Colour::Black,
        }
    }
}
