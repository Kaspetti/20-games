use bevy::prelude::*;

use crate::breakout::MovementSet;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, movement_system.in_set(MovementSet::Movement));
    }
}

#[derive(Component)]
struct Movement {
    speed: f32,
    direction: Vec3,
}

fn movement_system(mut query: Query<(&mut Transform, &Movement)>, time: Res<Time>) {
    for (mut transform, movement) in &mut query {
        transform.translation += movement.direction * movement.speed * time.delta_secs();
    }
}
