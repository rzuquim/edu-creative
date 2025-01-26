pub use crate::render::*;
pub use bevy::prelude::*;
pub use bevy::window::PrimaryWindow;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct GameRunningSystemSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct MovementSystemSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct ConfinementSystemSet;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    Starting,
    PauseStartRoutine,
    #[default]
    Running,
    Paused,
}
