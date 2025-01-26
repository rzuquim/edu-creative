use super::{PlayerHitEvt, MIN_DISTANCE_TO_ENEMY_HIT};
use crate::{enemy::Enemy, player::Player, prelude::*};

pub fn check_enemy_hit(
    mut player_hit_pub: EventWriter<PlayerHitEvt>,
    player_query: Query<&Transform, With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for enemy_transform in &enemy_query {
            let distance = player_transform
                .translation
                .distance(enemy_transform.translation);

            if distance > MIN_DISTANCE_TO_ENEMY_HIT {
                continue;
            }

            info!("Enemy hit player!");
            player_hit_pub.send(PlayerHitEvt);
        }
    }
}
