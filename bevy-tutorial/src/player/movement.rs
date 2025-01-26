use super::{Player, PLAYER_SPEED, PLAYER_SPRITE_SIZE};
use crate::{prelude::*, render::window_limits};

pub fn move_player(
    mut player_query: Query<&mut Transform, With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyA) {
            direction += LEFT;
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyD) {
            direction += RIGHT;
        }
        if keyboard_input.pressed(KeyCode::ArrowUp) || keyboard_input.pressed(KeyCode::KeyW) {
            direction += UP;
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) || keyboard_input.pressed(KeyCode::KeyS) {
            direction += DOWN;
        }

        if direction == Vec3::ZERO {
            return;
        }

        // NOTE: normalizing diagonal movement
        direction = direction.normalize();
        transform.translation += direction * PLAYER_SPEED * time.delta_secs();
    }
}

pub fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let window = window_query.get_single().unwrap();

        let (x_min, x_max) = window_limits(window.width(), HALF_PLAYER_SPRITE_SIZE);
        let (y_min, y_max) = window_limits(window.height(), HALF_PLAYER_SPRITE_SIZE);

        let mut confined_player_pos = player_transform.translation;

        if confined_player_pos.x < x_min {
            confined_player_pos.x = x_min;
        } else if confined_player_pos.x > x_max {
            confined_player_pos.x = x_max;
        }

        if confined_player_pos.y < y_min {
            confined_player_pos.y = y_min;
        } else if confined_player_pos.y > y_max {
            confined_player_pos.y = y_max;
        }

        if confined_player_pos == player_transform.translation {
            return;
        }

        player_transform.translation = confined_player_pos;
    }
}

const LEFT: Vec3 = Vec3::new(-1.0, 0.0, 0.0);
const UP: Vec3 = Vec3::new(0.0, 1.0, 0.0);
const RIGHT: Vec3 = Vec3::new(1.0, 0.0, 0.0);
const DOWN: Vec3 = Vec3::new(0.0, -1.0, 0.0);

const HALF_PLAYER_SPRITE_SIZE: f32 = PLAYER_SPRITE_SIZE / 2.0;
