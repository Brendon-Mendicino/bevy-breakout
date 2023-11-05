use bevy::{math::vec2, prelude::*, render::camera::ScalingMode};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera);
    }
}

pub const WINDOW_SIZE: Vec2 = vec2(1280.0, 720.0);
pub const WINDOW_TOP_LEFT: Vec2 = vec2(-0.5 * WINDOW_SIZE.x, 0.5 * WINDOW_SIZE.y);

fn setup_camera(mut commands: Commands) {
    // Camera
    let mut camera = Camera2dBundle::default();
    camera.projection.scaling_mode = ScalingMode::Fixed {
        width: WINDOW_SIZE.x,
        height: WINDOW_SIZE.y,
    };

    commands.spawn(camera);
}
