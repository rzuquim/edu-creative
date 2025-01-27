use super::{
    GameOverEvt, PlayerGotGoodieEvt, MIN_DISTANCE_TO_ENEMY_HIT, MIN_DISTANCE_TO_GOODIE_HIT,
};
use crate::{enemy::Enemy, goodie::Goodie, player::Player, prelude::*};

pub fn check_enemy_hit(
    mut player_hit_pub: EventWriter<GameOverEvt>,
    player_query: Query<&Transform, With<Player>>,
    enemy_query: Query<(&Transform, Entity), With<Enemy>>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for (enemy_transform, enemy_entity) in &enemy_query {
            let distance = player_transform
                .translation
                .distance(enemy_transform.translation);

            if distance > MIN_DISTANCE_TO_ENEMY_HIT {
                continue;
            }

            info!("Enemy hit player!");
            player_hit_pub.send(GameOverEvt { enemy_entity });
        }
    }
}

pub fn check_goodie_hit(
    mut player_hit_pub: EventWriter<PlayerGotGoodieEvt>,
    player_query: Query<&Transform, With<Player>>,
    goodie_query: Query<(&Transform, Entity), With<Goodie>>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for (goodie_transform, goodie_entity) in &goodie_query {
            let distance = player_transform
                .translation
                .distance(goodie_transform.translation);

            if distance > MIN_DISTANCE_TO_GOODIE_HIT {
                continue;
            }

            info!("Player got goodie!");
            player_hit_pub.send(PlayerGotGoodieEvt { goodie_entity });
        }
    }
}

pub fn consume_goodie(
    mut commands: Commands,
    mut consume_goodie_reader: EventReader<PlayerGotGoodieEvt>,
) {
    for evt in consume_goodie_reader.read() {
        commands.entity(evt.goodie_entity).despawn();
    }
}
