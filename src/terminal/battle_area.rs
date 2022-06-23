use crate::utils::Vector2 as Vector2;
use crate::terminal::rect::Rect as Rect;

use super::parameters;


pub struct BattleArea{
    pub rect: Rect
}


impl BattleArea {
    pub fn new(s_point: Vector2) -> BattleArea{
        let data = Rect::new( parameters::BATTLE_AREA_SIZE,  s_point, "Area".to_string());

        let area = BattleArea { 
            rect: data
        };
        area
    }
}
