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
    pub position: Vector2,      // real position
    pub logic_postion: Vector2, // position in view of logic elements inside Rect
    pub view: Zones,
    pub last_content: *const Cell,
}

impl Cursor {
    // Called before drawing, cursor will change the cell he is on, to be representative
    pub fn change_cell_view(&self, zone: &mut Rect) {
        if let Some(cell) = zone.content(&self.position) {
            if cell.bg == symbols::CURSOR.bg {
                // cursor already there
                return;
            }
        }
        // call function to change cell representation
        zone.change_cell_data(self.position, None, None, Some(symbols::CURSOR.bg));
    }
}

impl Default for Cursor {
    // Reminder: This is default, and it should not work with default, anyway values need to be changed somewhere on init stage
    fn default() -> Self {
        Cursor {
            position: Vector2 { x: 0, y: 0 },
            logic_postion: Vector2 { x: 0, y: 0 },
            view: Zones::Area,
            last_content: ptr::null(),
        }
    }
}
