use bevy::prelude::*;

#[derive(Bundle, Default, Debug)]
pub(crate) struct Spacer {
    pub(crate) node: NodeBundle,
}

impl Spacer {
    pub(crate) fn around(val: Val) -> Self {
        Self {
            node: NodeBundle {
                style: Style {
                    margin: UiRect::all(val),
                    ..default()
                },
                ..default()
            },
        }
    }

    pub(crate) fn style(mut self, f: fn(&mut Style) -> ()) -> Self {
        f(&mut self.node.style);
        self
    }
}
