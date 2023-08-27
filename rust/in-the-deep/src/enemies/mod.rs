use godot::prelude::*;

pub mod owen;

#[derive(Default)]
struct EnemyData {
    health: real,
}

pub trait Enemy {
    fn damage(&mut self, amount: real);
    fn die(&mut self);
}

impl EnemyData {
    pub fn with_health(health: real) -> Self {
        Self {
            health,
            ..Default::default()
        }
    }
}
