use super::cell::Cell;
use crate::common::vector2::Vector2;

pub enum Zones {
    Area,
}

pub struct Cursor {
    pub position: Vector2, // logic position inside current rect
    pub view: Zones,
    pub last_content: Cell,
}

impl Default for Cursor {
    // Reminder: This is default, and it should not work with default, anyway values need to be changed somewhere on init stage
    fn default() -> Self {
        Cursor {
            position: Vector2 { x: 0, y: 0 },
            view: Zones::Area,
            last_content: Cell::default(),
        }
    }
}
