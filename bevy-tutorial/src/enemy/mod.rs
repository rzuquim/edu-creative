mod lifecycle;
mod movement;

use crate::prelude::*;
use lifecycle::*;
use movement::*;

#[derive(Component)]
pub struct Enemy {}

pub struct Plugin;

pub const ENEMY_SPAWN_PERIOD: f32 = 5.0;
pub const ENEMY_SPRITE_SIZE: f32 = 64.0;
pub const ENEMY_MIN_SPEED: f32 = 300.0;
pub const ENEMY_MAX_SPEED: f32 = 400.0;
pub const HALF_ENEMY_SPRITE_SIZE: f32 = ENEMY_SPRITE_SIZE / 2.0;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemiesSpawn>()
            .add_event::<SpawnEnemyEvt>()
            .add_systems(Update, spawn_enemy)
            .add_systems(Update, spawn_enemies_over_time)
            .add_systems(Update, move_enemy.in_set(MovementSystemSet))
            .add_systems(Update, confine_enemy.in_set(ConfinementSystemSet));
    }
}
