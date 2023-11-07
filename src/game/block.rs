use bevy::{math::vec2, prelude::*};

use super::*;

pub struct BlockPlugin;

impl Plugin for BlockPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), setup_block)
            .add_systems(
                Update,
                (spawn_new_blocks.after(block_go_down)).run_if(in_state(AppState::Game)),
            )
            .add_systems(OnExit(AppState::Game), cleanup_block);
    }
}

#[derive(Component, Clone)]
pub struct Block;

#[derive(Bundle, Clone)]
pub struct BlockBundle {
    block: Block,
    health: Health,
    collider: Collider,
    sprite: SpriteBundle,
}

#[derive(Resource, Clone, Deref, DerefMut)]
pub struct BlockGoDown(pub Timer);

#[derive(Resource, Default, Clone)]
pub struct BlockSpawn {
    pub go_down_counter: u32,
}

impl Block {
    // Blocks
    pub const HEIGHT: i32 = 9;
    pub const WIDTH: i32 = 12;
    pub const SIZE: Vec2 = vec2(
        ((WALL_WIDTH - WALL_THICKNESS - Self::PADDING) / Self::WIDTH as f32) - Self::PADDING,
        25.0,
    );
    pub const PADDING: f32 = 5.0;

    pub const GO_DOWN_TIMEOUT: Duration = Duration::from_secs(10);

    pub const SPAWN_AFTER_GO_DOWN: u32 = 2;
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
        Self(Timer::new(Block::GO_DOWN_TIMEOUT, TimerMode::Repeating))
    }
}

fn setup_block(mut commands: Commands) {
    commands.init_resource::<BlockGoDown>();
    commands.init_resource::<BlockSpawn>();
}

fn cleanup_block(mut commands: Commands) {
    commands.remove_resource::<BlockGoDown>();
    commands.remove_resource::<BlockSpawn>();
}

pub fn block_go_down(
    mut timer: ResMut<BlockGoDown>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Block>>,
) {
    timer.tick(time.delta());

    if !timer.just_finished() {
        return;
    }

    for mut transform in &mut query {
        transform.translation.y -= (Block::SIZE.y + Block::PADDING) as f32;
    }
}

pub fn spawn_new_blocks(
    mut commands: Commands,
    mut block_spawn: ResMut<BlockSpawn>,
    go_down: Res<BlockGoDown>,
    main_box: Res<MainBox>,
) {
    if !go_down.just_finished() {
        return;
    }

    block_spawn.go_down_counter = (block_spawn.go_down_counter + 1) % Block::SPAWN_AFTER_GO_DOWN;

    let spawn = block_spawn.go_down_counter == 0;
    if !spawn {
        return;
    }

    commands.spawn_batch(spawn_column(main_box));
}

fn spawn_column(main_box: Res<MainBox>) -> Vec<BlockBundle> {
    let mut blocks = Vec::with_capacity(Block::HEIGHT as usize);
    let mut wall_top_right = 0.5 * vec3(-main_box.size.x, main_box.size.y, 0.0);
    // Add a half of a block size
    wall_top_right += vec3(
        Block::SIZE.x * 0.5 + Block::PADDING,
        -(Block::SIZE.y * 0.5 + Block::PADDING),
        0.0,
    );

    for w in 0..Block::WIDTH {
        let mut pos = vec3(w as f32 * (Block::SIZE.x + Block::PADDING), 0.0, 0.0);

        pos += wall_top_right;

        blocks.push(BlockBundle::from_translation(pos));
    }

    blocks
}
