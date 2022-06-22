use super::cell::{Cell, Colour};

pub const EMPTY_CELL: Cell = Cell{ content: ' ', fg: Colour::Gray, bg: Colour::Black };
pub const EMPTY_BATTLE_AREA_CELL: Cell = Cell{ content: 'X', fg: Colour::Gray, bg: Colour::Black };