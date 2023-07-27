pub mod events;
mod game;
mod main_menu;
mod systems;


use game::GamePlugin;
use main_menu::MainMenuPlugin;

use systems::*;

use bevy::prelude::*;
use bevy_wasm_window_resize::WindowResizePlugin;


fn main() {
    App::new()
        //Bevy Plugins
        .insert_resource(ClearColor(Color::rgb(0.5, 0.5, 0.9)))
        .add_plugin(WindowResizePlugin)
        .add_plugins(DefaultPlugins)
        .add_state::<AppState>()
        //My Plugins
        .add_plugin(MainMenuPlugin)
        .add_plugin(GamePlugin)
        //Startup Systems
        .add_startup_system(spawn_camera)
       
        //Systems
        .add_system(transition_to_game_state)
        .add_system(transition_to_main_menu_state)
        .add_system(exit_game)
        .add_system(handle_game_over)
        .run();
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,

}



