use bevy::prelude::*;

pub const LENTES_SPAWN_TIME: f32 = 1.0;

#[derive(Resource)]
pub struct LentesSpawnTimer {
    pub timer: Timer,
}

impl Default for LentesSpawnTimer {
    fn default() -> LentesSpawnTimer {
        LentesSpawnTimer {
            timer: Timer::from_seconds(LENTES_SPAWN_TIME, TimerMode::Repeating),
        }
    }
}