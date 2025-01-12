use bevy::{app::App, prelude::*, DefaultPlugins};
use game::GamePlugin;

mod game;
mod input;
mod menu;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GamePlugin)
        // .init_state::<AppState>()
        // .add_systems(OnEnter(AppState::Setup), initial_setup)
        // .add_systems(Update, transition_to_menu.run_if(in_state(AppState::Setup)))
        .run();
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    Setup,
    InMenu,
    InGame,
}

pub fn initial_setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn transition_to_menu(mut app_state: ResMut<NextState<AppState>>) {
    app_state.set(AppState::InMenu);
}
