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

pub trait SystemSetsExts {
    fn declare_sets<'a, const N: usize>(
        self: &'a mut Self,
        sets: [impl IntoSystemSetConfigs; N],
    ) -> &'a mut Self;

    fn on_startup<'a, M>(
        self: &'a mut Self,
        set: impl SystemSet,
        systems: impl IntoSystemConfigs<M>,
    ) -> &'a mut Self;

    fn on_update<'a, M>(
        self: &'a mut Self,
        set: impl SystemSet,
        systems: impl IntoSystemConfigs<M>,
    ) -> &'a mut Self;
}

impl SystemSetsExts for App {
    fn declare_sets<'a, const N: usize>(
        self: &'a mut Self,
        sets: [impl IntoSystemSetConfigs; N],
    ) -> &'a mut Self {
        for set in sets {
            self.configure_sets(Update, set);
        }
        return self;
    }

    fn on_startup<'a, M>(
        self: &'a mut Self,
        set: impl SystemSet,
        systems: impl IntoSystemConfigs<M>,
    ) -> &'a mut Self {
        return self.add_systems(Startup, systems.in_set(set));
    }

    fn on_update<'a, M>(
        self: &'a mut Self,
        set: impl SystemSet,
        systems: impl IntoSystemConfigs<M>,
    ) -> &'a mut Self {
        return self.add_systems(Update, systems.in_set(set));
    }
}
