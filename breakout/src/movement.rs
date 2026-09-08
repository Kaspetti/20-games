use bevy::prelude::*;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {}
}

#[derive(Component)]
struct Movement {
    speed: f32,
    direction: Vec3,
}
