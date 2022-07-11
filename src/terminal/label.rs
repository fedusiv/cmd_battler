use crate::common::{description::TextDescription, vector2::Vector2};

use super::cell::Cell;

#[derive(Clone)]
pub struct Label {
    pub position: Vector2, // logic position in rect
    pub content: Vec<Cell>,
}

impl Label {
    pub fn new(position: Vector2, length: u32) -> Label {
        Label {
            position,
            content: vec![Cell::default(); length as usize],
        }
    }

    pub fn set_text(&mut self, text: TextDescription) {
        let string: Vec<char> = text.text.into_owned().chars().collect();
        for i in 0..self.content.len() {
            let content: char;
            if i >= string.len(){
                content = ' ';
            }
            else{
                content = string[i as usize];
            }
            self.content[i as usize].content = content;
            self.content[i as usize].fg = text.color;
        }
    }
}
