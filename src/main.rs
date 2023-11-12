use bevy::prelude::*;
use camera::*;
use game::GamePlugin;
use game_over::GameOverPlugin;
use game_won::GameWonPlugin;
use menu::*;

mod camera;
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

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Breakout".to_string(),
                        resizable: false,
                        resolution: (950.0, 450.0).into(),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_state::<AppState>()
        .add_plugins((
            GamePlugin,
            GameOverPlugin,
            GameWonPlugin,
            MenuPlugin,
            CameraPlugin,
        ))
        .insert_resource(ClearColor(Color::AZURE))
        .run();
}
