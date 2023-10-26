use bevy::{prelude::*, render::camera::ScalingMode};
use game::GamePlugin;
use game_over::GameOverPlugin;
use game_won::GameWonPlugin;
use menu::*;

mod game;
mod game_over;
mod game_won;
mod menu;

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

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Breakout".to_string(),
                        resizable: false,
                        resolution: (1200.0, 800.0).into(),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_state::<AppState>()
        .add_plugins((GamePlugin, GameOverPlugin, GameWonPlugin, MenuPlugin))
        .insert_resource(ClearColor(Color::AZURE))
        .add_systems(Startup, setup)
        .run();
}
