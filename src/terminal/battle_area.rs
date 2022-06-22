use crate::utils::Vector2 as Vector2;
use crate::terminal::rect::Rect as Rect;

use super::symbols;


pub struct BattleArea{
    pub rect: Rect
}


impl BattleArea {
    pub fn new(s_point: Vector2) -> BattleArea{
        let data = Rect::new(40, 20, s_point);

        let mut area = BattleArea { 
            rect: data
        };
        area.rect.fill_all_cells(symbols::EMPTY_BATTLE_AREA_CELL);
        area
    }
}
