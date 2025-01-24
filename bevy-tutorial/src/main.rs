mod player;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_camera)
        .add_plugins(player::Plugin)
        .configure_sets(Update, MovementSystemSet.before(ConfinementSystemSet))
        .run();
}

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct MovementSystemSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct ConfinementSystemSet;
