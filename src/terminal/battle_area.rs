use crate::utils::Vector2 as Vector2;
use crate::terminal::rect::Rect as Rect;

use super::parameters;


pub struct BattleArea{
    pub rect: Rect,
    last_cursor_pos: Vector2    // local postion
}


impl BattleArea {
    pub fn new(s_point: Vector2) -> BattleArea{
        let data = Rect::new( parameters::BATTLE_AREA_SIZE,  s_point, "Area".to_string());
        let pos = Vector2{x:1,y:1};

        let area = BattleArea { 
            rect: data,
            last_cursor_pos: pos
        };
        area
    }

    // When cursor moving to other rect, need to get position of starting point for cursor
    // return position in global coordinates
    pub fn get_cursor_pos(&self) -> Vector2{
        self.rect.convert_to_global_coor_return(&self.last_cursor_pos)
    }
}
