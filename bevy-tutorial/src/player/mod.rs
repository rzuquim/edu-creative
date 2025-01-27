mod lifecycle;
mod movement;

use crate::prelude::*;
use lifecycle::*;
use movement::*;

pub struct Plugin;

pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SPRITE_SIZE: f32 = 64.0;
pub const HALF_PLAYER_SPRITE_SIZE: f32 = PLAYER_SPRITE_SIZE / 2.0;
pub const PLAYER_SPAWN_ANIMATION_DURATION: f32 = 0.6;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.on_startup(GameStartingSet, spawn_player)
            .on_update(GameStartingSet, (player_spawn_animation, activate_player))
            .on_update(MovementSet, move_player)
            .on_update(ConfinementSet, confine_player_movement);
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerSpawning {
    pub timer: Timer,
}
