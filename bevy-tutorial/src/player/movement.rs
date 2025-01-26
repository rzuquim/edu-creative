use super::{Player, HALF_PLAYER_SPRITE_SIZE, PLAYER_SPEED};
use crate::prelude::*;

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

        let mut translation = player_transform.translation;
        let confinement = check_confinement(
            &player_transform.translation,
            window,
            HALF_PLAYER_SPRITE_SIZE,
        );

        if confinement.is_confined() {
            return;
        }

        if confinement.is_outside_right() {
            translation.x = confinement.x_max;
            debug!("Player tring to leave screen on the right!");
        } else if confinement.is_outside_left() {
            translation.x = confinement.x_min;
            debug!("Player tring to leave screen on the left!");
        }

        if confinement.is_outside_top() {
            translation.y = confinement.y_max;
            debug!("Player tring to leave screen on the top!");
        } else if confinement.is_outside_bottom() {
            translation.y = confinement.y_min;
            debug!("Player tring to leave screen on the bottom!");
        }

        player_transform.translation = translation;
    }
}

const LEFT: Vec3 = Vec3::new(-1.0, 0.0, 0.0);
const UP: Vec3 = Vec3::new(0.0, 1.0, 0.0);
const RIGHT: Vec3 = Vec3::new(1.0, 0.0, 0.0);
const DOWN: Vec3 = Vec3::new(0.0, -1.0, 0.0);
