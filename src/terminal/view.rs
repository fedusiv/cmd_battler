use crate::utils::Vector2;
use std::collections::LinkedList;

use super::{
    backend,
    cell::{Cell, CellDraw},
    cursor::{self, Cursor, CursorMoves},
    parameters,
    rect::Rect,
};

pub struct View {
    area: Rect,
    cursor: Cursor,
    buffer: Vec<Cell>,
    window_size: Vector2,
}

impl View {
    pub fn create() -> View {
        let area = Rect::new(
            parameters::BATTLE_AREA_SIZE,
            Vector2 { x: 2, y: 2 }, // from where to draw this rect in global coordinates
            Vector2 { x: 1, y: 1 }, // where will be located first cursor position in local coordinates
            "Area".to_string(),
        );
        View {
            area,
            cursor: Cursor::default(),
            buffer: vec![Cell::default(); parameters::WINDOW_SIZE_US],
            window_size: Vector2::default(),
        }
    }

    pub fn init(&mut self) {
        self.cursor.position = self.area.get_cursor_pos();
        // need to init last used content for cursor, basically transfer to cursor pointer to data, from where to take values to return backwards when cursor is moving
        if let Some(pnt) = self.area.content_pointer(&self.cursor.position) {
            self.cursor.last_content = pnt;
            panic!("No content of cell in view init!");
        }
        self.window_size = parameters::WINDOW_SIZE;
    }

    // Main function of drawing.
    // Here decided in which zone this point is taking place and return cell value if it should be redrawn
    // position in global coordinates
    pub fn is_point_need_to_draw(&self, position: Vector2, element: Cell) -> Option<Cell> {
        let mut cell: Option<Cell> = None;
        // first parse area
        if let Some(c) = self.area.content(&position) {
            if c != element {
                cell = Some(c);
            }
        }
        // todo()! another views
        cell
    }

    fn get_cursor_view(&self) -> &mut Rect {
        let mut view = match self.cursor.view {
            cursor::Zones::Area => self.area,
        };
        &mut view
    }

    // Making chages to a cell where cursor is located
    pub fn cursor_apply_cell(&self) {
        // chose current view, where cursor is located
        let view = self.get_cursor_view();
        self.cursor.change_cell_view(view);
    }

    pub fn move_cursor(&mut self, direction: CursorMoves) {
        let current_view = self.get_cursor_view();
        let move_vector = match direction {
            CursorMoves::Up => Vector2 { x: 0, y: 1 },
            CursorMoves::Down => Vector2 { x: 0, y: -1 },
            CursorMoves::Left => Vector2 { x: -1, y: 0 },
            CursorMoves::Right => Vector2 { x: 1, y: 0 },
        };
        let destination = self.cursor.position + move_vector;
    }

    // Making draw
    pub fn draw(&mut self) {
        self.cursor_apply_cell(); // made changes to cell where cursor takes place
                                  // create list with elements, which should be drawn
        let mut draw_list: LinkedList<CellDraw> = LinkedList::new();
        // iterate through current buffer of all cell data
        // if it is not equal, need to draw and save
        for y in 0..self.window_size.y {
            // for representative used two loops as x and y coordinates
            for x in 0..self.window_size.x {
                let current_id = (y * self.window_size.x + x) as usize;
                let point = Vector2 { x, y }; // curent point
                let element = self.buffer[current_id];
                // Check does this position need to be redrawn
                if let Some(new_cell) = self.is_point_need_to_draw(point, element) {
                    self.buffer[current_id] = new_cell.clone();
                    let cell = CellDraw {
                        cell: self.buffer[current_id],
                        point,
                    };
                    draw_list.push_back(cell);
                }
                // if point is not in anyzone, definitely we do not need to redraw it
            }
        }
        backend::draw(draw_list);
    }
}
