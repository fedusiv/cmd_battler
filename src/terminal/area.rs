use crate::utils::Vector2;

pub struct Area2D{
    pub width: u16,
    pub height: u16,
    
    pub start_point: Vector2, // from where start to draw

    pub visible: bool,  // flag represent, that does this area should be drawn (or displayed)

    pub data: Vec<char>
}

impl Area2D{
    pub fn new(w: u16, h: u16, s_point: Vector2) -> Area2D{
        let data = vec![' '; w as usize * h as usize];
        Area2D{
            width: w,
            height: h,
            start_point: s_point,
            visible: false,
            data
        }
    }

    pub fn fill_with_data(&mut self, symbol: char){
        for i in 0..self.data.len(){
            self.data[i] = symbol;
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

    // get character value of given position inside area
    pub fn get_symbol(&self, point: &Vector2) -> char{
        let area_point = Vector2{y: point.y - self.start_point.y, x: point.x - self.start_point.x};
        let index = ( self.width * area_point.y + area_point.x ) as usize;
        self.data[index]
    }
}