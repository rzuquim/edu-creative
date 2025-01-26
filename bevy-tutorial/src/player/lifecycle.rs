use super::{Player, PlayerSpawning, PLAYER_SPAWN_ANIMATION_DURATION};
use crate::{game::PlayerReadyToStart, prelude::*};

pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    info!("Spawning player!");
    commands.spawn((
        Player,
        Sprite {
            image: asset_server.load("ball_blue_small.png"),
            ..default()
        },
        Transform::from_scale(Vec3::ZERO),
        PlayerSpawning {
            timer: Timer::from_seconds(PLAYER_SPAWN_ANIMATION_DURATION, TimerMode::Once),
        },
    ));
}

pub fn player_spawn_animation(
    mut player_query: Query<(&mut Transform, &mut PlayerSpawning), With<Player>>,
    time: Res<Time>,
) {
    if let Ok((mut transform, mut spawning)) = player_query.get_single_mut() {
        spawning.timer.tick(time.delta());
        if spawning.timer.finished() {
            return;
        }

        let size = spawning.timer.elapsed().as_secs_f32() / PLAYER_SPAWN_ANIMATION_DURATION;

        transform.scale.x = size;
        transform.scale.y = size;
    }
}

pub fn activate_player(
    mut commands: Commands,
    player_query: Query<(Entity, &PlayerSpawning), With<Player>>,
    mut player_ready_to_start_pub: EventWriter<PlayerReadyToStart>,
) {
    if let Ok((entity, spawning)) = player_query.get_single() {
        if !spawning.timer.finished() {
            return;
        }
        info!("Player ready to start!");
        commands.entity(entity).remove::<PlayerSpawning>();
        player_ready_to_start_pub.send(PlayerReadyToStart);
    }
}
