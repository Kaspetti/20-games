use bevy::prelude::*;

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(SceneState::InGame);
        app.insert_state(GameState::Paused);

        app.add_systems(Update, pause_system.run_if(in_state(SceneState::InGame)));
    }
}

#[derive(States, Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub enum SceneState {
    MainMenu,
    InGame,
}

#[derive(States, Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub enum GameState {
    Paused,
    Playing,
}

fn pause_system(
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::Space) {
        match state.get() {
            GameState::Playing => next_state.set(GameState::Paused),
            GameState::Paused => next_state.set(GameState::Playing),
        }
    }
}
