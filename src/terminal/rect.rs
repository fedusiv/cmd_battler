use crate::terminal::cell::Cell;
use crate::utils::{self, Vector2};

use super::cell::Colour;
use super::symbols;

#[derive(Clone)]
pub struct Rect {
    pub size: Vector2,
    name: String,
    pub start_point: Vector2, // from where start to draw
    last_cursor_pos: Vector2, // logic postion of point where should be located cursor

    pub visible: bool, // flag represent, that does this area should be drawn (or displayed)

    pub data: Vec<Cell>,
    pub logic_positions: Vec<Vec<Vector2>>,
}

impl Rect {
    pub fn new(size: Vector2, s_point: Vector2, c_point: Vector2, name: String) -> Rect {
        let data = vec![Cell::default(); size.x as usize * size.y as usize];
        let mut rect = Rect {
            size,
            name,
            start_point: s_point,
            last_cursor_pos: c_point,
            visible: false,
            data,
        };
        rect.init_borders();
        rect
    }

    // Set border symbols to required places.
    fn init_borders(&mut self) {
        let height = self.size.y;
        let width = self.size.x;

        for y in 0..height {
            let mut x = 0;
            while x < width {
                // need while loop because we need to write a name, look down and you will understand

                let point = Vector2 { x, y };
                let mut id = self.get_id_from_pos(point);

                // Corners
                if point == (Vector2 { x: 0, y: 0 }) {
                    self.data[id] = symbols::BORDER_LEFT_UP_CORNER;
                } else if point == (Vector2 { x: width - 1, y: 0 }) {
                    self.data[id] = symbols::BORDER_RIGHT_UP_CORNER;
                } else if point
                    == (Vector2 {
                        x: 0,
                        y: height - 1,
                    })
                {
                    self.data[id] = symbols::BORDER_LEFT_BOTTOM_CORNER;
                } else if point
                    == (Vector2 {
                        x: width - 1,
                        y: height - 1,
                    })
                {
                    self.data[id] = symbols::BORDER_RIGHT_BOTTOM_CORNER;
                }
                // vertical borders
                else if (point.x == 0) || (point.x == width - 1) {
                    self.data[id] = symbols::BORDER_VERTICAL;
                }
                // Name of Rect in Horizontal upper border
                else if point == (Vector2 { x: 2, y: 0 }) {
                    for c in self.name.chars() {
                        self.data[id] = utils::create_char_cell(c);
                        x += 1;
                        id += 1;
                    }
                    continue; // to avoid incrementing x again in the end of loop
                }
                // horizontal borders
                else if (point.y == 0) || (point.y == height - 1) {
                    self.data[id] = symbols::BORDER_HORIZONTAL;
                }
                // empty cell
                else {
                    self.data[id] = symbols::EMPTY_CELL;
                }
                x += 1;
            }
        }
    }

    // check does given point take place in current area
    // position in global coordinates
    pub fn is_in_area(&self, point: &Vector2) -> bool {
        // maybe so straight forward check, but ok
        if (point.x >= self.start_point.x) && (point.x < self.start_point.x + self.size.x) {
            if (point.y >= self.start_point.y) && (point.y < self.start_point.y + self.size.y) {
                return true;
            }
        }
        false
    }

    // get content of required global position
    pub fn content(&self, point: &Vector2) -> Option<Cell> {
        if self.is_in_area(point) {
            let local_pos = self.convert_to_local_coordinate_return(point);
            let id = self.get_id_from_pos(local_pos);
            Some(self.data[id])
        } else {
            None
        }
    }

    // get pointer of content inside rect by global position
    pub fn content_pointer(&self, point: &Vector2) -> Option<*const Cell> {
        if self.is_in_area(point) {
            let local_pos = self.convert_to_local_coordinate_return(point);
            let id = self.get_id_from_pos(local_pos);
            Some(&(self.data[id]))
        } else {
            None
        }
    }

    // converting position of given point inside rect to position in global rect
    // position in local coordinates
    #[allow(dead_code)]
    pub fn convert_to_global_coor(&self, point: &mut Vector2) {
        *point = *point + self.start_point;
    }
    // converting position of given point inside rect to position in global rect
    // position in local coordinates
    pub fn convert_to_global_coor_return(&self, point: &Vector2) -> Vector2 {
        let pos = *point + self.start_point;
        pos
    }
    // converting position of given position in global coordinates to local coordinates
    pub fn convert_to_local_coordinate(&self, point: &mut Vector2) {
        *point = *point - self.start_point;
    }
    // converting position of given position in global coordinates to local coordinates
    pub fn convert_to_local_coordinate_return(&self, point: &Vector2) -> Vector2 {
        let pos = *point - self.start_point;
        pos
    }

    // position in global coordinates
    pub fn change_cell_data(
        &mut self,
        position: Vector2,
        content: Option<char>,
        fg: Option<Colour>,
        bg: Option<Colour>,
    ) {
        if !self.is_in_area(&position) {
            panic!("Trying to change background color of wrong cell in wrong area")
        }
        let mut pos = position;
        self.convert_to_local_coordinate(&mut pos);
        let id = self.get_id_from_pos(pos);
        // Change content
        if let Some(c) = content {
            self.data[id].content = c;
        }
        if let Some(color) = fg {
            self.data[id].fg = color;
        }
        if let Some(color) = bg {
            self.data[id].bg = color;
        }
    }

    // calculate id of cell inside data from given local position
    fn get_id_from_pos(&self, pos: Vector2) -> usize {
        (pos.y * self.size.x + pos.x) as usize
    }

    // When cursor moving to other rect, need to get position of starting point for cursor
    // return position in global coordinates
    pub fn get_cursor_pos(&self) -> Vector2 {
        self.convert_to_global_coor_return(&self.last_cursor_pos)
    }
}
