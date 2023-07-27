use bevy::prelude::*;

pub mod components;
mod resources;
mod systems;


use resources::*;
use systems::*;

use crate::AppState;
use super::SimulationState;

pub const NUMBER_OF_LENTES: usize = 10;
pub const LENTES_SIZE: f32 = 30.0; // This is the starn sprite size.

pub struct LentesPlugin;

impl Plugin for LentesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LentesSpawnTimer>()
           //Start up .add_startup_system(spawn_lentes)
           //Systems .add_system(tick_lentes_spawn_timer) .add_system(spawn_lentes_over_time);
           //On enter state
           .add_system(spawn_lentes.in_schedule(OnEnter(AppState::Game)))
           
           //Systems
           .add_systems (
            (
                tick_lentes_spawn_timer,
                spawn_lentes_over_time,
            )
            .in_set(OnUpdate(AppState::Game))
            .in_set(OnUpdate(SimulationState::Running)),
           )
           //Exit state system
           .add_system(despawn_lentes.in_schedule(OnExit(AppState::Game)));
    }
 }