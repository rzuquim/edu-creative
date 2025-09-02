use super::{Enemy, EnemyActive, EnemySpawning, ENEMY_SPAWN_ANIMATION_DURATION};
use crate::prelude::*;

pub fn enemy_spawn_animation_run(
    mut enemy_query: Query<(&mut Transform, &mut EnemySpawning), With<Enemy>>,
    time: Res<Time>,
) {
    for (mut transform, mut spawning) in enemy_query.iter_mut() {
        spawning.timer.tick(time.delta());
        if spawning.timer.finished() {
            return;
        }

        let size = spawning.timer.fraction();

        transform.scale.x = size;
        transform.scale.y = size;
    }
}

pub fn activate_enemy(
    mut commands: Commands,
    enemy_query: Query<(Entity, &EnemySpawning), With<Enemy>>,
) {
    for (entity, spawning) in &enemy_query {
        if spawning.timer.finished() {
            commands
                .entity(entity)
                .remove::<EnemySpawning>()
                .insert(EnemyActive);
        }
    }
}
