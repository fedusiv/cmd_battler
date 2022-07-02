use crate::utils::Vector2;

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
}
