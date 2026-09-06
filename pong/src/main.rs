use std::f32::consts::PI;

use bevy::camera::ScalingMode;
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

// TODO: Common "Velocity"/"Moving" Component
// TODO: Slow start velocity
// TODO: Main Menu
// TODO: AI

// State
#[derive(States, Debug, Hash, Eq, PartialEq, Clone)]
enum GameState {
    Paused,
    InGame,
}

// Components
#[derive(Component)]
struct Moving {
    speed: f32,
    direction: Vec3,
}

#[derive(Component)]
struct Bar(u8);

#[derive(Component)]
struct Ball;

#[derive(Component)]
struct ScoreText;

#[derive(Component)]
struct PauseText;

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
        app.insert_state(GameState::Paused);
        app.add_systems(Startup, setup);
        app.add_systems(
            Update,
            (
                (bar_input, movement, clamp_bars, collision, ball_bounds)
                    .chain()
                    .run_if(in_state(GameState::InGame)),
                toggle_pause,
                update_score_text,
            ),
        );
        app.add_systems(OnEnter(GameState::Paused), show_pause_text);
        app.add_systems(OnExit(GameState::Paused), hide_pause_text);
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

    let color = Color::WHITE;
    let material = materials.add(color);

    spawn_bar(0, &material, &mut commands, &mut meshes);
    spawn_bar(1, &material, &mut commands, &mut meshes);

    let ball_mesh = meshes.add(Circle::new(BALL_RADIUS));
    commands.spawn((
        Ball,
        Moving {
            speed: BALL_SPEED,
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
        Visibility::Visible,
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
                WINDOW_WIDTH as f32 / 2.0 - 5.0
            } else {
                -(WINDOW_WIDTH as f32) / 2.0 + 5.0
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
            if keys.pressed(KeyCode::ArrowUp) {
                direction.y += 1.0;
            }
            if keys.pressed(KeyCode::ArrowDown) {
                direction.y -= 1.0;
            }
        } else {
            if keys.pressed(KeyCode::KeyW) {
                direction.y += 1.0;
            }
            if keys.pressed(KeyCode::KeyS) {
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
    mut score: ResMut<Score>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let (mut sliding, mut transform) = ball.into_inner();

    if transform.translation.y - BALL_RADIUS <= -(WINDOW_HEIGHT as f32) / 2.0 {
        transform.translation.y = -(WINDOW_HEIGHT as f32) / 2.0 + BALL_RADIUS + 0.1;
        sliding.direction.y *= -1.0;
    }
    if transform.translation.y + BALL_RADIUS >= WINDOW_HEIGHT as f32 / 2.0 {
        transform.translation.y = WINDOW_HEIGHT as f32 / 2.0 - BALL_RADIUS + 0.1;
        sliding.direction.y *= -1.0;
    }

    if transform.translation.x >= WINDOW_WIDTH as f32 / 2.0
        || transform.translation.x <= -(WINDOW_WIDTH as f32) / 2.0
    {
        next_state.set(GameState::Paused);

        if transform.translation.x > 0.0 {
            score.p2 += 1;
        } else {
            score.p1 += 1;
        }

        transform.translation = Vec3::new(0.0, 0.0, 0.0);

        let mut rng = rand::rng();
        sliding.direction = Vec3::new(
            (rng.random_range(-1.0..1.0) as f32).signum(),
            rng.random_range(-0.25..0.25),
            0.0,
        );
    }
}

#[allow(clippy::type_complexity)]
fn collision(
    bars: Query<&Transform, (With<Bar>, Without<Ball>)>,
    ball: Single<(&mut Transform, &mut Moving), (With<Ball>, Without<Bar>)>,
    sound_effect: Res<SoundEffect>,
    mut commands: Commands,
) {
    let (mut ball_transform, mut ball_moving) = ball.into_inner();

    for bar_transform in &bars {
        let aabb = Vec3::new(BAR_WIDTH / 2.0, BAR_HEIGHT / 2.0, 0.0);

        let distance = ball_transform.translation - bar_transform.translation;
        let clamped_distance = distance.clamp(-aabb, aabb);
        let closest_point = bar_transform.translation + clamped_distance;

        let collision = (ball_transform.translation - closest_point).length() <= BALL_RADIUS;

        if collision {
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
        });
    }
}

fn update_score_text(score_text: Single<&mut Text, With<ScoreText>>, score: Res<Score>) {
    if score.is_changed() {
        score_text.into_inner().0 = format!("{:02} | {:02}", score.p2, score.p1);
    }
}

fn show_pause_text(pause_text: Single<&mut Visibility, With<PauseText>>) {
    *pause_text.into_inner() = Visibility::Visible;
}

fn hide_pause_text(pause_text: Single<&mut Visibility, With<PauseText>>) {
    *pause_text.into_inner() = Visibility::Hidden;
}
