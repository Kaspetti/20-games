use bevy::prelude::*;

use crate::state::{GameState, SceneState};

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(SceneState::InGame), initialize_ui);
        app.add_systems(OnEnter(GameState::Paused), show_pause_text);
        app.add_systems(OnEnter(GameState::Playing), hide_pause_text);
    }
}

#[derive(Component)]
struct PauseText;

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
}

fn show_pause_text(pause_text: Single<&mut Visibility, With<PauseText>>) {
    *pause_text.into_inner() = Visibility::Visible;
}

fn hide_pause_text(pause_text: Single<&mut Visibility, With<PauseText>>) {
    *pause_text.into_inner() = Visibility::Hidden;
}
