use bevy::prelude::*;

use crate::{arena::ARENA_HEIGHT, movement::Movement, state::SceneState};

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(SceneState::InGame), spawn_ball);
    }
}

const BALL_SPEED: f32 = 500.0;
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
            speed: BALL_SPEED,
            direction: -Vec3::Y,
        },
        Mesh2d(mesh),
        MeshMaterial2d(material),
        Transform::from_xyz(0.0, -(ARENA_HEIGHT / 2.0) + ARENA_HEIGHT / 5.0, 0.0),
        DespawnOnExit(SceneState::InGame),
    ));
}
