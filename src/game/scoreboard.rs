use bevy::{prelude::*, sprite::Anchor};

use crate::{camera, AppState};

pub struct ScoreboardPlugin;

impl Plugin for ScoreboardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), setup_scoreboard)
            .add_systems(OnExit(AppState::Game), cleanup_scoreboard)
            .add_systems(Update, update_scoreboard.run_if(in_state(AppState::Game)));
    }
}

#[derive(Resource, Clone, Copy, Deref, DerefMut)]
pub struct Scoreboard(pub u32);

#[derive(Component, Clone)]
pub struct ScoreboardText;

impl Scoreboard {
    pub const FONT_SIZE: f32 = 40.0;
    pub const TEXT_COLOR: Color = Color::rgb(0.5, 0.5, 1.0);
    pub const SCORE_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);
}

pub fn setup_scoreboard(mut commands: Commands) {
    commands.insert_resource(Scoreboard(0));

    commands.spawn((
        ScoreboardText,
        Text2dBundle {
            text: Text::from_sections([
                TextSection::new(
                    "Score: ",
                    TextStyle {
                        font_size: Scoreboard::FONT_SIZE,
                        color: Scoreboard::TEXT_COLOR,
                        ..default()
                    },
                ),
                TextSection::from_style(TextStyle {
                    font_size: Scoreboard::FONT_SIZE,
                    color: Scoreboard::SCORE_COLOR,
                    ..default()
                }),
            ]),
            transform: Transform::from_translation(camera::WINDOW_TOP_LEFT.extend(0.0)),
            text_anchor: Anchor::TopLeft,
            ..default()
        },
    ));
}

fn cleanup_scoreboard(mut commands: Commands, query: Query<Entity, With<ScoreboardText>>) {
    commands.remove_resource::<Scoreboard>();

    for entity in &query {
        commands.entity(entity).despawn();
    }
}

pub fn update_scoreboard(
    score: Res<Scoreboard>,
    mut query: Query<&mut Text, With<ScoreboardText>>,
) {
    let mut text = query.single_mut();
    text.sections[1].value = score.to_string();
}
