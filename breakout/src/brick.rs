use bevy::prelude::*;

use crate::{
    arena::{ARENA_HEIGHT, ARENA_WIDTH},
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
const BRICK_GAP: f32 = 10.0;

const BRICK_WIDTH: f32 =
    (ARENA_WIDTH - BRICK_GAP * (BRICK_COLUMNS as f32 + 1.0)) / BRICK_COLUMNS as f32;
const BRICK_HEIGHT: f32 = 20.0;

const BRICK_COLORS: &[Color] = &[
    Color::srgb(1.0, 0.0, 0.0),
    Color::srgb(1.0, 0.647, 0.0),
    Color::srgb(1.0, 1.0, 0.0),
    Color::srgb(0.0, 1.0, 0.0),
];

fn spawn_bricks(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let bar_mesh = meshes.add(Rectangle::new(BRICK_WIDTH, BRICK_HEIGHT));

    let material_handles: Vec<Handle<ColorMaterial>> = BRICK_COLORS
        .iter()
        .map(|&color| materials.add(color))
        .collect();

    let x_start = -(ARENA_WIDTH / 2.0) + BRICK_GAP + BRICK_WIDTH / 2.0;
    let x_step = BRICK_GAP + BRICK_WIDTH;

    let y_start = ARENA_HEIGHT / 2.0 - (BRICK_GAP + BRICK_HEIGHT / 2.0);
    let y_step = BRICK_GAP + BRICK_HEIGHT;

    for y in 0..BRICK_ROWS {
        let y = y as f32;
        for x in 0..BRICK_COLUMNS {
            let x = x as f32;
            commands.spawn((
                Collider {
                    collider_type: ColliderType::Brick,
                    aabb: Vec3::new(BRICK_WIDTH / 2.0, BRICK_HEIGHT / 2.0, 0.0),
                },
                Mesh2d(bar_mesh.clone()),
                MeshMaterial2d(material_handles[(y / 2.0).floor() as usize].clone()),
                Transform::from_xyz(x_start + x_step * x, y_start - y_step * y, 0.0),
                DespawnOnExit(SceneState::InGame),
            ));
        }
    }
}
