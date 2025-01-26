mod collisions;

use crate::{
    enemy::HALF_ENEMY_SPRITE_SIZE, goodie::HALF_GOODIE_SPRITE_SIZE,
    player::HALF_PLAYER_SPRITE_SIZE, prelude::*,
};
use collisions::*;

pub struct Plugin;

pub const MIN_DISTANCE_TO_ENEMY_HIT: f32 = HALF_PLAYER_SPRITE_SIZE + HALF_ENEMY_SPRITE_SIZE;
pub const MIN_DISTANCE_TO_GOODIE_HIT: f32 = HALF_PLAYER_SPRITE_SIZE + HALF_GOODIE_SPRITE_SIZE;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerHitEvt>()
            .add_event::<PlayerGotGoodieEvt>()
            .add_systems(Update, check_enemy_hit)
            .add_systems(Update, check_goodie_hit);
    }
}

#[derive(Event)]
pub struct PlayerHitEvt;

#[derive(Event)]
pub struct PlayerGotGoodieEvt;
