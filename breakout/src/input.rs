use bevy::prelude::*;

use crate::{bar::Bar, movement::Movement, schedule::GameSet};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, bar_input.in_set(GameSet::Input));
    }
}

fn bar_input(bar_query: Single<&mut Movement, With<Bar>>, input: Res<ButtonInput<KeyCode>>) {
    let mut movement = bar_query.into_inner();

    let mut direction = Vec3::ZERO;

    if input.pressed(KeyCode::ArrowLeft) || input.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }
    if input.pressed(KeyCode::ArrowRight) || input.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }

    movement.direction = direction.normalize_or_zero();
}
