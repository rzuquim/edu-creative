use crate::{game::GameOverEvt, prelude::*};

use super::{Enemy, EnemyActive, EnemyDespawning, ENEMY_SPAWN_ANIMATION_DURATION};

pub fn start_despawn_enemies(
    mut commands: Commands,
    mut game_over_evt_reader: EventReader<GameOverEvt>,
    enemy_query: Query<Entity, With<Enemy>>,
) {
    for evt in game_over_evt_reader.read() {
        info!("Starting to despawn enemy!");
        for enemy_entity in &enemy_query {
            if enemy_entity == evt.enemy_entity {
                continue;
            }

            commands
                .entity(enemy_entity)
                .remove::<EnemyActive>()
                .insert(EnemyDespawning::default());
        }
    }
}

pub fn enemy_despawn_animation_run(
    mut enemy_query: Query<(&mut Transform, &mut EnemyDespawning), With<Enemy>>,
    time: Res<Time>,
) {
    for (mut transform, mut despawning) in enemy_query.iter_mut() {
        despawning.timer.tick(time.delta());
        if despawning.timer.finished() {
            return;
        }

        let size =
            1.0 - (despawning.timer.elapsed().as_secs_f32() / ENEMY_SPAWN_ANIMATION_DURATION);

        transform.scale.x = size;
        transform.scale.y = size;
    }
}

pub fn enemy_despawn(
    mut commands: Commands,
    enemy_query: Query<(Entity, &EnemyDespawning), With<Enemy>>,
) {
    for (entity, spawning) in &enemy_query {
        if !spawning.timer.finished() {
            return;
        }

        info!("Despawning enemy!");
        commands.entity(entity).despawn();
    }
}
