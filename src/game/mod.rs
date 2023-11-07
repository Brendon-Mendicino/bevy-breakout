use std::time::Duration;

use crate::AppState;
use bevy::{math::*, prelude::*, sprite::collide_aabb::*};
use powerup::*;
use scoreboard::*;

use self::ball::{Ball, BallBundle, BallCollision, BallEnlargmentTimer, BallPlugin};
use self::block::{block_go_down, Block, BlockBundle, BlockPlugin};
use self::dmg_text::{spawn_dmg_text, DmgTextPlugin};
use self::paddle::{Paddle, PaddleBundle, PaddleEnlargedTimer, PaddlePlugin};

mod ball;
mod block;
mod dmg_text;
mod paddle;
mod powerup;
mod scoreboard;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), setup_game)
            .add_plugins((
                PowerupPlugin,
                BallPlugin,
                PaddlePlugin,
                BlockPlugin,
                DmgTextPlugin,
                ScoreboardPlugin,
            ))
            .add_systems(Update, (bevy::window::close_on_esc,))
            .add_systems(
                FixedUpdate,
                (
                    block_go_down,
                    apply_velocity,
                    (
                        check_ball_collision,
                        check_ball_out_of_bound,
                        check_powerups_collision,
                    )
                        .after(apply_velocity)
                        .after(block_go_down),
                    check_game_over.after(check_ball_out_of_bound),
                    check_game_won.after(check_ball_out_of_bound),
                )
                    .run_if(in_state(AppState::Game)),
            )
            .add_systems(
                OnExit(AppState::Game),
                (
                    cleanup_component::<Paddle>,
                    cleanup_component::<Block>,
                    cleanup_component::<Powerup>,
                    cleanup_component::<Wall>,
                    cleanup_component::<Ball>,
                    cleanup_component::<Text>,
                ),
            );
    }
}

// Wall
const WALL_WIDTH: f32 = 1200.0;
const WALL_HEIGHT: f32 = 600.0;
const WALL_THICKNESS: f32 = 30.0;

#[derive(Component, Clone, Deref, DerefMut)]
pub struct Velocity(pub Vec2);

#[derive(Component, Clone, Deref, DerefMut)]
pub struct Health(pub u32);

#[derive(Component, Clone, Deref, DerefMut)]
pub struct Attack(pub u32);

#[derive(Component, Clone)]
pub struct Collider {
    pub size: Vec2,
}

#[derive(Component)]
struct GameEntities;

#[derive(Component, Deref, DerefMut)]
pub struct PlayerCollider(pub Collider);

#[derive(Component, Clone)]
pub struct Wall;

#[derive(Bundle)]
struct WallBundle {
    sprite: SpriteBundle,
    collider: Collider,
    wall: Wall,
}

#[derive(Resource, Clone, Copy)]
pub struct MainBox {
    pub size: Vec2,
}

fn setup_game(mut commands: Commands) {
    // Paddle
    commands.spawn(PaddleBundle::default());

    // Ball
    commands.spawn(BallBundle::default());

    // Walls
    let top_bot = vec2(WALL_WIDTH, WALL_THICKNESS);
    let left_right = vec2(WALL_THICKNESS, WALL_HEIGHT);
    let walls = [
        (
            top_bot + vec2(left_right[0], 0.0),
            vec2(0.0, 0.5 * left_right[1]),
        ),
        (
            top_bot + vec2(left_right[0], 0.0),
            vec2(0.0, -0.5 * left_right[1]),
        ),
        (
            left_right + vec2(0.0, top_bot[1]),
            vec2(0.5 * top_bot[0], 0.0),
        ),
        (
            left_right + vec2(0.0, top_bot[1]),
            vec2(-0.5 * top_bot[0], 0.0),
        ),
    ];

    for wall in walls {
        commands.spawn(WallBundle {
            sprite: SpriteBundle {
                transform: Transform {
                    translation: wall.1.extend(0.0),
                    ..default()
                },
                sprite: Sprite {
                    color: Color::SILVER,
                    custom_size: Some(wall.0),
                    ..default()
                },
                ..default()
            },
            collider: Collider { size: wall.0 },
            wall: Wall,
        });
    }

    let main_box = MainBox {
        size: vec2(
            WALL_WIDTH as f32 - WALL_THICKNESS,
            WALL_HEIGHT as f32 - WALL_THICKNESS,
        ),
    };
    commands.insert_resource(main_box);

    // Blocks
    spawn_blocks(&mut commands, main_box);
}

fn spawn_blocks(commands: &mut Commands, main_box: MainBox) {
    for w in 0..Block::WIDTH {
        for h in (0..Block::HEIGHT).step_by(2) {
            let mut pos = vec3(
                w as f32 * (Block::SIZE.x + Block::PADDING),
                -h as f32 * (Block::SIZE.y + Block::PADDING),
                0.0,
            );

            let mut wall_top_right = 0.5 * vec3(-main_box.size.x, main_box.size.y, 0.0);

            // Add a half of a block size
            wall_top_right += vec3(
                Block::SIZE.x * 0.5 + Block::PADDING,
                -(Block::SIZE.y * 0.5 + Block::PADDING),
                0.0,
            );

            pos += wall_top_right;

            commands.spawn(BlockBundle::from_translation(pos));
        }
    }
}

fn apply_velocity(mut query: Query<(&Velocity, &mut Transform)>, time: Res<Time>) {
    let dt = time.delta_seconds();
    for (velocity, mut transform) in &mut query {
        transform.translation.x += velocity.x * dt;
        transform.translation.y += velocity.y * dt;
    }
}

fn check_ball_collision(
    mut balls: Query<(&Transform, &mut Velocity, &Attack, &Ball)>,
    mut colliders: Query<(
        Entity,
        &Transform,
        &Collider,
        Option<&mut Health>,
        Option<&Block>,
        Option<&Paddle>,
    )>,
    collision_sound: Res<BallCollision>,
    mut scoreboard: ResMut<Scoreboard>,
    mut commands: Commands,
) {
    for (entity, transform, collider, mut health, block, paddle) in &mut colliders {
        for (ball_t, mut ball_v, attack, ball) in &mut balls {
            let collision = collide(
                ball_t.translation,
                ball.size,
                transform.translation,
                collider.size,
            );

            let Some(collision) = collision else { continue };

            let mut reflect_x = false;
            let mut reflect_y = false;
            let mut inside = false;
            match collision {
                Collision::Bottom => reflect_y = ball_v.y > 0.0,
                Collision::Top => reflect_y = ball_v.y < 0.0,
                Collision::Right => reflect_x = ball_v.x < 0.0,
                Collision::Left => reflect_x = ball_v.x > 0.0,
                Collision::Inside => inside = true,
            }

            // Play bounce sound
            commands.spawn(AudioBundle {
                source: collision_sound.clone(),
                settings: PlaybackSettings::DESPAWN,
            });

            if paddle.is_some() && !inside {
                let dir = ball_t.translation - transform.translation;
                ball_v.0 = dir.xy().normalize() * ball_v.length();
                break;
            }

            if reflect_x {
                ball_v.x = -ball_v.x;
            }
            if reflect_y {
                ball_v.y = -ball_v.y;
            }

            if block.is_some() {
                /* If the health is not zero continue with the ball iteration */
                let Some(ref mut health) = health else {
                    unreachable!()
                };

                spawn_dmg_text(&mut commands, ball_t.translation, **attack);

                if ***health > **attack {
                    ***health -= **attack;
                    continue;
                }

                **scoreboard += 1;
                commands.entity(entity).despawn();
                Powerup::spawn_powerup(&mut commands, ball_t.translation);
            }

            break;
        }
    }
}

fn duplicate_balls(
    commands: &mut Commands,
    query_ball: &Query<(&Velocity, &Transform, &mut Ball, &mut Sprite), Without<Paddle>>,
) {
    let balls = query_ball
        .into_iter()
        .map(|(v, t, _, _)| BallBundle::from_trans_vel(*t, Velocity(-v.0)))
        .collect::<Vec<_>>();

    commands.spawn_batch(balls);
}

fn enlarge_paddle(commands: &mut Commands, sprite: &mut Sprite, collider: &mut Collider) {
    sprite.custom_size = Some(Paddle::ENLARGED_SIZE);
    collider.size = Paddle::ENLARGED_SIZE;

    commands.insert_resource(PaddleEnlargedTimer::default());
}

fn enlarge_balls(
    commands: &mut Commands,
    query_ball: &mut Query<(&Velocity, &Transform, &mut Ball, &mut Sprite), Without<Paddle>>,
) {
    for (_, _, mut collider, mut sprite) in query_ball {
        sprite.custom_size = Some(Ball::ENLARGED_SIZE);
        collider.size = Ball::ENLARGED_SIZE;
    }

    commands.insert_resource(BallEnlargmentTimer::default());
}

fn check_powerups_collision(
    mut commands: Commands,
    query: Query<(Entity, &Transform, &PlayerCollider, &Powerup)>,
    mut query_paddle: Query<(&Transform, &mut Collider, &mut Sprite), With<Paddle>>,
    mut query_ball: Query<(&Velocity, &Transform, &mut Ball, &mut Sprite), Without<Paddle>>,
) {
    let (paddle_transform, mut paddle_collider, mut paddle_sprite) = query_paddle.single_mut();

    for (entity, transform, collider, powerup) in &query {
        let collision = collide(
            transform.translation,
            collider.size,
            paddle_transform.translation,
            paddle_collider.size,
        );

        if collision.is_none() {
            continue;
        }

        match powerup.class {
            PowerupClass::DuplicateBall => duplicate_balls(&mut commands, &query_ball),
            PowerupClass::EnlargeBall => enlarge_balls(&mut commands, &mut query_ball),
            PowerupClass::EnlargePaddle => {
                enlarge_paddle(&mut commands, &mut paddle_sprite, &mut paddle_collider)
            }
        }

        commands.entity(entity).despawn();
    }
}

fn check_game_over(mut state: ResMut<NextState<AppState>>, query: Query<(), With<Ball>>) {
    if !query.is_empty() {
        return;
    }

    state.set(AppState::GameOver);
}

fn check_game_won(mut state: ResMut<NextState<AppState>>, query: Query<(), With<Block>>) {
    if !query.is_empty() {
        return;
    }
    state.set(AppState::GameWon);
}

fn cleanup_component<C>(mut commands: Commands, query: Query<Entity, With<C>>)
where
    C: Component,
{
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

fn check_ball_out_of_bound(
    mut commands: Commands,
    main_box: Res<MainBox>,
    query: Query<(Entity, &Transform, &Ball)>,
) {
    for (entity, Transform { translation, .. }, Ball { size }) in &query {
        if translation.y - 0.5 * size.y < -0.5 * main_box.size.y {
            commands.entity(entity).despawn();
        }
    }
}
