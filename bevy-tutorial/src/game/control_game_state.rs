use crate::{
    enemy::{Enemy, EnemyActive, STARTUP_ENEMIES_COUNT},
    prelude::*,
};

pub fn toggle_pause_game(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if !keyboard_input.just_pressed(KeyCode::Escape) {
        return;
    }
    match state.get() {
        GameState::Starting => next_state.set(GameState::PauseStartRoutine),
        GameState::PauseStartRoutine => next_state.set(GameState::Starting),
        GameState::Paused => next_state.set(GameState::Running),
        GameState::Running => next_state.set(GameState::Paused),
    }
}

pub fn check_if_game_can_start(
    enemies_query: Query<&EnemyActive, With<Enemy>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if enemies_query.iter().len() == STARTUP_ENEMIES_COUNT {
        next_state.set(GameState::Running);
    }
}
