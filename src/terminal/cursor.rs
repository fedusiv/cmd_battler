use crate::utils::Vector2;
use super::{rect::Rect, symbols};

pub enum Zones{
    Area
}

pub struct Cursor{
    position: Vector2,  // real position
    zone: Zones
}

impl Cursor{
    // Called before drawing, cursor will change the cell he is on, to be representative
    fn change_cell_view(&self, zone: &Rect){
        let cell = zone.content_unsafe(&self.position);
        if cell.bg == symbols::CURSOR.bg{
            // cursor already there
            return ;
        }
        
    }
}

impl Default for Cursor {
    fn default() -> Self {
        Cursor { 
            position: Vector2 { x: 0, y: 0 },
            zone: Zones::Area }
    }
}
