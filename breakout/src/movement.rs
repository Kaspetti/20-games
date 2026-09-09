use bevy::prelude::*;

use crate::schedule::GameSet;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, movement_system.in_set(GameSet::Movement));
    }
}

#[derive(Component)]
pub struct Movement {
    pub speed: f32,
    pub direction: Vec3,
}

fn movement_system(mut query: Query<(&mut Transform, &Movement)>, time: Res<Time>) {
    for (mut transform, movement) in &mut query {
        transform.translation += movement.direction * movement.speed * time.delta_secs();
    }
}
