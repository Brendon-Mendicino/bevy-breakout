use bevy::{math::vec2, prelude::*};

use super::*;

#[derive(Component, Clone)]
pub struct Block;

#[derive(Bundle, Clone)]
pub struct BlockBundle {
    block: Block,
    health: Health,
    collider: Collider,
    sprite: SpriteBundle,
}

#[derive(Resource, Clone)]
pub struct BlockGoDown {
    timer: Timer,
}

impl Block {
    // Blocks
    pub const HEIGHT: i32 = 10;
    pub const WIDTH: i32 = 12;
    pub const SIZE: Vec2 = vec2(
        ((WALL_WIDTH - WALL_THICKNESS - Self::PADDING) / Self::WIDTH as f32) - Self::PADDING,
        25.0,
    );
    pub const PADDING: f32 = 5.0;

    pub const GO_DOWN_TIMEOUT: Duration = Duration::from_secs(10);
}

impl Default for BlockBundle {
    fn default() -> Self {
        Self {
            block: Block,
            health: Health(1),
            collider: Collider { size: Block::SIZE },
            sprite: SpriteBundle {
                transform: Transform::default(),
                sprite: Sprite {
                    color: Color::NAVY,
                    custom_size: Some(Block::SIZE),
                    ..default()
                },
                ..default()
            },
        }
    }
}

impl BlockBundle {
    pub fn from_translation(translation: Vec3) -> Self {
        Self {
            sprite: {
                let mut sprite = Self::default().sprite;
                sprite.transform.translation = translation;
                sprite
            },
            ..default()
        }
    }
}

impl Default for BlockGoDown {
    fn default() -> Self {
        Self {
            timer: Timer::new(Block::GO_DOWN_TIMEOUT, TimerMode::Repeating),
        }
    }
}

pub struct BlockPlugin;

impl Plugin for BlockPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), setup_block)
            .add_systems(OnExit(AppState::Game), cleanup_block);
    }
}

fn setup_block(mut commands: Commands) {
    commands.init_resource::<BlockGoDown>();
}

fn cleanup_block(mut commands: Commands) {
    commands.remove_resource::<BlockGoDown>();
}

pub fn block_go_down(
    mut timer: ResMut<BlockGoDown>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Block>>,
) {
    timer.timer.tick(time.delta());

    if !timer.timer.just_finished() {
        return;
    }

    for mut transform in &mut query {
        transform.translation.y -= (Block::SIZE.y + Block::PADDING) as f32;
    }
}
