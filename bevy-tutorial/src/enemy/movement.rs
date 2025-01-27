use super::{Enemy, EnemyActive, ENEMY_MAX_SPEED, ENEMY_MIN_SPEED, HALF_ENEMY_SPRITE_SIZE};
use crate::prelude::*;
use rand::random;

pub fn move_enemy(
    mut enemy_query: Query<(&mut Transform, &EnemyMovement, &EnemyActive), With<Enemy>>,
    time: Res<Time>,
) {
    for (mut transform, enemy_movement, _) in enemy_query.iter_mut() {
        transform.translation +=
            enemy_movement.direction * enemy_movement.speed * time.delta_secs();
    }
}

pub fn confine_enemy(
    mut enemy_query: Query<(&Transform, &mut EnemyMovement, &EnemyActive), With<Enemy>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    for (transform, mut enemy_movement, _) in enemy_query.iter_mut() {
        let confinement = check_confinement(&transform.translation, window, HALF_ENEMY_SPRITE_SIZE);

        if confinement.is_confined() {
            continue;
        }

        if confinement.is(OUTSIDE_RIGHT) || confinement.is(OUTSIDE_LEFT) {
            enemy_movement.direction.x *= -1.0;
            debug!("Enemy bounced horizontally!");
        }

        if confinement.is(OUTSIDE_TOP) || confinement.is(OUTSIDE_BOTTOM) {
            enemy_movement.direction.y *= -1.0;
            debug!("Enemy bounced vertically!");
        }
    }
}

#[derive(Component)]
pub struct EnemyMovement {
    pub direction: Vec3,
    pub speed: f32,
}

impl Default for EnemyMovement {
    fn default() -> Self {
        let direction = Vec3::new(random::<f32>() + 0.1, random::<f32>() + 0.1, 0.).normalize();
        let speed = ENEMY_MIN_SPEED + (random::<f32>() * (ENEMY_MAX_SPEED - ENEMY_MIN_SPEED));
        return EnemyMovement { direction, speed };
    }
}
