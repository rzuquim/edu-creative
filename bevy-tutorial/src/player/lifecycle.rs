use crate::prelude::*;
use super::Player;

pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Player {},
        Sprite {
            image: asset_server.load("ball_blue_small.png"),
            ..default()
        },
    ));
}
