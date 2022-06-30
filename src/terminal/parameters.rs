use crate::utils::Vector2;

pub const INPUT_POLLING_TIMEOUT: u64 = 100; // ms

pub const WINDOW_DRAW_TIME: u128 = 100; // ms
pub const WINDOW_SIZE: Vector2 = Vector2 { x: 92, y: 32 };
pub const WINDOW_SIZE_US: usize = WINDOW_SIZE.x as usize * WINDOW_SIZE.y as usize;

pub const BATTLE_AREA_SIZE: Vector2 = Vector2 { x: 62, y: 22 };
