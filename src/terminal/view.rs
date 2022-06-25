use crate::utils::Vector2;

use super::{rect::Rect, parameters, cell::Cell, 
            cursor::{self, Cursor}};

pub struct View{
    area: Rect,
    cursor: Cursor
}

impl View {
    pub fn create() -> View{
        let area = Rect::new(
            parameters::BATTLE_AREA_SIZE,
            Vector2{x:2, y:2},  // from where to draw this rect in global coordinates
            Vector2 { x: 1, y: 1 }, // where will be located first cursor position in local coordinates 
            "Area".to_string());
        View { 
            area,
            cursor: Cursor::default()
        }
    }

    pub fn init(&mut self){
        self.cursor.position =  self.area.get_cursor_pos();
    }

    // Main function of drawing.
    // Here decided in which zone this point is taking place and return cell value if it should be redrawn
    // position in global coordinates
    pub fn is_point_need_to_draw(&self, position: Vector2, element: Cell) -> Option<Cell>{
        let mut cell: Option<Cell> = None;
        // first parse area
        if let Some(c) = self.area.content(&position){
            if c != element {
                cell = Some(c);
            }
        }
        // todo()! another views
        cell
    }

    pub fn cursor_apply_cell(&mut self){
        // chose current view, where cursor is located
        let view = match self.cursor.view{
            cursor::Zones::Area => &mut (self.area)
        };
        self.cursor.change_cell_view(view);
    }
}