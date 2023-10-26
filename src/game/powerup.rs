use bevy::{prelude::*, render::view::VisibilitySystems};
use rand::prelude::*;

use super::*;

const POWERUP_RNGS: [(PowerupClass, f32); 3] = [
    (PowerupClass::DuplicateBall, 0.2),
    (PowerupClass::EnlargeBall, 0.2),
    (PowerupClass::EnlargePaddle, 0.2),
];

#[derive(Component, Clone, Copy, Debug)]
pub enum PowerupClass {
    DuplicateBall,
    EnlargeBall,
    EnlargePaddle,
}

#[derive(Component, Clone, Copy, Debug)]
pub struct Powerup {
    pub size: Vec2,
    pub class: PowerupClass,
}

#[derive(Bundle)]
pub struct PowerupBundle {
    pub sprite: SpriteBundle,
    pub powerup: Powerup,
    pub collider: PlayerCollider,
    pub velocity: Velocity,
}

impl Powerup {
    const SIZE: Vec2 = Vec2::new(15.0, 15.0);
    const SPEED: Vec2 = Vec2::new(0.0, -50.0);
    const ROTATION_SPEED: f32 = 180.0;

    fn next_rng() -> Option<Self> {
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..POWERUP_RNGS.len());
        let prob = rng.gen::<f32>();

        if prob < POWERUP_RNGS[index].1 {
            return Some(Self {
                size: Self::SIZE,
                class: POWERUP_RNGS[index].0,
            });
        }
        None
    }

    fn get_powerup_bundle(powerup: Powerup, translation: Vec3) -> PowerupBundle {
        let (color, size) = match powerup.class {
            PowerupClass::DuplicateBall => (Color::RED, Self::SIZE),
            PowerupClass::EnlargeBall => (Color::VIOLET, Self::SIZE),
            PowerupClass::EnlargePaddle => (Color::MIDNIGHT_BLUE, Self::SIZE),
        };

        PowerupBundle {
            sprite: SpriteBundle {
                transform: Transform::from_translation(translation),
                sprite: Sprite {
                    color,
                    custom_size: Some(size),
                    ..default()
                },
                ..default()
            },
            powerup,
            collider: PlayerCollider(Collider { size }),
            velocity: Velocity(Self::SPEED),
        }
    }

    pub fn spawn_powerup(commands: &mut Commands, translation: Vec3) {
        let Some(powerup) = Powerup::next_rng() else { return };

        commands.spawn(Self::get_powerup_bundle(powerup, translation));
    }
}

pub struct PowerupPlugin;

impl Plugin for PowerupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_powerups).run_if(in_state(AppState::Game)))
            .add_systems(
                PostUpdate,
                check_powerups_out_of_bounds
                    .run_if(in_state(AppState::Game))
                    .after(VisibilitySystems::CheckVisibility),
            );
    }
}

fn update_powerups(time: Res<Time>, mut query: Query<&mut Transform, With<Powerup>>) {
    let dt = time.delta_seconds();
    for mut transform in &mut query {
        transform.rotate_y(dt * Powerup::ROTATION_SPEED);
    }
}

fn check_powerups_out_of_bounds(
    mut commands: Commands,
    query: Query<(Entity, &ComputedVisibility), With<Powerup>>,
) {
    for (entity, visibility) in &query {
        if !visibility.is_visible_in_view() {
            commands.entity(entity).despawn();
        }
    }
}
