mod game_over;
mod lifecycle;
mod movement;

use crate::prelude::*;
use game_over::*;
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
            .on_update(ConfinementSet, confine_player_movement)
            .on_update(
                GameOverSet,
                (
                    start_despawn_player,
                    player_despawn_animation_run,
                    player_despawn,
                ),
            );
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerSpawning {
    pub timer: Timer,
}

#[derive(Component)]
pub struct PlayerDespawning {
    pub timer: Timer,
}

impl Default for PlayerDespawning {
    fn default() -> Self {
        return Self {
            timer: Timer::from_seconds(PLAYER_SPAWN_ANIMATION_DURATION, TimerMode::Once),
        };
    }
}
