use bevy::prelude::*;

use crate::{
    collision::{Collider, ColliderType},
    state::SceneState,
};

pub struct ArenaPlugin;

impl Plugin for ArenaPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(SceneState::InGame), spawn_arena);
    }
}

pub const ARENA_WIDTH: f32 = 1000.0;
pub const ARENA_HEIGHT: f32 = 1000.0;

const WALL_COLOR: Color = Color::BLACK;
pub const WALL_THICKNESS: f32 = 15.0;

fn spawn_arena(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let material = materials.add(WALL_COLOR);

    // Spawn walls
    let wall_mesh = meshes.add(Rectangle::new(WALL_THICKNESS, ARENA_HEIGHT));
    commands.spawn((
        Collider {
            collider_type: ColliderType::Wall,
            aabb: Vec3::new(WALL_THICKNESS / 2.0, ARENA_HEIGHT / 2.0, 0.0),
        },
        Mesh2d(wall_mesh.clone()),
        MeshMaterial2d(material.clone()),
        Transform::from_xyz((ARENA_WIDTH - WALL_THICKNESS) / 2.0, 0.0, 0.0),
        DespawnOnExit(SceneState::InGame),
    ));
    commands.spawn((
        Collider {
            collider_type: ColliderType::Wall,
            aabb: Vec3::new(WALL_THICKNESS / 2.0, ARENA_HEIGHT / 2.0, 0.0),
        },
        Mesh2d(wall_mesh.clone()),
        MeshMaterial2d(material.clone()),
        Transform::from_xyz(-(ARENA_WIDTH - WALL_THICKNESS) / 2.0, 0.0, 0.0),
        DespawnOnExit(SceneState::InGame),
    ));

    // Spawn ceiling
    let ceiling_mesh = meshes.add(Rectangle::new(ARENA_WIDTH, WALL_THICKNESS));
    commands.spawn((
        Collider {
            collider_type: ColliderType::Ceiling,
            aabb: Vec3::new(ARENA_WIDTH / 2.0, WALL_THICKNESS / 2.0, 0.0),
        },
        Mesh2d(ceiling_mesh),
        MeshMaterial2d(material.clone()),
        Transform::from_xyz(0.0, (ARENA_HEIGHT - WALL_THICKNESS) / 2.0, 0.0),
        DespawnOnExit(SceneState::InGame),
    ));
}
