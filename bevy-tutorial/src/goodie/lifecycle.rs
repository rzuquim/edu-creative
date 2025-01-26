use rand::random;

use super::GOODIES_SPAWN_PERIOD;
use crate::{
    goodie::{GoodieSpawning, Star, GOODIE_SPAWN_ANIMATION_DURATION, HALF_GOODIE_SPRITE_SIZE},
    prelude::*,
};

pub fn spawn_goodies_over_time(
    mut spawn_goodies_pub: EventWriter<GoodieSpawnEvt>,
    mut goodies_spawn: ResMut<GoodiesSpawn>,
    time: Res<Time>,
) {
    goodies_spawn.timer.tick(time.delta());
    if !goodies_spawn.timer.finished() {
        return;
    }

    if goodies_spawn.goodies_curr_count >= goodies_spawn.goodies_max_count {
        return;
    }

    debug!("Spawning goodie!",);
    goodies_spawn.goodies_curr_count += 1;
    spawn_goodies_pub.send(GoodieSpawnEvt);
}

pub fn spawn_goodie(
    mut spawn_goodie_reader: EventReader<GoodieSpawnEvt>,
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    for _ in spawn_goodie_reader.read() {
        let (random_x, random_y) = (
            random::<f32>() * window.width(),
            random::<f32>() * window.height(),
        );

        let (goodie_pos_x, goodie_pos_y) = (
            project_pos_into_screen(random_x, window.width(), HALF_GOODIE_SPRITE_SIZE),
            project_pos_into_screen(random_y, window.height(), HALF_GOODIE_SPRITE_SIZE),
        );

        info!("Spawning goodie on {} {}!", goodie_pos_x, goodie_pos_y);

        let mut transform = Transform::from_xyz(goodie_pos_x, goodie_pos_y, 0.);
        // transform.scale = Vec3::ZERO;
        commands.spawn((
            Star {},
            Sprite {
                image: asset_server.load("star.png"),
                ..default()
            },
            transform,
            GoodieSpawning {
                timer: Timer::from_seconds(GOODIE_SPAWN_ANIMATION_DURATION, TimerMode::Once),
            },
        ));
    }
}

#[derive(Event)]
pub struct GoodieSpawnEvt;

#[derive(Resource)]
pub struct GoodiesSpawn {
    timer: Timer,
    goodies_curr_count: usize,
    goodies_max_count: usize,
}

impl Default for GoodiesSpawn {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(GOODIES_SPAWN_PERIOD, TimerMode::Repeating),
            goodies_curr_count: 0,
            goodies_max_count: 5,
        }
    }
}
