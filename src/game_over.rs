use bevy::prelude::*;

use crate::AppState;

#[derive(Resource, Clone, Copy)]
struct GameOverScene {
    text: Entity,
}

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::GameOver), setup_game_over)
            .add_systems(OnExit(AppState::GameOver), cleanup_game_over)
            .add_systems(Update, reload_game.run_if(in_state(AppState::GameOver)));
    }
}

fn reload_game(input: Res<Input<KeyCode>>, mut state: ResMut<NextState<AppState>>) {
    if input.pressed(KeyCode::Return) {
        state.set(AppState::Game);
    }
}

fn setup_game_over(mut commands: Commands) {
    let style = TextStyle {
        font_size: 60.0,
        color: Color::rgb(0.8, 0.8, 0.8),
        ..default()
    };

    let text = commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section("You Lost!", style.clone()));

            parent.spawn(TextBundle::from_section(
                "Press \"Enter\" to replay!",
                style,
            ));
        })
        .id();

    commands.insert_resource(GameOverScene { text });
}

fn cleanup_game_over(mut commands: Commands, scene: Res<GameOverScene>) {
    commands.entity(scene.text).despawn_recursive();
    commands.remove_resource::<GameOverScene>();
}
