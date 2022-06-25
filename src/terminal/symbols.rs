use super::cell::{Cell, Colour};

pub const EMPTY_CELL: Cell =                Cell{ content: ' ', fg: Colour::Gray, bg: Colour::Black };
pub const CURSOR: Cell =                    Cell{ content: '█', fg: Colour::Gray, bg: Colour::Yellow };
// Borders§
pub const BORDER_LEFT_UP_CORNER: Cell =         Cell{ content: '╔', fg: Colour::White, bg: Colour::Black };
pub const BORDER_RIGHT_UP_CORNER: Cell =        Cell{ content: '╗', fg: Colour::White, bg: Colour::Black };
pub const BORDER_LEFT_BOTTOM_CORNER: Cell =     Cell{ content: '╚', fg: Colour::White, bg: Colour::Black };
pub const BORDER_RIGHT_BOTTOM_CORNER: Cell =    Cell{ content: '╝', fg: Colour::White, bg: Colour::Black };
pub const BORDER_HORIZONTAL: Cell =             Cell{ content: '═', fg: Colour::White, bg: Colour::Black };
pub const BORDER_VERTICAL: Cell =               Cell{ content: '║', fg: Colour::White, bg: Colour::Black };
