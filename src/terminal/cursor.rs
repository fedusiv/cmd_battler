use super::cell::Cell;
use super::{rect::Rect, symbols};
use crate::utils::Vector2;
use std::ptr;

pub enum CursorMoves {
    Up,
    Down,
    Left,
    Right,
}

pub enum Zones {
    Area,
}

pub struct Cursor {
    pub position: Vector2, // logic position inside current rect
    pub view: Zones,
    pub last_content: *const Cell,
}

impl Default for Cursor {
    // Reminder: This is default, and it should not work with default, anyway values need to be changed somewhere on init stage
    fn default() -> Self {
        Cursor {
            position: Vector2 { x: 0, y: 0 },
            view: Zones::Area,
            last_content: ptr::null(),
        }
    }
}
