pub use crate::render::*;
pub use bevy::prelude::*;
pub use bevy::window::PrimaryWindow;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct GameStartingSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct GameRunningSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct MovementSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct ConfinementSet;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    Starting,
    PauseStartRoutine,
    Running,
    Paused,
}
