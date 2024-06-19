use bevy::prelude::*;

use crate::AppState;

#[derive(Resource, Clone, Copy)]
struct GameWonScene {
    text: Entity,
}

pub struct GameWonPlugin;

impl Plugin for GameWonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::GameWon), setup_game_over)
            .add_systems(OnExit(AppState::GameWon), cleanup_game_over)
            .add_systems(Update, reload_game.run_if(in_state(AppState::GameWon)));
    }
}

fn reload_game(input: Res<ButtonInput<KeyCode>>, mut state: ResMut<NextState<AppState>>) {
    if input.pressed(KeyCode::Enter) {
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
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section("You Won Nothing!", style.clone()));

            parent.spawn(TextBundle::from_section(
                "Press \"Enter\" to replay!",
                style,
            ));
        })
        .id();

    commands.insert_resource(GameWonScene { text });
}

fn cleanup_game_over(mut commands: Commands, scene: Res<GameWonScene>) {
    commands.entity(scene.text).despawn_recursive();
    commands.remove_resource::<GameWonScene>();
}
