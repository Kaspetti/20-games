use std::f32::consts::PI;

use bevy::camera::ScalingMode;
use bevy::input_focus::{FocusCause, InputFocus};
use bevy::prelude::*;
use rand::RngExt;

// Settings
const BAR_SPEED: f32 = 1000.0;
const BAR_WIDTH: f32 = 5.0;
const BAR_HEIGHT: f32 = 150.0;

const BALL_SPEED: f32 = 1500.0;
const BALL_RADIUS: f32 = 5.0;
const BALL_MAX_ANGLE: f32 = 45.0;

const WINDOW_WIDTH: u32 = 1280;
const WINDOW_HEIGHT: u32 = 720;

const BUTTON_BG_NORMAL: Color = Color::BLACK;
const BUTTON_BG_HOVER: Color = Color::srgb(0.25, 0.25, 0.25);
const BUTTON_BG_PRESS: Color = Color::WHITE;

const BUTTON_BORDER_NORMAL: Color = Color::WHITE;
const BUTTON_BORDER_HOVER: Color = Color::srgb(0.75, 0.75, 0.75);
const BUTTON_BORDER_PRESS: Color = Color::srgb(0.75, 0.75, 0.75);

const BUTTON_TEXT_NORMAL: Color = Color::WHITE;
const BUTTON_TEXT_HOVER: Color = Color::WHITE;
const BUTTON_TEXT_PRESS: Color = Color::BLACK;

// TODO: Main Menu
// TODO: AI

// State
#[derive(States, Debug, Hash, Eq, PartialEq, Clone, Copy)]
enum GameState {
    Paused,
    InGame,
    Ended,
    MainMenu,
}

// Components
#[derive(Component)]
struct Moving {
    speed: f32,
    direction: Vec3,
}

#[derive(Component)]
struct Bar(u8);

// Contains information on whether the ball
// has hit a racket yet
#[derive(Component)]
struct Ball(bool);

#[derive(Component)]
struct ScoreText;

#[derive(Component)]
struct PauseText;

#[derive(Component)]
struct MainMenu;

#[derive(Component)]
struct StartButton;

#[derive(Component)]
struct QuitButton;

// Resources
#[derive(Resource, Deref)]
struct SoundEffect {
    handle: Handle<AudioSource>,
}

#[derive(Resource)]
struct Score {
    p1: u32,
    p2: u32,
}

pub struct PongPlugin;

impl Plugin for PongPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Score { p1: 0, p2: 0 });
        app.init_resource::<SoundEffect>();
        app.init_resource::<InputFocus>();
        app.insert_state(GameState::MainMenu);
        app.add_systems(Startup, setup);
        app.add_systems(
            Update,
            (
                (bar_input, movement, clamp_bars, collision, ball_bounds)
                    .chain()
                    .run_if(in_state(GameState::InGame)),
                toggle_pause,
                update_score_text,
                button_system.run_if(in_state(GameState::MainMenu)),
            ),
        );

        app.add_systems(OnEnter(GameState::Paused), show_pause_text);
        app.add_systems(OnExit(GameState::Paused), hide_pause_text);

        app.add_systems(OnEnter(GameState::Ended), reset_game);

        app.add_systems(OnEnter(GameState::MainMenu), show_main_menu);
        app.add_systems(OnExit(GameState::MainMenu), hide_main_menu);
    }
}

impl FromWorld for SoundEffect {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        SoundEffect {
            handle: asset_server.load("sounds/pong.ogg"),
        }
    }
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(PongPlugin)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut rng = rand::rng();

    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::Fixed {
                width: WINDOW_WIDTH as f32,
                height: WINDOW_HEIGHT as f32,
            },
            ..OrthographicProjection::default_2d()
        }),
    ));

    main_menu_setup(&mut commands);

    let color = Color::WHITE;
    let material = materials.add(color);

    spawn_bar(0, &material, &mut commands, &mut meshes);
    spawn_bar(1, &material, &mut commands, &mut meshes);

    let ball_mesh = meshes.add(Circle::new(BALL_RADIUS));
    commands.spawn((
        Ball(false),
        Moving {
            speed: BALL_SPEED / 2.0,
            direction: Vec3::new(
                (rng.random_range(-1.0..1.0) as f32).signum(),
                rng.random_range(-0.25..0.25),
                0.0,
            ),
        },
        Mesh2d(ball_mesh),
        MeshMaterial2d(material),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    commands.spawn((
        ScoreText,
        Text::new("00 | 00"),
        Node {
            position_type: PositionType::Absolute,
            justify_self: JustifySelf::Center,
            top: px(25),
            ..default()
        },
    ));

    commands.spawn((
        PauseText,
        Text::new("Press <Space> to start..."),
        Node {
            position_type: PositionType::Absolute,
            justify_self: JustifySelf::Center,
            top: percent(55),
            ..default()
        },
        Visibility::Hidden,
    ));
}

fn spawn_bar(
    player: u8,
    material: &Handle<ColorMaterial>,
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
) {
    let bar_mesh = meshes.add(Rectangle::new(BAR_WIDTH, BAR_HEIGHT));

    commands.spawn((
        Bar(player),
        Moving {
            speed: BAR_SPEED,
            direction: Vec3::ZERO,
        },
        Mesh2d(bar_mesh),
        MeshMaterial2d(material.clone()),
        Transform::from_xyz(
            if player == 0 {
                -(WINDOW_WIDTH as f32) / 2.0 + 5.0
            } else {
                WINDOW_WIDTH as f32 / 2.0 - 5.0
            },
            0.0,
            0.0,
        ),
    ));
}

fn movement(mut entities: Query<(&Moving, &mut Transform)>, timer: Res<Time>) {
    for (moving, mut transform) in &mut entities {
        transform.translation += moving.direction * moving.speed * timer.delta_secs();
    }
}

fn bar_input(mut bars: Query<(&Bar, &mut Moving)>, keys: Res<ButtonInput<KeyCode>>) {
    for (bar, mut moving) in &mut bars {
        let mut direction = Vec3::ZERO;

        if bar.0 == 0 {
            if keys.pressed(KeyCode::KeyW) {
                direction.y += 1.0;
            }
            if keys.pressed(KeyCode::KeyS) {
                direction.y -= 1.0;
            }
        } else {
            if keys.pressed(KeyCode::ArrowUp) {
                direction.y += 1.0;
            }
            if keys.pressed(KeyCode::ArrowDown) {
                direction.y -= 1.0;
            }
        }

        moving.direction = direction;
    }
}

fn clamp_bars(mut bars: Query<&mut Transform, With<Bar>>) {
    for mut transform in &mut bars {
        transform.translation.y = transform.translation.y.clamp(
            (-(WINDOW_HEIGHT as f32) + BAR_HEIGHT) / 2.0,
            (WINDOW_HEIGHT as f32 - BAR_HEIGHT) / 2.0,
        );
    }
}

fn ball_bounds(
    ball: Single<(&mut Moving, &mut Transform), With<Ball>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut score: ResMut<Score>,
) {
    let (mut moving, mut transform) = ball.into_inner();

    if transform.translation.y - BALL_RADIUS <= -(WINDOW_HEIGHT as f32) / 2.0 {
        transform.translation.y = -(WINDOW_HEIGHT as f32) / 2.0 + BALL_RADIUS + 0.1;
        moving.direction.y *= -1.0;
    }
    if transform.translation.y + BALL_RADIUS >= WINDOW_HEIGHT as f32 / 2.0 {
        transform.translation.y = WINDOW_HEIGHT as f32 / 2.0 - BALL_RADIUS + 0.1;
        moving.direction.y *= -1.0;
    }

    if transform.translation.x >= WINDOW_WIDTH as f32 / 2.0
        || transform.translation.x <= -(WINDOW_WIDTH as f32) / 2.0
    {
        next_state.set(GameState::Ended);
        if transform.translation.x > 0.0 {
            score.p1 += 1;
        } else {
            score.p2 += 1;
        }
    }
}

#[allow(clippy::type_complexity)]
fn collision(
    bars: Query<&Transform, (With<Bar>, Without<Ball>)>,
    ball: Single<(&mut Transform, &mut Moving, &mut Ball), Without<Bar>>,
    sound_effect: Res<SoundEffect>,
    mut commands: Commands,
) {
    let (mut ball_transform, mut ball_moving, mut ball) = ball.into_inner();

    for bar_transform in &bars {
        let aabb = Vec3::new(BAR_WIDTH / 2.0, BAR_HEIGHT / 2.0, 0.0);

        let distance = ball_transform.translation - bar_transform.translation;
        let clamped_distance = distance.clamp(-aabb, aabb);
        let closest_point = bar_transform.translation + clamped_distance;

        let collision = (ball_transform.translation - closest_point).length() <= BALL_RADIUS;

        if collision {
            if !ball.0 {
                ball.0 = true;
                ball_moving.speed = BALL_SPEED;
            }

            commands.spawn((
                AudioPlayer::new(sound_effect.clone()),
                PlaybackSettings::DESPAWN,
            ));

            let offset = ball_transform.translation.y - bar_transform.translation.y;
            let normalized_offset = offset / (BAR_HEIGHT / 2.0);
            let bounce_angle = (BALL_MAX_ANGLE * normalized_offset) * (PI / 180.0);

            let sign = ball_transform.translation.x.signum();

            // Slide ball out of the bar
            ball_transform.translation.x =
                bar_transform.translation.x - ((BAR_WIDTH / 2.0) + BALL_RADIUS + 0.1) * sign;

            ball_moving.direction = Vec3::new(
                -sign * ops::cos(-sign * bounce_angle),
                -sign * ops::sin(-sign * bounce_angle),
                0.0,
            );
        }
    }
}

fn toggle_pause(
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        next_state.set(match state.get() {
            GameState::Paused => GameState::InGame,
            GameState::InGame => GameState::Paused,
            _ => *state.get(),
        });
    }
}

fn update_score_text(score_text: Single<&mut Text, With<ScoreText>>, score: Res<Score>) {
    if score.is_changed() {
        score_text.into_inner().0 = format!("{:02} | {:02}", score.p1, score.p2);
    }
}

fn show_pause_text(pause_text: Single<&mut Visibility, With<PauseText>>) {
    *pause_text.into_inner() = Visibility::Visible;
}

fn hide_pause_text(pause_text: Single<&mut Visibility, With<PauseText>>) {
    *pause_text.into_inner() = Visibility::Hidden;
}

fn reset_game(
    ball_entity: Single<(&mut Transform, &mut Moving, &mut Ball), Without<Bar>>,
    bars: Query<&mut Transform, (With<Bar>, Without<Ball>)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let (mut ball_transform, mut ball_moving, mut ball) = ball_entity.into_inner();
    reset_ball(&mut ball_transform, &mut ball_moving, &mut ball);

    reset_bars(bars);

    next_state.set(GameState::Paused);
}

fn reset_ball(ball_transform: &mut Transform, ball_moving: &mut Moving, ball: &mut Ball) {
    ball_transform.translation = Vec3::ZERO;

    ball_moving.speed = BALL_SPEED / 2.0;
    ball_moving.direction = Vec3::new(
        (rand::rng().random_range(-1.0..1.0) as f32).signum(),
        rand::rng().random_range(-0.25..0.25),
        0.0,
    );

    ball.0 = false;
}

fn reset_bars(mut bars: Query<&mut Transform, (With<Bar>, Without<Ball>)>) {
    for mut transform in &mut bars {
        transform.translation.y = 0.0;
    }
}

fn main_menu_setup(commands: &mut Commands) {
    commands.spawn((
        MainMenu,
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            row_gap: px(10),
            ..default()
        },
        Visibility::Visible,
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.9)),
        ZIndex(100),
        children![
            (
                Button,
                StartButton,
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
                BackgroundColor(Color::BLACK),
                children![(Text::new("Start Game"))]
            ),
            (
                Button,
                QuitButton,
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
                BackgroundColor(BUTTON_BG_NORMAL),
                children![(
                    Text::new("Exit Game"),
                    children![(TextColor(BUTTON_TEXT_NORMAL))]
                )]
            )
        ],
    ));
}

#[allow(clippy::type_complexity)]
fn button_system(
    mut input_focus: ResMut<InputFocus>,
    start_button: Single<
        (
            Entity,
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &mut Button,
            &Children,
        ),
        With<StartButton>,
    >,
    mut text_color_query: Query<&mut TextColor>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let (entity, interaction, mut background_color, mut border_color, mut button, children) =
        start_button.into_inner();
    let mut start_game_text_color = text_color_query.get_mut(children[0]).unwrap();

    match *interaction {
        Interaction::Pressed => {
            input_focus.set(entity, FocusCause::Pressed);
            *background_color = BackgroundColor(BUTTON_BG_PRESS);
            *border_color = BorderColor::all(BUTTON_BORDER_PRESS);
            **start_game_text_color = BUTTON_TEXT_PRESS;
            button.set_changed();

            next_state.set(GameState::Paused);
        }

        Interaction::Hovered => {
            input_focus.set(entity, FocusCause::Pressed);
            *background_color = BackgroundColor(BUTTON_BG_HOVER);
            *border_color = BorderColor::all(BUTTON_BORDER_HOVER);
            **start_game_text_color = BUTTON_TEXT_HOVER;
            button.set_changed();
        }

        Interaction::None => {
            input_focus.clear();
            *background_color = BackgroundColor(BUTTON_BG_NORMAL);
            **start_game_text_color = BUTTON_TEXT_NORMAL;
            *border_color = BorderColor::all(BUTTON_BORDER_NORMAL);
        }
    }
}

fn hide_main_menu(visibility: Single<&mut Visibility, With<MainMenu>>) {
    *visibility.into_inner() = Visibility::Hidden;
}

fn show_main_menu(visibility: Single<&mut Visibility, With<MainMenu>>) {
    *visibility.into_inner() = Visibility::Visible;
}
