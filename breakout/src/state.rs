use bevy::prelude::*;

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(SceneState::InGame);
    }
}

#[derive(States, Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub enum SceneState {
    MainMenu,
    InGame,
}
