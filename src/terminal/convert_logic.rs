use super::cell::Cell;
use super::symbols;
use crate::core::map_element::{GroundElement, MapElement};

pub fn convert_logic_to_gui(element: &MapElement) -> Cell {
    let mut cell = Cell::default();

    cell.bg = match element.ground {
        GroundElement::Grass => symbols::GROUND_GRASS.bg,
        GroundElement::Stone => symbols::GROUND_STONE.bg,
    };

    cell
}
