use std::borrow::Cow;

use crate::common::colour::Colour;

pub struct TextDescription {
    pub text: Cow<'static, str>,
    pub color: Colour,
}

pub const GROUND_LABEL_TEXT: TextDescription = TextDescription {
    text: Cow::Borrowed("layout"),
    color: Colour::Gray,
};
pub const GROUND_GRASS_TEXT: TextDescription = TextDescription {
    text: Cow::Borrowed("grass"),
    color: Colour::Green,
};
pub const GROUND_STONE_TEXT: TextDescription = TextDescription {
    text: Cow::Borrowed("stone"),
    color: Colour::Gray,
};
