mod prelude;

mod enemy;
mod game;
mod goodie;
mod player;
mod render;

use bevy::prelude::{Camera2d, DefaultPlugins};
use prelude::*;

fn main() {
    setup_settings();

    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_camera)
        .add_plugins(player::Plugin)
        .add_plugins(enemy::Plugin)
        .add_plugins(goodie::Plugin)
        .add_plugins(game::Plugin)
        .init_state::<GameState>()
        .configure_sets(
            Update,
            GameRunningSystemSet.run_if(in_state(GameState::Running)),
        )
        .configure_sets(Update, ConfinementSystemSet.in_set(GameRunningSystemSet))
        .configure_sets(
            Update,
            MovementSystemSet
                .before(ConfinementSystemSet)
                .in_set(GameRunningSystemSet),
        )
        .run();
}

fn setup_settings() {
    setup_logging();
}

fn setup_logging() {
    if let Ok(_) = std::env::var("RUST_LOG") {
        return;
    }

    let filter = if cfg!(debug_assertions) {
        "warn,tutorial=debug"
    } else {
        "warn,tutorial=info"
    };

    std::env::set_var("RUST_LOG", filter);
}

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
