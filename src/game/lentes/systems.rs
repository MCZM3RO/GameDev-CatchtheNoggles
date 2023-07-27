use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;

use super::NUMBER_OF_LENTES;
use super::components::*;
use super::resources::*;


pub fn spawn_lentes(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    for _ in 0..NUMBER_OF_LENTES {
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("sprites/lentesazul.png"),
                ..default()
            },
            Lentes {},
        ));
    }
}

pub fn despawn_lentes (mut commands: Commands, lentes_query: Query<Entity, With<Lentes>>,) {
    for lentes_entity in lentes_query.iter() {
        commands.entity(lentes_entity).despawn();
    }
}

pub fn tick_lentes_spawn_timer(mut lentes_spawn_timer: ResMut<LentesSpawnTimer>, time: Res<Time>) {
    lentes_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_lentes_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    lentes_spawn_timer: Res<LentesSpawnTimer>,
) {
    if lentes_spawn_timer.timer.finished() {
        let window = window_query.get_single().unwrap();
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("sprites/lentesazul.png"),
                ..default()
            },
            Lentes {},
        ));
    }
}