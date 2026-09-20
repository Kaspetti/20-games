use bevy::{
    input_focus::{FocusCause, InputFocus},
    prelude::*,
};

use crate::state::{GameState, SceneState};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, initialize);
        app.add_systems(OnEnter(SceneState::MainMenu), initialize);
        app.add_systems(Update, button_system.run_if(in_state(SceneState::MainMenu)));
    }
}

const BUTTON_BG_NORMAL_COLOR: Color = Color::srgba(0.0, 0.0, 0.0, 0.5);
const BUTTON_BG_HOVER_COLOR: Color = Color::srgba(0.0, 0.0, 0.0, 0.75);
const BUTTON_BG_PRESSED_COLOR: Color = Color::srgba(0.0, 0.0, 0.0, 0.25);

#[derive(Component)]
enum ButtonType {
    Start,
    Exit,
}

fn button_system(
    mut input_focus: ResMut<InputFocus>,
    mut interaction_query: Query<
        (
            Entity,
            &Interaction,
            &mut BackgroundColor,
            &mut Button,
            &ButtonType,
        ),
        Changed<Interaction>,
    >,
    mut commands: Commands,
    mut next_scene: ResMut<NextState<SceneState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (entity, interaction, mut color, mut button, button_type) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                input_focus.set(entity, FocusCause::Pressed);
                *color = BUTTON_BG_PRESSED_COLOR.into();

                match button_type {
                    ButtonType::Start => {
                        next_scene.set(SceneState::InGame);
                        next_state.set(GameState::Paused);
                    }

                    ButtonType::Exit => {
                        commands.write_message(AppExit::Success);
                    }
                }

                button.set_changed();
            }
            Interaction::Hovered => {
                input_focus.set(entity, FocusCause::Pressed);
                *color = BUTTON_BG_HOVER_COLOR.into();
                button.set_changed();
            }
            Interaction::None => {
                input_focus.clear();
                *color = BUTTON_BG_NORMAL_COLOR.into();
            }
        }
    }
}

fn initialize(mut commands: Commands) {
    commands.spawn((
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            row_gap: px(5),
            ..default()
        },
        DespawnOnExit(SceneState::MainMenu),
        children![
            button("Start Game", ButtonType::Start),
            button("Exit Game", ButtonType::Exit),
        ],
    ));
}

fn button(button_text: &str, button_type: ButtonType) -> impl Bundle {
    (
        Button,
        button_type,
        Node {
            width: px(150),
            height: px(65),
            border: UiRect::all(px(5)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            border_radius: BorderRadius::MAX,
            ..default()
        },
        BorderColor::all(Color::WHITE),
        BackgroundColor(BUTTON_BG_NORMAL_COLOR),
        children![(
            Text::new(button_text),
            TextColor(Color::srgb(0.9, 0.9, 0.9)),
        )],
    )
}
