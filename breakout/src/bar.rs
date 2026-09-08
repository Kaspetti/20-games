use bevy::prelude::*;

use crate::state::SceneState;

pub struct BarPlugin;
impl Plugin for BarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(SceneState::InGame), setup);
    }
}

const BAR_COLOR: Color = Color::srgb(1.0, 0.0, 0.0);
const BAR_WIDTH: f32 = 100.0;
const BAR_HEIGHT: f32 = 10.0;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let material = materials.add(BAR_COLOR);
    let bar_mesh = meshes.add(Rectangle::new(BAR_WIDTH, BAR_HEIGHT));

    commands.spawn((
        Mesh2d(bar_mesh),
        MeshMaterial2d(material.clone()),
        Transform::from_xyz(0.0, -(crate::WINDOW_HEIGHT as f32) / 2.0 + 100.0, 0.0),
        DespawnOnExit(SceneState::InGame),
    ));
}
