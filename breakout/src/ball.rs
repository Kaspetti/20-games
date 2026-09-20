use bevy::prelude::*;

use crate::{
    arena::ARENA_HEIGHT,
    brick::{BRICK_COLUMNS, BRICK_ROWS, BrickBroken, BrokenBricks},
    movement::Movement,
    state::SceneState,
};
use rand::RngExt;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(SceneState::InGame), spawn_ball);
        app.add_observer(update_speed);
    }
}

const BALL_START_SPEED: f32 = 500.0;
const BALL_END_SPEED: f32 = 1000.0;
const BALL_RADIUS: f32 = 10.0;
const BALL_COLOR: Color = Color::srgb(1.0, 1.0, 0.0);

#[derive(Component)]
pub struct Ball {
    pub radius: f32,
}

fn spawn_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mesh = meshes.add(Circle::new(BALL_RADIUS));
    let material = materials.add(BALL_COLOR);

    commands.spawn((
        Ball {
            radius: BALL_RADIUS,
        },
        Movement {
            speed: BALL_START_SPEED,
            direction: Vec3::new(rand::rng().random_range(-0.1..0.1), -1.0, 0.0),
        },
        Mesh2d(mesh),
        MeshMaterial2d(material),
        Transform::from_xyz(0.0, -(ARENA_HEIGHT / 2.0) + ARENA_HEIGHT / 5.0, 0.0),
        DespawnOnExit(SceneState::InGame),
    ));
}

fn update_speed(
    _: On<BrickBroken>,
    ball_movement: Single<&mut Movement, With<Ball>>,
    broken_bricks: Res<BrokenBricks>,
) {
    let mut movement = ball_movement.into_inner();

    if broken_bricks.0 == 0.0 {
        movement.speed = BALL_START_SPEED;
    }

    movement.speed = BALL_START_SPEED
        + (broken_bricks.0 / (BRICK_COLUMNS * BRICK_ROWS) as f32)
            * (BALL_END_SPEED - BALL_START_SPEED);

    println!("{}", movement.speed);
}
