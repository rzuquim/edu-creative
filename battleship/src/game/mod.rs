use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ball);
    }
}

fn spawn_ball(mut commands: Commands, asset_server: Res<AssetServer>) {
    //let window = window_query.get_single().unwrap();

    commands.spawn((Sprite::from_image(asset_server.load("ball.png")), Ball));
}

#[derive(Component)]
pub struct Ball;
