use crate::common::description::{self, TextDescription};
use crate::common::vector2::Vector2;

#[derive(Copy, Clone)]
pub enum GroundElement {
    Grass,
    Stone,
}

#[derive(Copy, Clone)]
pub struct MapElement {
    pub ground: GroundElement,
    pub position: Vector2,
}

impl MapElement {
    pub fn new() -> MapElement {
        MapElement {
            ground: GroundElement::Grass,
            position: Vector2::default(),
        }
    }

    pub fn text_of_ground(&self) -> TextDescription {
        match self.ground {
            GroundElement::Grass => description::GROUND_GRASS_TEXT,
            GroundElement::Stone => description::GROUND_STONE_TEXT,
        }
    }
}
