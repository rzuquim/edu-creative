use crate::{game::GameOverEvt, prelude::*};

use super::{Player, PlayerDespawning, PLAYER_SPAWN_ANIMATION_DURATION};

pub fn start_despawn_player(
    mut commands: Commands,
    mut game_over_evt_reader: EventReader<GameOverEvt>,
    player_query: Query<Entity, With<Player>>,
) {
    for _ in game_over_evt_reader.read() {
        if let Ok(player_entity) = player_query.get_single() {
            info!("Starting to despawn player!");

            commands
                .entity(player_entity)
                .insert(PlayerDespawning::default());
        }
    }
}

pub fn player_despawn_animation_run(
    mut player_query: Query<(&mut Transform, &mut PlayerDespawning), With<Player>>,
    time: Res<Time>,
) {
    if let Ok((mut transform, mut despawning)) = player_query.get_single_mut() {
        despawning.timer.tick(time.delta());
        if despawning.timer.finished() {
            return;
        }

        let size =
            1.0 - (despawning.timer.elapsed().as_secs_f32() / PLAYER_SPAWN_ANIMATION_DURATION);

        transform.scale.x = size;
        transform.scale.y = size;
    }
}

pub fn player_despawn(
    mut commands: Commands,
    player_query: Query<(Entity, &PlayerDespawning), With<Player>>,
) {
    if let Ok((player_entity, despawning)) = player_query.get_single() {
        if !despawning.timer.finished() {
            return;
        }

        info!("Despawning player!");
        commands.entity(player_entity).despawn();
    }
}
