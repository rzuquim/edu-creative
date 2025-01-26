pub use crate::render::*;
pub use bevy::prelude::*;
pub use bevy::window::PrimaryWindow;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct MovementSystemSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct ConfinementSystemSet;
