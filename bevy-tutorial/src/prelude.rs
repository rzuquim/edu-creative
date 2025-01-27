pub use crate::render::*;
pub use bevy::prelude::*;
pub use bevy::window::PrimaryWindow;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    Starting,
    PauseStartRoutine,
    Running,
    Paused,
    GameOver,
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    MainMenu,
    #[default]
    InGame,
}

pub trait SystemSetsExts {
    fn on_update<S: States, M>(&mut self, s: S, systems: impl IntoSystemConfigs<M>) -> &mut Self;
}

impl SystemSetsExts for App {
    fn on_update<S: States, M>(&mut self, s: S, systems: impl IntoSystemConfigs<M>) -> &mut Self {
        return self.add_systems(Update, systems.run_if(in_state(s)));
    }
}
