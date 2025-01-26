use super::{Enemy, EnemyMovement, ENEMY_SPAWN_PERIOD, HALF_ENEMY_SPRITE_SIZE};
use crate::prelude::*;
use rand::random;

pub fn spawn_enemies_over_time(
    mut commands: Commands,
    mut enemies_spawn: ResMut<EnemiesSpawn>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    if !enemies_spawn.timer.finished() {
        return;
    }

    debug!(
        "Spawning enemy, if not reached limit, {}/{}",
        enemies_spawn.enemy_curr_count, enemies_spawn.enemy_max_count
    );

    if enemies_spawn.enemy_curr_count >= enemies_spawn.enemy_max_count {
        return;
    }

    // TODO: trigger event to spawn
    enemies_spawn.enemy_curr_count += 1;

    let window = window_query.get_single().unwrap();
    let (random_x, random_y) = (
        random::<f32>() * window.width(),
        random::<f32>() * window.height(),
    );

    let (enemy_pos_x, enemy_pos_y) = (
        project_pos_into_screen(random_x, window.width(), HALF_ENEMY_SPRITE_SIZE),
        project_pos_into_screen(random_y, window.height(), HALF_ENEMY_SPRITE_SIZE),
    );

    info!("Spawning enemy on {} {}!", enemy_pos_x, enemy_pos_y);

    commands.spawn((
        Enemy {},
        Sprite {
            image: asset_server.load("ball_red_small.png"),
            ..default()
        },
        Transform::from_xyz(enemy_pos_x, enemy_pos_y, 0.),
        EnemyStatus::Spawning,
        EnemyMovement::random(),
    ));
}

pub fn tick_enemy_spawn_timer(mut enemy_spawn_timer: ResMut<EnemiesSpawn>, time: Res<Time>) {
    enemy_spawn_timer.timer.tick(time.delta());
}

#[derive(Component)]
pub enum EnemyStatus {
    Spawning,
    // CanHurtPlayer,
}

#[derive(Resource)]
pub struct EnemiesSpawn {
    timer: Timer,
    enemy_curr_count: usize,
    enemy_max_count: usize,
}

impl Default for EnemiesSpawn {
    fn default() -> EnemiesSpawn {
        EnemiesSpawn {
            timer: Timer::from_seconds(ENEMY_SPAWN_PERIOD, TimerMode::Repeating),
            enemy_curr_count: 0,
            enemy_max_count: 5,
        }
    }
}
