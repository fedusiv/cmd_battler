pub struct Vector2{
    pub x: u16,
    pub y: u16
}


impl Vector2{
    // Validate, that current vector is greater or equal
    pub fn goe(&self, compare: Vector2) -> bool{
        if compare.x >= self.x && compare.y >= self.y {
            return true;
        }
        false
    }

    // return true if vector is less than given vector
    pub fn lt(&self, compare: Vector2) -> bool{
        if self.x < compare.x || self.y < compare.y {
            return true;
        }
        false
    }
}
