mod lifecycle;
mod movement;

use crate::{prelude::*, ConfinementSystemSet, MovementSystemSet};
use lifecycle::*;
use movement::*;

pub struct Plugin;

pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SPRITE_SIZE: f32 = 64.0;
pub const HALF_PLAYER_SPRITE_SIZE: f32 = PLAYER_SPRITE_SIZE / 2.0;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, move_player.in_set(MovementSystemSet))
            .add_systems(Update, confine_player_movement.in_set(ConfinementSystemSet));
    }
}

#[derive(Component)]
pub struct Player {}
