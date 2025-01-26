use super::{ENEMY_MAX_SPEED, ENEMY_MIN_SPEED, HALF_ENEMY_SPRITE_SIZE};
use crate::prelude::*;
use rand::random;

pub fn move_enemy(mut enemy_query: Query<(&mut Transform, &EnemyMovement)>, time: Res<Time>) {
    for (mut transform, enemy_movement) in enemy_query.iter_mut() {
        transform.translation +=
            enemy_movement.direction * enemy_movement.speed * time.delta_secs();
    }
}

pub fn confine_enemy(
    mut enemy_query: Query<(&Transform, &mut EnemyMovement)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    for (transform, mut enemy_movement) in enemy_query.iter_mut() {
        let confinement = check_confinement(&transform.translation, window, HALF_ENEMY_SPRITE_SIZE);

        if confinement.is_confined() {
            continue;
        }

        if confinement.is_outside_right() || confinement.is_outside_left() {
            enemy_movement.direction.x *= -1.0;
            debug!("Enemy bounced horizontally!");
        }

        if confinement.is_outside_top() || confinement.is_outside_bottom() {
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

impl EnemyMovement {
    pub fn random() -> Self {
        let direction = Vec3::new(random::<f32>(), random::<f32>(), 0.).normalize();
        let speed = ENEMY_MIN_SPEED + (random::<f32>() * (ENEMY_MAX_SPEED - ENEMY_MIN_SPEED));
        return EnemyMovement { direction, speed };
    }
}
