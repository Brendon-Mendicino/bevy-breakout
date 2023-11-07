use bevy::{math::*, prelude::*};

use super::{level::Level, *};

pub struct PaddlePlugin;

impl Plugin for PaddlePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LevelUp>().add_event::<ExpUp>().add_systems(
            Update,
            (move_paddle, handle_paddle_timer, level_up).run_if(in_state(AppState::Game)),
        );
    }
}

#[derive(Component, Clone)]
pub struct Paddle;

#[derive(Event)]
pub struct LevelUp(pub u32);

#[derive(Event, Deref, DerefMut)]
pub struct ExpUp(pub u32);

impl Paddle {
    pub const START: Vec2 = vec2(0., -250.);
    pub const SIZE: Vec2 = vec2(120., 20.);
    pub const ENLARGED_SIZE: Vec2 = vec2(240.0, 20.);
    pub const COLOR: Color = Color::rgb(0.3, 0.3, 0.7);
    pub const SPEED: f32 = 500.0;
    pub const TIMEOUT: f32 = 10.0;

    pub const LEVEL_UP_MULTIPLYER: f32 = 1.5;
    pub const LEVEL_CAP: u32 = 50;
}

#[derive(Bundle, Clone)]
pub struct PaddleBundle {
    pub paddle: Paddle,
    pub collider: Collider,
    pub level: Level,
    pub sprite: SpriteBundle,
}

impl Default for PaddleBundle {
    fn default() -> Self {
        Self {
            paddle: Paddle,
            collider: Collider { size: Paddle::SIZE },
            level: Level { level: 0, exp: 0 },
            sprite: SpriteBundle {
                transform: Transform {
                    translation: Paddle::START.extend(0.0),
                    ..default()
                },
                sprite: Sprite {
                    color: Paddle::COLOR,
                    custom_size: Some(Paddle::SIZE),
                    ..default()
                },
                ..default()
            },
        }
    }
}

#[derive(Resource)]
pub struct PaddleEnlargedTimer {
    pub timer: Timer,
}

impl Default for PaddleEnlargedTimer {
    fn default() -> Self {
        Self {
            timer: Timer::new(Duration::from_secs_f32(Paddle::TIMEOUT), TimerMode::Once),
        }
    }
}

fn move_paddle(
    mut query: Query<(&mut Transform, &Collider), With<Paddle>>,
    input: Res<Input<KeyCode>>,
    main_box: Res<MainBox>,
    time: Res<Time>,
) {
    let (mut transform, collider) = query.single_mut();
    let mut direction = 0.0;

    if input.pressed(KeyCode::Left) {
        direction += -1.0;
    }

    if input.pressed(KeyCode::Right) {
        direction += 1.0;
    }

    direction = direction * Paddle::SPEED * time.delta_seconds();

    transform.translation.x += direction;

    transform.translation.x = transform.translation.x.clamp(
        -(main_box.size.x - collider.size.x) * 0.5,
        (main_box.size.x - collider.size.x) * 0.5,
    );
}

fn handle_paddle_timer(
    mut commands: Commands,
    timer: Option<ResMut<PaddleEnlargedTimer>>,
    time: Res<Time>,
    mut query: Query<(&mut Sprite, &mut Collider), With<Paddle>>,
) {
    let Some(mut timer) = timer else { return };

    timer.timer.tick(time.delta());

    if !timer.timer.finished() {
        return;
    }

    let (mut sprite, mut collider) = query.single_mut();
    sprite.custom_size = Some(Paddle::SIZE);
    collider.size = Paddle::SIZE;

    commands.remove_resource::<PaddleEnlargedTimer>();
}

fn level_up(
    mut level_up: EventWriter<LevelUp>,
    mut exp_up: EventReader<ExpUp>,
    mut query: Query<&mut Level, With<Paddle>>,
) {
    if exp_up.is_empty() {
        return;
    }

    let mut level = query.single_mut();
    for exp in exp_up.read() {
        level.exp += exp.0;
    }

    let level_cap = level_exp_cap(level.level);

    if level.exp > level_cap {
        level.exp -= level_cap;
        level.level += 1;
        level_up.send(LevelUp(level.level));
    }
}

pub fn level_exp_cap(level: u32) -> u32 {
    (Paddle::LEVEL_CAP as f32 * (1. + level as f32 * Paddle::LEVEL_UP_MULTIPLYER)) as u32
}
