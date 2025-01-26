mod lifecycle;
mod movement;

use crate::prelude::*;
use lifecycle::*;

#[derive(Component)]
pub struct Enemy {}

pub struct Plugin;

pub const ENEMY_SPAWN_PERIOD: f32 = 5.0;
pub const ENEMY_SPRITE_SIZE: f32 = 64.0;
pub const HALF_ENEMY_SPRITE_SIZE: f32 = ENEMY_SPRITE_SIZE / 2.0;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemiesSpawn>()
            .add_systems(Update, (spawn_enemies_over_time, tick_enemy_spawn_timer));
    }
}
