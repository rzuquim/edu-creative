mod game_over;
mod lifecycle;
mod movement;
mod spawn_animation;

use crate::prelude::*;
use game_over::*;
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
            .on_update(
                GameState::Starting,
                (
                    spawn_startup_enemies,
                    enemy_spawn_animation_run,
                    spawn_enemy,
                    activate_enemy,
                ),
            )
            .on_update(
                GameState::Running,
                (
                    spawn_enemies_over_time,
                    enemy_spawn_animation_run,
                    spawn_enemy,
                    activate_enemy,
                    move_enemy,
                    confine_enemy.after(move_enemy),
                ),
            )
            .on_update(
                GameState::GameOver,
                (
                    start_despawn_enemies,
                    enemy_despawn_animation_run,
                    enemy_despawn,
                ),
            );
    }
}

#[derive(Component)]
pub struct EnemySpawning {
    pub timer: Timer,
}

#[derive(Component)]
pub struct EnemyDespawning {
    pub timer: Timer,
}

#[derive(Component)]
pub struct EnemyActive;

impl Default for EnemySpawning {
    fn default() -> Self {
        return Self {
            timer: Timer::from_seconds(ENEMY_SPAWN_ANIMATION_DURATION, TimerMode::Once),
        };
    }
}

impl Default for EnemyDespawning {
    fn default() -> Self {
        return Self {
            timer: Timer::from_seconds(ENEMY_SPAWN_ANIMATION_DURATION, TimerMode::Once),
        };
    }
}
