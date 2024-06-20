use bevy::prelude::*;

#[derive(Bundle, Debug, Default)]
pub(crate) struct UiText {
    pub(crate) text: TextBundle,
}

impl UiText {
    pub(crate) fn new(section: &str) -> Self {
        Self {
            text: TextBundle::from_section(section, Self::default_style()),
        }
    }

    pub(crate) fn text_style(mut self, f: fn(&mut TextStyle) -> ()) -> Self {
        f(&mut self.text.text.sections.first_mut().unwrap().style);
        self
    }

    pub(crate) fn default_style() -> TextStyle {
        TextStyle {
            font: default(),
            font_size: 24.,
            color: Color::rgb(0.8, 0.8, 0.8),
        }
    }
}
