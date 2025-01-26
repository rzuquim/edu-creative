mod lifecycle;
mod spawn_animation;

use crate::prelude::*;
use lifecycle::*;
use spawn_animation::*;

pub struct Plugin;

pub const GOODIE_SPRITE_SIZE: f32 = 64.0;
pub const HALF_GOODIE_SPRITE_SIZE: f32 = GOODIE_SPRITE_SIZE / 2.0;
pub const GOODIES_SPAWN_PERIOD: f32 = 6.0;
pub const GOODIE_SPAWN_ANIMATION_DURATION: f32 = 0.6;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GoodiesSpawn>()
            .add_event::<GoodieSpawnEvt>()
            .add_systems(Update, spawn_goodies_over_time)
            .add_systems(Update, spawn_goodie)
            .add_systems(Update, star_spawn_animation_run)
            .add_systems(Update, star_activate);
    }
}

#[derive(Component)]
pub struct Star;

#[derive(Component)]
pub struct GoodieSpawning {
    pub timer: Timer,
}

#[derive(Component)]
pub struct GoodieActive;
