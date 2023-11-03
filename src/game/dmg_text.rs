use std::time::Duration;

use bevy::{math::vec2, prelude::*};

use crate::AppState;

use super::Velocity;

pub struct DmgTextPlugin;

impl Plugin for DmgTextPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (fade_text).run_if(in_state(AppState::Game)));
    }
}

#[derive(Component, Clone)]
pub struct DmgText {
    pub timer: Timer,
}

impl DmgText {
    pub const SPEED: Vec2 = vec2(0.0, 40.0);
    pub const FADING_DURATION: Duration = Duration::from_secs(1);
}

pub fn fade_text(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Text, &mut DmgText)>,
) {
    let dt = time.delta();

    for (entity, mut text, mut dmg_text) in &mut query {
        dmg_text.timer.tick(dt);

        // Despawn text if time is elapsed
        if dmg_text.timer.finished() {
            commands.entity(entity).despawn();
            continue;
        }

        let fading = 1.0 - dmg_text.timer.elapsed_secs() / DmgText::FADING_DURATION.as_secs_f32();
        text.sections[0].style.color.set_a(fading);
    }
}

pub fn spawn_dmg_text(commands: &mut Commands, translation: Vec3, dmg: u32) {
    commands.spawn((
        DmgText {
            timer: Timer::new(DmgText::FADING_DURATION, TimerMode::Once),
        },
        Velocity(DmgText::SPEED),
        Text2dBundle {
            text: Text::from_section(
                dmg.to_string(),
                TextStyle {
                    font_size: 30.0,
                    color: Color::RED,
                    ..default()
                },
            ),
            transform: Transform::from_translation(translation),
            ..default()
        },
    ));
}
