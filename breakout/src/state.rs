use bevy::prelude::*;

use crate::{
    arena::ARENA_HEIGHT,
    ball::Ball,
    brick::{BRICK_COLUMNS, BRICK_ROWS, BrickBroken, BrokenBricks},
};

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(SceneState::MainMenu);
        app.insert_state(GameState::Playing);

        app.add_systems(Update, pause_handler.run_if(in_state(SceneState::InGame)));
        app.add_systems(
            Update,
            exit_handler
                .run_if(in_state(GameState::Paused))
                .run_if(in_state(SceneState::InGame)),
        );

        app.add_systems(
            Update,
            check_loss
                .run_if(in_state(SceneState::InGame))
                .run_if(in_state(GameState::Playing)),
        );

        app.add_systems(
            Update,
            game_over_handler
                .run_if(in_state(SceneState::InGame))
                .run_if(in_state(GameState::GameOver)),
        );

        app.add_observer(check_win);

        app.insert_resource(LastGameResult(None));
    }
}

const TOTAL_BRICKS: f32 = (BRICK_COLUMNS * BRICK_ROWS) as f32;

#[derive(States, Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub enum SceneState {
    MainMenu,
    InGame,
}

#[derive(States, Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub enum GameState {
    Paused,
    Playing,
    GameOver,
}

#[derive(Resource)]
pub struct LastGameResult(pub Option<GameResult>);

pub enum GameResult {
    Won,
    Lost,
}

fn pause_handler(
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::Space) {
        match state.get() {
            GameState::Playing => next_state.set(GameState::Paused),
            GameState::Paused => next_state.set(GameState::Playing),
            GameState::GameOver => {}
        }
    }
}

fn game_over_handler(
    mut next_state: ResMut<NextState<GameState>>,
    mut next_scene: ResMut<NextState<SceneState>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::Escape) {
        next_scene.set(SceneState::MainMenu);
    }

    if input.just_pressed(KeyCode::Space) {
        next_scene.set(SceneState::InGame);
        next_state.set(GameState::Paused);
    }
}

fn exit_handler(mut next_scene: ResMut<NextState<SceneState>>, input: Res<ButtonInput<KeyCode>>) {
    if input.just_pressed(KeyCode::Escape) {
        next_scene.set(SceneState::MainMenu);
    }
}

fn check_win(
    _: On<BrickBroken>,
    broken_bricks: Res<BrokenBricks>,
    mut next_state: ResMut<NextState<GameState>>,
    mut last_game_result: ResMut<LastGameResult>,
) {
    if broken_bricks.0 == TOTAL_BRICKS {
        next_state.set(GameState::GameOver);
        last_game_result.0 = Some(GameResult::Won);
    }
}

fn check_loss(
    ball: Single<&Transform, With<Ball>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut last_game_result: ResMut<LastGameResult>,
) {
    let transform = ball.into_inner();

    if transform.translation.y <= -(ARENA_HEIGHT / 2.0) {
        next_state.set(GameState::GameOver);
        last_game_result.0 = Some(GameResult::Lost);
    }
}
