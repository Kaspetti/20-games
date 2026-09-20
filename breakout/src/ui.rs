use bevy::prelude::*;

use crate::state::{GameResult, GameState, LastGameResult, SceneState};

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(SceneState::InGame), initialize_ui);
        app.add_systems(OnEnter(GameState::Paused), show_pause_text);
        app.add_systems(OnEnter(GameState::Playing), hide_pause_text);

        app.add_systems(OnEnter(GameState::GameOver), show_game_over_text);
        app.add_systems(OnExit(GameState::GameOver), hide_game_over_text);
    }
}

#[derive(Component)]
struct PauseText;

#[derive(Component)]
struct GameOverText;

#[derive(Component)]
struct ResultText;

fn initialize_ui(mut commands: Commands) {
    commands.spawn((
        PauseText,
        Text::new("Game Paused. Press <Space> to resume..."),
        Node {
            position_type: PositionType::Absolute,
            justify_self: JustifySelf::Center,
            top: percent(55),
            ..default()
        },
        Visibility::Visible,
        DespawnOnExit(SceneState::InGame),
    ));

    commands.spawn((
        PauseText,
        Text::new("Press <Esc> to return to main menu."),
        Node {
            position_type: PositionType::Absolute,
            justify_self: JustifySelf::Center,
            top: percent(60),
            ..default()
        },
        Visibility::Visible,
        DespawnOnExit(SceneState::InGame),
    ));

    commands.spawn((
        ResultText,
        GameOverText,
        Text::new("You won!"),
        Node {
            position_type: PositionType::Absolute,
            justify_self: JustifySelf::Center,
            top: percent(55),
            ..default()
        },
        Visibility::Hidden,
        DespawnOnExit(SceneState::InGame),
    ));
    commands.spawn((
        GameOverText,
        Text::new("Press <Esc> to return to main menu, or <Space> to go again!"),
        Node {
            position_type: PositionType::Absolute,
            justify_self: JustifySelf::Center,
            top: percent(60),
            ..default()
        },
        Visibility::Hidden,
        DespawnOnExit(SceneState::InGame),
    ));
}

fn show_pause_text(mut pause_texts: Query<&mut Visibility, With<PauseText>>) {
    for pause_text in &mut pause_texts {
        *pause_text.into_inner() = Visibility::Visible;
    }
}

fn hide_pause_text(mut pause_texts: Query<&mut Visibility, With<PauseText>>) {
    for pause_text in &mut pause_texts {
        *pause_text.into_inner() = Visibility::Hidden;
    }
}

fn show_game_over_text(
    mut game_over_texts: Query<(&mut Visibility, &mut Text, Has<ResultText>), With<GameOverText>>,
    last_result: Res<LastGameResult>,
) {
    for (game_over_text, mut text, is_result) in &mut game_over_texts {
        *game_over_text.into_inner() = Visibility::Visible;

        if is_result {
            text.0 = match last_result.0 {
                Some(GameResult::Won) => "You won!".to_string(),
                Some(GameResult::Lost) => "You lost!".to_string(),
                None => "You managed to end the game without a result... what?".to_string(),
            }
        }
    }
}

fn hide_game_over_text(mut game_over_texts: Query<&mut Visibility, With<GameOverText>>) {
    for game_over_text in &mut game_over_texts {
        *game_over_text.into_inner() = Visibility::Hidden;
    }
}
