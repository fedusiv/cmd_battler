use crate::utils::Vector2;
use super::{rect::Rect, symbols};

pub enum Zones{
    Area
}

pub struct Cursor{
    pub position: Vector2,  // real position
    pub zone: Zones
}

impl Cursor{
    // Called before drawing, cursor will change the cell he is on, to be representative
    pub fn change_cell_view(&self, zone: &mut Rect){
        let cell = zone.content_unsafe(&self.position);
        if cell.bg == symbols::CURSOR.bg{
            // cursor already there
            return ;
        }
        // call function to change cell representation
        zone.change_cell_data(self.position, None, None, Some(symbols::CURSOR.bg));
        
    }
}

impl Default for Cursor {
    fn default() -> Self {
        Cursor { 
            position: Vector2 { x: 0, y: 0 },
            zone: Zones::Area }
    }
}
