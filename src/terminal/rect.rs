use crate::utils::Vector2;
use crate::terminal::cell::Cell;

pub struct Rect{
    pub width: u16,
    pub height: u16,
    
    pub start_point: Vector2, // from where start to draw

    pub visible: bool,  // flag represent, that does this area should be drawn (or displayed)

    pub data: Vec<Cell>
}

impl Rect{
    pub fn new(w: u16, h: u16, s_point: Vector2) -> Rect{
        let data = vec![Cell::default(); w as usize * h as usize];
        Rect{
            width: w,
            height: h,
            start_point: s_point,
            visible: false,
            data
        }
    }

    // check does given point take place in current area
    pub fn is_in_area(&self, point: &Vector2) -> bool{
        // maybe so straight forward check, but ok
        if (point.x >= self.start_point.x) && (point.x < self.start_point.x + self.width){
            if (point.y >= self.start_point.y) && (point.y < self.start_point.y + self.height)
            {
                return true;
            }
        }
        false
    }

    pub fn fill_all_cells(&mut self, cell: Cell){
        for i in 0..self.data.len(){
            self.data[i] = cell;
        }

    }

    // This function works without verification because expect to be run is_in_area first
    pub fn content_unsafe(&self, point: &Vector2) -> Cell{
            let area_point = Vector2{y: point.y - self.start_point.y, x: point.x - self.start_point.x};
            let index = ( self.width * area_point.y + area_point.x ) as usize;
            self.data[index]
    }

    // converting position of given point inside rect to position in global rect
    pub fn convert_to_global_coor(&self, point: &mut Vector2){
        *point = *point + self.start_point;
    }

}