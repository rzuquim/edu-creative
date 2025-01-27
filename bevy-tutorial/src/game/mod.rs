mod collisions;
mod control_game_state;

use crate::{
    enemy::HALF_ENEMY_SPRITE_SIZE, goodie::HALF_GOODIE_SPRITE_SIZE,
    player::HALF_PLAYER_SPRITE_SIZE, prelude::*,
};
use collisions::*;
use control_game_state::*;

pub struct Plugin;

pub const MIN_DISTANCE_TO_ENEMY_HIT: f32 =
    (HALF_PLAYER_SPRITE_SIZE + HALF_ENEMY_SPRITE_SIZE) * COLLISION_TOLERANCE;
pub const MIN_DISTANCE_TO_GOODIE_HIT: f32 =
    (HALF_PLAYER_SPRITE_SIZE + HALF_GOODIE_SPRITE_SIZE) * COLLISION_TOLERANCE;
const COLLISION_TOLERANCE: f32 = 0.5;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerHitEvt>()
            .add_event::<PlayerGotGoodieEvt>()
            .add_event::<PlayerReadyToStart>()
            .add_systems(Update, check_if_game_can_start.in_set(GameStartingSet))
            .add_systems(
                Update,
                (
                    toggle_pause_game,
                    check_enemy_hit,
                    check_goodie_hit,
                    consume_goodie,
                )
                    .in_set(GameRunningSet),
            );
    }
}

#[derive(Event)]
pub struct PlayerHitEvt;

#[derive(Event)]
pub struct PlayerGotGoodieEvt {
    pub goodie_entity: Entity,
}

#[derive(Event)]
pub struct PlayerReadyToStart;
