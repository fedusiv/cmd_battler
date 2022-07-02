use crate::utils::Vector2;

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub enum Colour {
    Reset,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    Gray,
    LightRed,
    LightGreen,
    LightYellow,
    LightBlue,
    LightMagenta,
    LightCyan,
    LightGray,
    White,
    Rgb(u8, u8, u8),
    Indexed(u8),
}

impl From<Colour> for crossterm::style::Color {
    fn from(color: Colour) -> Self {
        use crossterm::style::Color as CColor;

        match color {
            Colour::Reset => CColor::Reset,
            Colour::Black => CColor::Black,
            Colour::Red => CColor::DarkRed,
            Colour::Green => CColor::DarkGreen,
            Colour::Yellow => CColor::DarkYellow,
            Colour::Blue => CColor::DarkBlue,
            Colour::Magenta => CColor::DarkMagenta,
            Colour::Cyan => CColor::DarkCyan,
            Colour::Gray => CColor::DarkGrey,
            Colour::LightRed => CColor::Red,
            Colour::LightGreen => CColor::Green,
            Colour::LightBlue => CColor::Blue,
            Colour::LightYellow => CColor::Yellow,
            Colour::LightMagenta => CColor::Magenta,
            Colour::LightCyan => CColor::Cyan,
            Colour::LightGray => CColor::Grey,
            Colour::White => CColor::White,
            Colour::Indexed(i) => CColor::AnsiValue(i),
            Colour::Rgb(r, g, b) => CColor::Rgb { r, g, b },
        }
    }
}

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
