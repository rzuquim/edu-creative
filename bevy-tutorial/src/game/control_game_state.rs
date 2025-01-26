use crate::prelude::*;

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
