use crate::utils::{Vector2, self};
use crate::terminal::cell::Cell;

use super::cell::Colour;
use super::symbols;

pub struct Rect{
    pub size: Vector2,
    name: String,
    pub start_point: Vector2, // from where start to draw

    pub visible: bool,  // flag represent, that does this area should be drawn (or displayed)

    pub data: Vec<Cell>
}

impl Rect{
    pub fn new(size: Vector2, s_point: Vector2, name: String) -> Rect{
        let data = vec![Cell::default(); size.x as usize * size.y as usize];
        let mut rect = Rect{
            size,
            name,
            start_point: s_point,
            visible: false,
            data
        };
        rect.init_borders();
        rect
    }

    // Set border symbols to required places.
    fn init_borders(&mut self){

        let height = self.size.y;
        let width = self.size.x;

        for y in 0..height{
            let mut x = 0;
            while x < width{    // need while loop because we need to write a name, look down and you will understand

                let point = Vector2{x,y};
                let mut id = self.get_id_from_pos(point);

                // Corners
                if point == (Vector2{x:0, y:0}) {
                    self.data[id] = symbols::BORDER_LEFT_UP_CORNER;
                }
                else
                if  point == (Vector2{x: width -1, y:0}) {
                    self.data[id] = symbols::BORDER_RIGHT_UP_CORNER;
                }
                else
                if  point == (Vector2{x: 0, y:height -1}) {
                    self.data[id] = symbols::BORDER_LEFT_BOTTOM_CORNER;
                }
                else
                if point == (Vector2{x: width -1, y: height -1}) {
                    self.data[id] = symbols::BORDER_RIGHT_BOTTOM_CORNER;
                }
                // vertical borders
                else 
                if (point.x == 0) || (point.x == width - 1) {
                    self.data[id] = symbols::BORDER_VERTICAL;
                }
                // Name of Rect in Horizontal upper border
                else
                if point == (Vector2{x:2, y: 0}){
                    for c in self.name.chars(){
                        self.data[id] = utils::create_char_cell(c);
                        x+=1;
                        id+=1;
                    }
                    continue;   // to avoid incrementing x again in the end of loop
                }
                // horizontal borders
                else
                if  (point.y == 0) || (point.y == height - 1) {
                    self.data[id] = symbols::BORDER_HORIZONTAL;
                }
                // empty cell
                else {
                    self.data[id] = symbols::EMPTY_CELL;
                }
                x+=1;
            }
        }
    }

    // check does given point take place in current area
    pub fn is_in_area(&self, point: &Vector2) -> bool{
        // maybe so straight forward check, but ok
        if (point.x >= self.start_point.x) && (point.x < self.start_point.x + self.size.x){
            if (point.y >= self.start_point.y) && (point.y < self.start_point.y + self.size.y)
            {
                return true;
            }
        }
        false
    }

    // This function works without verification because expect to be run is_in_area first
    // point in global coordinates
    pub fn content_unsafe(&self, point: &Vector2) -> Cell{
            let area_point = Vector2{y: point.y - self.start_point.y, x: point.x - self.start_point.x};
            let index = ( self.size.x * area_point.y + area_point.x ) as usize;
            self.data[index]
    }

    // converting position of given point inside rect to position in global rect
    // position in local coordinates
    pub fn convert_to_global_coor(&self, point: &mut Vector2){
        *point = *point + self.start_point;
    }

    // converting position of given position in global coordinates to local coordinates
    pub fn convert_to_local_coordinate(&self, point: &mut Vector2){
        *point = *point - self.start_point;
    }

    // position in global coordinates
    pub fn change_cell_bg(&self, position: Vector2, color: Colour){
        if !self.is_in_area(&position){
            panic!("Trying to change background color of wrong cell in wrong area")
        }
        let mut pos = position;
        self.convert_to_local_coordinate(&mut pos);
        let id = self.get_id_from_pos(pos);
        self.data[id] = Cell{
            content: self.data[id].content,
            fg: symbols::CURSOR.fg,
            bg: symbols::CURSOR.bg
        }

    }

    // calculate id of cell inside data from given local position
    fn get_id_from_pos(&self, pos: Vector2) -> usize{
        (pos.y * self.size.x + self.size.x )as usize
    }

}