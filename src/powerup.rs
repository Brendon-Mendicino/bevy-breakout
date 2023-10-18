use bevy::prelude::*;
use rand::prelude::*;

const POWERUP_RNGS: [(PowerupClass, f32); 1] = [(PowerupClass::DuplicateBall, 0.2)];

#[derive(Component, Clone, Copy, Debug)]
pub enum PowerupClass {
    DuplicateBall,
}

#[derive(Component, Clone, Copy, Debug)]
pub struct Powerup {
    pub size: Vec2,
    pub class: PowerupClass,
}

impl Powerup {
    pub const SIZE: Vec2 = Vec2::new(15.0, 15.0);
    pub const SPEED: Vec2 = Vec2::new(0.0, -50.0);

    pub fn next_rng() -> Option<Self> {
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
}
