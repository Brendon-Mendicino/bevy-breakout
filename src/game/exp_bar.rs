use bevy::{math::vec2, prelude::*, sprite::Anchor};

use crate::{camera, AppState};

use super::{
    level::Level,
    paddle::{level_exp_cap, Paddle},
};

pub struct ExpBarPlugin;

impl Plugin for ExpBarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), spawn_exp_bar)
            .add_systems(Update, update_bar.run_if(in_state(AppState::Game)));
    }
}

#[derive(Component, Clone)]
pub struct ExpBar {
    max_len: f32,
}

fn spawn_exp_bar(mut commands: Commands) {
    let box_size = vec2(camera::WINDOW_SIZE.x, 40.0);
    let padding = vec2(15.0, 15.0);

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::ORANGE,
                anchor: Anchor::BottomLeft,
                custom_size: Some(box_size),
                ..default()
            },
            transform: Transform::from_translation(camera::WINDOW_BOT_LEFT.extend(0.0)),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::GRAY,
                    anchor: Anchor::BottomLeft,
                    custom_size: Some(box_size - padding),
                    ..default()
                },
                transform: Transform::from_translation(0.5 * padding.extend(0.0)),
                ..default()
            });

            parent.spawn((
                ExpBar {
                    max_len: (box_size - padding).x,
                },
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::RED,
                        anchor: Anchor::BottomLeft,
                        custom_size: Some(box_size - padding),
                        ..default()
                    },
                    transform: Transform::from_translation(0.05 * box_size.extend(0.0)),
                    ..default()
                },
            ));
        });
}

fn update_bar(level_q: Query<&Level, With<Paddle>>, mut bar_q: Query<(&mut Sprite, &ExpBar)>) {
    let level = level_q.single();
    let (mut sprite, bar) = bar_q.single_mut();

    let percentage = level.exp as f32 / level_exp_cap(level.level) as f32;

    sprite.custom_size = sprite
        .custom_size
        .map(|size| vec2(percentage * bar.max_len, size.y));
}
