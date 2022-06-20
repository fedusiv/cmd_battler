use crate::utils::Vector2 as Vector2;
use crate::terminal::symbols as symbols;
use crate::terminal::area::Area2D as Area2D;


pub struct BattleArea{
    pub area: Area2D
}


impl BattleArea {
    pub fn new(s_point: Vector2) -> BattleArea{
        let data = Area2D::new(10, 10, s_point);

        let mut area = BattleArea { 
            area: data
        };
        area.area.fill_with_data(symbols::DOT_SYMBOL);
        area
    }
}
