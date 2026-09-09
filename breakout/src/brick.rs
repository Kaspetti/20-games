use bevy::prelude::*;

use crate::{
    arena::ARENA_WIDTH,
    collision::{Collider, ColliderType},
    state::SceneState,
};

pub struct BrickPlugin;
impl Plugin for BrickPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(SceneState::InGame), spawn_bricks);
    }
}

const BRICK_ROWS: u32 = 8;
const BRICK_COLUMNS: u32 = 16;
const BRICK_GAP: f32 = 5.0;

const BRICK_WIDTH: f32 =
    (ARENA_WIDTH - BRICK_GAP * (BRICK_COLUMNS as f32 + 1.0)) / BRICK_COLUMNS as f32;
const BRICK_HEIGHT: f32 = 10.0;
const BRICK_COLOR: Color = Color::srgb(0.0, 1.0, 0.0);

fn spawn_bricks(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let material = materials.add(BRICK_COLOR);
    let bar_mesh = meshes.add(Rectangle::new(BRICK_WIDTH, BRICK_HEIGHT));

    for y in 1..BRICK_ROWS {
        for x in 1..BRICK_COLUMNS {
            commands.spawn((
                Collider {
                    collider_type: ColliderType::Brick,
                    aabb: Vec3::new(BRICK_WIDTH / 2.0, BRICK_HEIGHT / 2.0, 0.0),
                },
                Mesh2d(bar_mesh.clone()),
                MeshMaterial2d(material.clone()),
                Transform::from_xyz(
                    -ARENA_WIDTH / 2.0 + (BRICK_GAP + BRICK_WIDTH / 2.0) * x as f32,
                    0.0,
                    0.0,
                ),
                DespawnOnExit(SceneState::InGame),
            ));
        }
    }
}
