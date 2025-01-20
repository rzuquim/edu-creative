mod lifecycle;

use bevy::{
    app::Startup,
    prelude::{App, Component},
};
use lifecycle::spawn_player;

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
    }
}

#[derive(Component)]
pub struct Player {}
