use super::{GoodieActive, GoodieSpawning, Star, GOODIE_SPAWN_ANIMATION_DURATION};
use crate::prelude::*;

pub fn star_spawn_animation_run(
    mut enemy_query: Query<(&mut Transform, &mut GoodieSpawning), With<Star>>,
    time: Res<Time>,
) {
    for (mut transform, mut spawning) in enemy_query.iter_mut() {
        spawning.timer.tick(time.delta());
        if spawning.timer.finished() {
            return;
        }

        let size = spawning.timer.elapsed().as_secs_f32() / GOODIE_SPAWN_ANIMATION_DURATION;

        transform.scale.x = size;
        transform.scale.y = size;
    }
}

pub fn star_activate(
    mut commands: Commands,
    enemy_query: Query<(Entity, &GoodieSpawning), With<Star>>,
) {
    for (entity, spawning) in &enemy_query {
        if spawning.timer.finished() {
            commands
                .entity(entity)
                .remove::<GoodieSpawning>()
                .insert(GoodieActive);
        }
    }
}
