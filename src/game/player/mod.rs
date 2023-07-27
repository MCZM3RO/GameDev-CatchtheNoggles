use bevy::prelude::*;

pub mod components;
mod systems;

use systems::*;

use crate::AppState;
use super::SimulationState;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct MovementSystemSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct ConfinementSystem;





pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
        //config system sets
        .configure_set(MovementSystemSet.before(ConfinementSystem))
        //start up systems  .add_startup_system(spawn_player)
        //On EnterState
        .add_system(spawn_player.in_schedule(OnEnter(AppState::Game)))
        //systems
        //.add_system(
            //player_movement
            //.in_set(MovementSystemSet)
            //.run_if(in_state(AppState::Game)) 
            //.run_if(in_state(SimulationState::Running)),         )
            //  .add_system(
            // confine_player_movement
            //.in_set(ConfinementSystem)
            //.run_if(in_state(AppState::Game)) 
            //.run_if(in_state(SimulationState::Running)),         )
        .add_systems(
            (
                player_movement.in_set(MovementSystemSet),
                confine_player_movement.in_set(ConfinementSystem),
            )
                .in_set(OnUpdate(AppState::Game))
                .in_set(OnUpdate(SimulationState::Running)),
        )

        .add_systems(
            (enemy_hit_player, player_hit_lentes)
                 .in_set(OnUpdate(AppState::Game))
                 .in_set(OnUpdate(SimulationState::Running)),
           
        )
       

       //Exit statesystem
    .add_system(despawn_player.in_schedule(OnExit(AppState::Game)));

    }
 }