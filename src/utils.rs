use crate::terminal::cell::Colour;
use std::ops::{Add, Sub};

use super::terminal::cell::Cell;

pub type Vector2Int = i16;

#[derive(Default, Copy, Clone)]
pub struct Vector2 {
    pub x: Vector2Int,
    pub y: Vector2Int,
}

impl Vector2 {
    // return true if vector is less than given vector
    pub fn lt(&self, compare: Vector2) -> bool {
        if self.x < compare.x || self.y < compare.y {
            return true;
        }
        false
    }
}

impl Add for Vector2 {
    type Output = Self;
    fn add(self, vector: Self) -> Self::Output {
        Self {
            x: self.x + vector.x,
            y: self.y + vector.y,
        }
    }
}

impl Sub for Vector2 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl PartialEq for Vector2 {
    fn eq(&self, other: &Self) -> bool {
        if (self.x == other.x) && (self.y == other.y) {
            return true;
        }
        false
    }
}

pub fn create_char_cell(c: char) -> Cell {
    Cell {
        content: c,
        fg: Colour::White,
        bg: Colour::Black,
    }
}
