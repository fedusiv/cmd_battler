use super::map_element::MapElement;
use crate::common::description::TextDescription;
use crate::common::vector2::Vector2;
use crate::core::config;
use std::collections::LinkedList;

pub struct Core {
    map_size: Vector2,
    map_content: Vec<Vec<MapElement>>,
    map_changes: LinkedList<MapElement>, // holds changes between previous update call and current
}

impl Core {
    pub fn new() -> Core {
        Core {
            map_size: config::MAP_SIZE,
            map_content: vec![
                vec![MapElement::new(); config::MAP_SIZE.x as usize];
                config::MAP_SIZE.y as usize
            ],
            map_changes: LinkedList::new(),
        }
    }

    pub fn start(&mut self) {
        // generating data of map, after that need to fill map changes
        let mut pos = Vector2::default();
        for row in self.map_content.iter_mut() {
            for element in row.iter_mut() {
                element.position = pos;
                pos.x += 1;
                self.map_changes.push_back(element.clone());
            }
            pos.x = 0;
            pos.y += 1;
        }
    }
    pub fn update(&mut self) {
        self.map_changes.clear(); // clear in the beggining
    }

    pub fn map_changes(&self) -> &LinkedList<MapElement> {
        &self.map_changes
    }

    fn map_element(&self, position: Vector2) -> MapElement {
        self.map_content[position.y as usize][position.x as usize]
    }

    pub fn under_cursor_information(&self, position: Vector2) -> TextDescription {
        let element = self.map_element(position);
        element.text_of_ground()
    }
}
