use bevy::prelude::*;

use crate::{
    arena::{ARENA_HEIGHT, ARENA_WIDTH, WALL_THICKNESS},
    collision::{Collider, ColliderType},
    movement::Movement,
    schedule::GameSet,
    state::SceneState,
};

pub struct BarPlugin;
impl Plugin for BarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(SceneState::InGame), spawn_bar);
        app.add_systems(Update, clamp_system.in_set(GameSet::Bounds));
    }
}

const BAR_COLOR: Color = Color::srgb(1.0, 0.0, 0.0);
const BAR_WIDTH: f32 = 100.0;
const BAR_HEIGHT: f32 = 10.0;

const BAR_SPEED: f32 = 500.0;

#[derive(Component)]
pub struct Bar;

fn spawn_bar(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let material = materials.add(BAR_COLOR);
    let bar_mesh = meshes.add(Rectangle::new(BAR_WIDTH, BAR_HEIGHT));

    commands.spawn((
        Bar,
        Collider {
            collider_type: ColliderType::Bar,
            aabb: Vec3::new(BAR_WIDTH / 2.0, BAR_HEIGHT / 2.0, 0.0),
        },
        Movement {
            speed: BAR_SPEED,
            direction: Vec3::ZERO,
        },
        Mesh2d(bar_mesh),
        MeshMaterial2d(material.clone()),
        Transform::from_xyz(0.0, -(ARENA_HEIGHT / 2.0) + ARENA_HEIGHT / 10.0, 0.0),
        DespawnOnExit(SceneState::InGame),
    ));
}

fn clamp_system(bar_q: Single<&mut Transform, With<Bar>>) {
    let mut transform = bar_q.into_inner();

    transform.translation.x = transform.translation.x.clamp(
        (BAR_WIDTH - ARENA_WIDTH) / 2.0,
        (ARENA_WIDTH - BAR_WIDTH) / 2.0,
    )
}
