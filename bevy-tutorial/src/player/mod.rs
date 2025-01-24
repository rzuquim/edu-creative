mod lifecycle;
mod movement;

use bevy::{
    app::{Startup, Update},
    prelude::{App, Component},
};

use lifecycle::*;
use movement::*;

pub struct Plugin;

pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SPRITE_SIZE: f32 = 64.0;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, (move_player, confine_player_movement));
    }
}

#[derive(Component)]
pub struct Player {}
