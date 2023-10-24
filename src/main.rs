use std::time::Duration;

use bevy::{math::*, prelude::*, render::camera::ScalingMode, sprite::collide_aabb::*};
use game_over::GameOverPlugin;
use game_won::GameWonPlugin;
use menu::*;
use powerup::*;

mod game;
mod game_over;
mod game_won;
mod menu;
mod powerup;

// Paddle
const PADDLE_START: Vec2 = Vec2::new(0., -250.);
const PADDLE_SIZE: Vec2 = Vec2::new(120., 20.);
const PADDLE_ENLARGED_SIZE: Vec2 = vec2(240.0, 20.);
const PADDLE_COLOR: Color = Color::rgb(0.3, 0.3, 0.7);
const PADDLE_SPEED: f32 = 500.0;
const PADDLE_TIMEOUT: f32 = 10.0;

// Ball
const BALL_COLOR: Color = Color::rgb(0.9, 0.4, 0.2);
const BALL_START: Vec2 = vec2(-70.0, 1.0);
const BALL_SIZE: Vec2 = vec2(30.0, 30.0);
const BALL_ENLARGED_SIZE: Vec2 = vec2(60.0, 60.0);
const BALL_SPEED: f32 = 400.0;
const BALL_DIRECTION: Vec2 = vec2(0.5, -0.5);
const BALL_TIMEOUT: f32 = 10.0;

// Wall
const WALL_WIDTH: f32 = 1200.0;
const WALL_HEIGHT: f32 = 600.0;
const WALL_THICKNESS: f32 = 30.0;

// Blocks
const BLOCK_HEIGHT: i32 = 10;
const BLOCK_WIDTH: i32 = 12;
const BLOCK_SIZE: Vec2 = vec2(
    ((WALL_WIDTH - WALL_THICKNESS - BLOCK_PADDING) / BLOCK_WIDTH as f32) - BLOCK_PADDING,
    25.0,
);
const BLOCK_PADDING: f32 = 5.0;

// Scoreboard
const SCOREBOARD_FONT_SIZE: f32 = 40.0;
const SCOREBOARD_TEXT_PADDING: Val = Val::Px(5.0);
const TEXT_COLOR: Color = Color::rgb(0.5, 0.5, 1.0);
const SCORE_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);

#[derive(Component)]
struct Paddle;

#[derive(Component)]
struct Ball {
    size: Vec2,
}

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct Collider {
    pub size: Vec2,
}

#[derive(Component)]
struct GameEntities;

#[derive(Component, Deref, DerefMut)]
pub struct PlayerCollider(pub Collider);

#[derive(Component)]
struct Block;

#[derive(Component)]
struct Wall;

#[derive(Bundle)]
struct WallBundle {
    sprite: SpriteBundle,
    collider: Collider,
    wall: Wall,
}

#[derive(Resource, Clone, Copy)]
struct Scoreboard {
    score: u32,
}

#[derive(Resource)]
struct BallEnlargmentTimer {
    timer: Timer,
}

#[derive(Resource)]
struct PaddleEnlargedTimer {
    timer: Timer,
}

#[derive(Debug, States, Default, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
    GameWon,
}

fn setup(mut commands: Commands) {
    // Camera
    let mut camera = Camera2dBundle::default();
    camera.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: 1280.0,
        min_height: 720.0,
    };

    commands.spawn(camera);
}

fn setup_game(mut commands: Commands) {
    // Paddle
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: PADDLE_START.extend(0.0),
                ..default()
            },
            sprite: Sprite {
                color: PADDLE_COLOR,
                custom_size: Some(PADDLE_SIZE),
                ..default()
            },
            ..default()
        },
        Paddle,
        Collider { size: PADDLE_SIZE },
    ));

    // Ball
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: BALL_START.extend(0.0),
                ..default()
            },
            sprite: Sprite {
                color: BALL_COLOR,
                custom_size: Some(BALL_SIZE),
                ..default()
            },
            ..default()
        },
        Ball { size: BALL_SIZE },
        Velocity(BALL_SPEED * BALL_DIRECTION),
    ));

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

    spawn_blocks(&mut commands);

    // Scoreboard
    commands.spawn(
        TextBundle::from_sections([
            TextSection::new(
                "Score: ",
                TextStyle {
                    font_size: SCOREBOARD_FONT_SIZE,
                    color: TEXT_COLOR,
                    ..default()
                },
            ),
            TextSection::from_style(TextStyle {
                font_size: SCOREBOARD_FONT_SIZE,
                color: SCORE_COLOR,
                ..default()
            }),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: SCOREBOARD_TEXT_PADDING,
            left: SCOREBOARD_TEXT_PADDING,
            ..default()
        }),
    );
}

fn spawn_blocks(commands: &mut Commands) {
    for w in 0..BLOCK_WIDTH {
        for h in 0..BLOCK_HEIGHT {
            let mut pos = vec3(
                w as f32 * (BLOCK_SIZE.x + BLOCK_PADDING),
                -h as f32 * (BLOCK_SIZE.y + BLOCK_PADDING),
                0.0,
            );

            let mut wall_top_right = vec3(
                (-WALL_WIDTH as f32 + WALL_THICKNESS) * 0.5,
                (WALL_HEIGHT as f32 - WALL_THICKNESS) * 0.5,
                0.0,
            );

            // Add a half of a block size
            wall_top_right += vec3(
                BLOCK_SIZE.x * 0.5 + BLOCK_PADDING,
                -(BLOCK_SIZE.y * 0.5 + BLOCK_PADDING),
                0.0,
            );

            pos += wall_top_right;

            commands.spawn((
                SpriteBundle {
                    transform: Transform {
                        translation: pos,
                        ..default()
                    },
                    sprite: Sprite {
                        color: Color::NAVY,
                        custom_size: Some(BLOCK_SIZE),
                        ..default()
                    },
                    ..default()
                },
                Block,
                Collider { size: BLOCK_SIZE },
            ));
        }
    }
}

fn move_paddle(
    mut query: Query<&mut Transform, With<Paddle>>,
    input: Res<Input<KeyCode>>,
    time_step: Res<FixedTime>,
) {
    let mut paddle_transform = query.single_mut();
    let mut direction = 0.0;

    if input.pressed(KeyCode::Left) {
        direction += -1.0;
    }

    if input.pressed(KeyCode::Right) {
        direction += 1.0;
    }

    direction = direction * PADDLE_SPEED * time_step.period.as_secs_f32();

    paddle_transform.translation.x += direction;

    paddle_transform.translation.x = paddle_transform.translation.x.clamp(
        -(WALL_WIDTH - WALL_THICKNESS - PADDLE_SIZE.x) * 0.5,
        (WALL_WIDTH - WALL_THICKNESS - PADDLE_SIZE.x) * 0.5,
    );
}

fn apply_velocity(mut query: Query<(&Velocity, &mut Transform)>, time_step: Res<FixedTime>) {
    let dt = time_step.period.as_secs_f32();
    for (velocity, mut transform) in &mut query {
        transform.translation.x += velocity.x * dt;
        transform.translation.y += velocity.y * dt;
    }
}

fn check_ball_collision(
    mut balls: Query<(&Transform, &mut Velocity, &Ball)>,
    colliders: Query<(
        Entity,
        &Transform,
        &Collider,
        Option<&Block>,
        Option<&Paddle>,
    )>,
    mut scoreboard: ResMut<Scoreboard>,
    mut commands: Commands,
) {
    for (entity, transform, collider, block, paddle) in &colliders {
        for (ball_t, mut ball_v, ball) in &mut balls {
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
                scoreboard.score += 1;
                commands.entity(entity).despawn();
                Powerup::spawn_powerup(&mut commands, ball_t.translation);
            }

            break;
        }
    }
}

fn check_ball_out_of_bound(mut commands: Commands, query: Query<(Entity, &Transform), With<Ball>>) {
    for (entity, Transform { translation, .. }) in &query {
        if translation.y < PADDLE_START.y {
            commands.entity(entity).despawn();
        }
    }
}

fn update_scoreboard(score: Res<Scoreboard>, mut query: Query<&mut Text>) {
    let mut text = query.single_mut();
    text.sections[1].value = score.score.to_string();
}

fn duplicate_balls(
    commands: &mut Commands,
    query_ball: &Query<(&Velocity, &Transform, &mut Ball, &mut Sprite), Without<Paddle>>,
) {
    let balls = query_ball
        .into_iter()
        .map(|(v, t, _, _)| {
            (
                Velocity(-v.0),
                Ball { size: BALL_SIZE },
                SpriteBundle {
                    transform: t.clone(),
                    sprite: Sprite {
                        color: BALL_COLOR,
                        custom_size: Some(BALL_SIZE),
                        ..default()
                    },
                    ..default()
                },
            )
        })
        .collect::<Vec<_>>();

    commands.spawn_batch(balls);
}

fn enlarge_paddle(commands: &mut Commands, sprite: &mut Sprite, collider: &mut Collider) {
    sprite.custom_size = Some(PADDLE_ENLARGED_SIZE);
    collider.size = PADDLE_ENLARGED_SIZE;

    commands.insert_resource(PaddleEnlargedTimer {
        timer: Timer::new(Duration::from_secs_f32(PADDLE_TIMEOUT), TimerMode::Once),
    });
}

fn enlarge_balls(
    commands: &mut Commands,
    query_ball: &mut Query<(&Velocity, &Transform, &mut Ball, &mut Sprite), Without<Paddle>>,
) {
    for (_, _, mut collider, mut sprite) in query_ball {
        sprite.custom_size = Some(BALL_ENLARGED_SIZE);
        collider.size = BALL_ENLARGED_SIZE;
    }

    commands.insert_resource(BallEnlargmentTimer {
        timer: Timer::new(Duration::from_secs_f32(BALL_TIMEOUT), TimerMode::Once),
    });
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

fn reset_scoreboard(mut score: ResMut<Scoreboard>) {
    score.score = 0;
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
        sprite.custom_size = Some(BALL_SIZE);
        collider.size = BALL_SIZE;
    }

    commands.remove_resource::<BallEnlargmentTimer>();
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
    sprite.custom_size = Some(PADDLE_SIZE);
    collider.size = PADDLE_SIZE;

    commands.remove_resource::<PaddleEnlargedTimer>();
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Breakout".to_string(),
                        resizable: false,
                        resolution: (800.0, 600.0).into(),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_state::<AppState>()
        .add_plugins(GameOverPlugin)
        .add_plugins(GameWonPlugin)
        .add_plugins(PowerupPlugin)
        .insert_resource(ClearColor(Color::AZURE))
        .insert_resource(Scoreboard { score: 0 })
        .add_systems(Startup, setup)
        .add_systems(OnEnter(AppState::MainMenu), setup_menu)
        .add_systems(
            Update,
            (menu_button, menu_key).run_if(in_state(AppState::MainMenu)),
        )
        .add_systems(OnExit(AppState::MainMenu), cleanup_menu)
        .add_systems(
            Update,
            (
                bevy::window::close_on_esc,
                (update_scoreboard, handle_ball_timer, handle_paddle_timer)
                    .run_if(in_state(AppState::Game)),
            ),
        )
        .add_systems(OnEnter(AppState::Game), setup_game)
        .add_systems(
            OnExit(AppState::Game),
            (
                cleanup_component::<Paddle>,
                cleanup_component::<Block>,
                cleanup_component::<Powerup>,
                cleanup_component::<Wall>,
                cleanup_component::<Ball>,
                cleanup_component::<Text>,
                reset_scoreboard,
            ),
        )
        .add_systems(
            FixedUpdate,
            (
                move_paddle,
                apply_velocity,
                check_ball_collision.after(apply_velocity),
                check_ball_out_of_bound.after(apply_velocity),
                check_powerups_collision.after(apply_velocity),
                check_game_over.after(check_ball_out_of_bound),
                check_game_won.after(check_ball_out_of_bound),
            )
                .run_if(in_state(AppState::Game)),
        )
        .run();
}
