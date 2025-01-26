use super::{Enemy, EnemyMovement, EnemySpawning, ENEMY_SPAWN_PERIOD, HALF_ENEMY_SPRITE_SIZE};
use crate::{enemy::ENEMY_SPAWN_ANIMATION_DURATION, prelude::*};
use rand::random;

pub fn spawn_enemies_over_time(
    mut enemies_spawn: ResMut<EnemiesSpawn>,
    mut spawn_enemy_pub: EventWriter<EnemySpawnEvt>,
    time: Res<Time>,
) {
    enemies_spawn.timer.tick(time.delta());
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
    spawn_enemy_pub.send(EnemySpawnEvt);
}

pub fn spawn_enemy(
    mut spawn_enemy_reader: EventReader<EnemySpawnEvt>,
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    for _ in spawn_enemy_reader.read() {
        let (random_x, random_y) = (
            random::<f32>() * window.width(),
            random::<f32>() * window.height(),
        );

        let (enemy_pos_x, enemy_pos_y) = (
            project_pos_into_screen(random_x, window.width(), HALF_ENEMY_SPRITE_SIZE),
            project_pos_into_screen(random_y, window.height(), HALF_ENEMY_SPRITE_SIZE),
        );

        info!("Spawning enemy on {} {}!", enemy_pos_x, enemy_pos_y);

        let mut transform = Transform::from_xyz(enemy_pos_x, enemy_pos_y, 0.);
        transform.scale = Vec3::ZERO;
        commands.spawn((
            Enemy {},
            Sprite {
                image: asset_server.load("ball_red_small.png"),
                ..default()
            },
            transform,
            EnemySpawning {
                timer: Timer::from_seconds(ENEMY_SPAWN_ANIMATION_DURATION, TimerMode::Once),
            },
            EnemyMovement::random(),
        ));
    }
}

#[derive(Event)]
pub struct EnemySpawnEvt;

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
            enemy_max_count: 15,
        }
    }
}
