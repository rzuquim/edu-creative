mod lifecycle;
mod movement;
mod spawn_animation;

use crate::prelude::*;
use lifecycle::*;
use movement::*;
use spawn_animation::*;

#[derive(Component)]
pub struct Enemy;

pub struct Plugin;

pub const STARTUP_ENEMIES_COUNT: usize = 5;
pub const ENEMY_SPAWN_PERIOD: f32 = 5.0;
pub const ENEMY_SPAWN_ANIMATION_DURATION: f32 = 0.5;
pub const ENEMY_SPRITE_SIZE: f32 = 64.0;
pub const ENEMY_MIN_SPEED: f32 = 300.0;
pub const ENEMY_MAX_SPEED: f32 = 400.0;
pub const HALF_ENEMY_SPRITE_SIZE: f32 = ENEMY_SPRITE_SIZE / 2.0;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemiesSpawn>()
            .add_event::<EnemySpawnEvt>()
            .add_systems(
                Update,
                (
                    spawn_startup_enemies,
                    enemy_spawn_animation_run,
                    spawn_enemy,
                    activate_enemy,
                )
                    .in_set(GameStartingSet),
            )
            .add_systems(
                Update,
                (
                    spawn_enemies_over_time,
                    enemy_spawn_animation_run,
                    spawn_enemy,
                    activate_enemy,
                )
                    .in_set(GameRunningSet),
            )
            .add_systems(Update, move_enemy.in_set(MovementSet))
            .add_systems(Update, confine_enemy.in_set(ConfinementSet));
    }
}

#[derive(Component)]
pub struct EnemySpawning {
    pub timer: Timer,
}

#[derive(Component)]
pub struct EnemyActive;

// {
//     timer: Timer::from_seconds(ENEMY_SPAWN_ANIMATION_DURATION, TimerMode::Once),
// },
