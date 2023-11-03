use std::time::Duration;

use bevy::{math::*, prelude::*};

use crate::AppState;

use super::*;

#[derive(Component, Clone)]
pub struct Ball {
    pub size: Vec2,
}

impl Ball {
    pub const COLOR: Color = Color::rgb(0.9, 0.4, 0.2);
    pub const START: Vec2 = vec2(-70.0, 1.0);
    pub const SIZE: Vec2 = vec2(30.0, 30.0);
    pub const ENLARGED_SIZE: Vec2 = vec2(60.0, 60.0);
    pub const SPEED: f32 = 400.0;
    pub const DIRECTION: Vec2 = vec2(0.5, -0.5);
    pub const TIMEOUT: f32 = 10.0;
}

#[derive(Resource, Default, Deref, DerefMut)]
pub struct BallCollision(pub Handle<AudioSource>);

#[derive(Bundle, Clone)]
pub struct BallBundle {
    pub ball: Ball,
    pub velocity: Velocity,
    pub attack: Attack,
    pub sprite: SpriteBundle,
}

impl BallBundle {
    pub fn from_trans_vel(transform: Transform, velocity: Velocity) -> Self {
        Self {
            velocity,
            sprite: SpriteBundle {
                transform,
                sprite: Sprite {
                    color: Ball::COLOR,
                    custom_size: Some(Ball::SIZE),
                    ..default()
                },
                ..default()
            },
            ..default()
        }
    }
}

impl Default for BallBundle {
    fn default() -> Self {
        Self {
            ball: Ball { size: Ball::SIZE },
            attack: Attack(1),
            velocity: Velocity(Ball::SPEED * Ball::DIRECTION),
            sprite: SpriteBundle {
                transform: Transform::from_translation(Ball::START.extend(0.0)),
                sprite: Sprite {
                    color: Ball::COLOR,
                    custom_size: Some(Ball::SIZE),
                    ..default()
                },
                ..default()
            },
        }
    }
}

#[derive(Resource)]
pub struct BallEnlargmentTimer {
    pub timer: Timer,
}

impl Default for BallEnlargmentTimer {
    fn default() -> Self {
        Self {
            timer: Timer::new(Duration::from_secs_f32(Ball::TIMEOUT), TimerMode::Once),
        }
    }
}

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ball)
            .add_systems(Update, (handle_ball_timer).run_if(in_state(AppState::Game)));
    }
}

fn setup_ball(mut commands: Commands, asset_server: Res<AssetServer>) {
    let sound = asset_server.load("audio/ball_bounce.ogg");
    commands.insert_resource(BallCollision(sound));
}

fn handle_ball_timer(
    mut commands: Commands,
    timer: Option<ResMut<BallEnlargmentTimer>>,
    time: Res<Time>,
    mut query: Query<(&mut Sprite, &mut Ball)>,
) {
    let Some(mut timer) = timer else { return };

    timer.timer.tick(time.delta());

    if !timer.timer.finished() {
        return;
    }

    for (mut sprite, mut collider) in &mut query {
        sprite.custom_size = Some(Ball::SIZE);
        collider.size = Ball::SIZE;
    }

    commands.remove_resource::<BallEnlargmentTimer>();
}
