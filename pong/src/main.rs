use std::f32::consts::PI;

use bevy::camera::ScalingMode;
use bevy::prelude::*;
use rand::RngExt;

const BAR_SPEED: f32 = 1000.0;
const BAR_WIDTH: f32 = 5.0;
const BAR_HEIGHT: f32 = 150.0;

const BALL_SPEED: f32 = 1500.0;
const BALL_RADIUS: f32 = 5.0;
const BALL_MAX_ANGLE: f32 = 45.0;

const WINDOW_WIDTH: u32 = 1280;
const WINDOW_HEIGHT: u32 = 720;

#[derive(Component)]
struct Movable {
    speed: f32,
}

#[derive(Component)]
struct Sliding {
    speed: f32,
    direction: Vec3,
}

#[derive(Component, Default)]
struct Collider;

#[derive(Component)]
#[require(Collider)]
struct Bar(u8);

#[derive(Component)]
#[require(Collider)]
struct Ball;

#[derive(Component)]
struct ScoreText;

#[derive(Component)]
struct PauseText;

#[derive(Resource)]
struct IsPaused(bool);

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
        app.insert_resource(IsPaused(true));
        app.insert_resource(Score { p1: 0, p2: 0 });
        app.init_resource::<SoundEffect>();
        app.add_systems(Startup, setup);
        app.add_systems(
            Update,
            (
                ((movement, sliding), collision)
                    .chain()
                    .run_if(run_if_unpaused),
                pause_handler,
                update_score_text,
                paused_text_visibility,
            ),
        );
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
        Sliding {
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
        Movable { speed: BAR_SPEED },
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

fn movement(
    mut bars: Query<(&Bar, &Movable, &mut Transform)>,
    timer: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    for (bar, movable, mut transform) in &mut bars {
        let mut direction = Vec3::new(0.0, 0.0, 0.0);

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

        transform.translation += direction * movable.speed * timer.delta_secs();

        transform.translation.y = transform.translation.y.clamp(
            (-(WINDOW_HEIGHT as f32) + BAR_HEIGHT) / 2.0,
            (WINDOW_HEIGHT as f32 - BAR_HEIGHT) / 2.0,
        );
    }
}

fn sliding(
    ball: Single<(&mut Sliding, &mut Transform), With<Ball>>,
    timer: Res<Time>,
    mut paused: ResMut<IsPaused>,
    mut score: ResMut<Score>,
) {
    let (mut sliding, mut transform) = ball.into_inner();
    transform.translation += sliding.direction.normalize() * sliding.speed * timer.delta_secs();

    if transform.translation.y - BALL_RADIUS <= -(WINDOW_HEIGHT as f32) / 2.0 {
        transform.translation.y = -(WINDOW_HEIGHT as f32) / 2.0 + BALL_RADIUS + 0.1;
        sliding.direction.y *= -1.0;
    }
    if transform.translation.y + BALL_RADIUS >= WINDOW_HEIGHT as f32 / 2.0 {
        transform.translation.y = WINDOW_HEIGHT as f32 / 2.0 - BALL_RADIUS - 0.1;
        sliding.direction.y *= -1.0;
    }

    // TODO: Slide ball out of edge when touching to prevent "stuck" on edge

    if transform.translation.x >= WINDOW_WIDTH as f32 / 2.0
        || transform.translation.x <= -(WINDOW_WIDTH as f32) / 2.0
    {
        paused.0 = true;

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
    bars: Query<&Transform, (With<Bar>, With<Collider>, Without<Ball>)>,
    ball: Single<(&Transform, &mut Sliding), (With<Ball>, With<Collider>)>,
    sound_effect: Res<SoundEffect>,
    mut commands: Commands,
) {
    let (ball_transform, mut ball_sliding) = ball.into_inner();

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

            ball_sliding.direction = Vec3::new(
                -sign * ops::cos(-sign * bounce_angle),
                -sign * ops::sin(-sign * bounce_angle),
                0.0,
            );
        }
    }
}

fn pause_handler(mut paused: ResMut<IsPaused>, keys: Res<ButtonInput<KeyCode>>) {
    if keys.just_pressed(KeyCode::Space) {
        paused.0 = !paused.0;
    }
}

fn update_score_text(score_text: Single<&mut Text, With<ScoreText>>, score: Res<Score>) {
    if score.is_changed() {
        score_text.into_inner().0 = format!("{:02} | {:02}", score.p2, score.p1);
    }
}

fn paused_text_visibility(
    pause_text: Single<&mut Visibility, With<PauseText>>,
    paused: Res<IsPaused>,
) {
    if paused.is_changed() {
        *pause_text.into_inner() = if paused.0 {
            Visibility::Visible
        } else {
            Visibility::Hidden
        }
    }
}

fn run_if_unpaused(paused: Res<IsPaused>) -> bool {
    !paused.0
}
