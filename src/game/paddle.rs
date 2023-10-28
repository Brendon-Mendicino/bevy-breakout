use bevy::{math::*, prelude::*};

use super::*;

#[derive(Component, Clone)]
pub struct Paddle;

impl Paddle {
    pub const START: Vec2 = vec2(0., -250.);
    pub const SIZE: Vec2 = vec2(120., 20.);
    pub const ENLARGED_SIZE: Vec2 = vec2(240.0, 20.);
    pub const COLOR: Color = Color::rgb(0.3, 0.3, 0.7);
    pub const SPEED: f32 = 500.0;
    pub const TIMEOUT: f32 = 10.0;
}

#[derive(Bundle, Clone)]
pub struct PaddleBundle {
    pub paddle: Paddle,
    pub collider: Collider,
    pub sprite: SpriteBundle,
}

impl Default for PaddleBundle {
    fn default() -> Self {
        Self {
            paddle: Paddle,
            collider: Collider { size: Paddle::SIZE },
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

pub struct PaddlePlugin;

impl Plugin for PaddlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (move_paddle, handle_paddle_timer).run_if(in_state(AppState::Game)),
        );
    }
}

fn move_paddle(
    mut query: Query<(&mut Transform, &Collider), With<Paddle>>,
    input: Res<Input<KeyCode>>,
    main_box: Res<MainBox>,
    time_step: Res<FixedTime>,
) {
    let (mut transform, collider) = query.single_mut();
    let mut direction = 0.0;

    if input.pressed(KeyCode::Left) {
        direction += -1.0;
    }

    if input.pressed(KeyCode::Right) {
        direction += 1.0;
    }

    direction = direction * Paddle::SPEED * time_step.period.as_secs_f32();

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
