use crate::utils::Vector2;

pub const WINDOW_DRAW_TIME: u128 = 200;
pub const WINDOW_SIZE: Vector2= Vector2 {x:60, y:30};
pub const WINDOW_SIZE_US: usize = WINDOW_SIZE.x as usize * WINDOW_SIZE.y as usize;