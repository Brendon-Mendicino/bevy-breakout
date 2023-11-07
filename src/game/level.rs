use bevy::prelude::*;

#[derive(Component, Clone, Default)]
pub struct Level {
    pub level: u32,
    pub exp: u32,
}
