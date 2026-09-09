use crate::{
    bar::BarPlugin, movement::MovementPlugin, schedule::SchedulePlugin, state::StatePlugin,
};

use bevy::{camera::ScalingMode, prelude::*};

pub struct BreakoutPlugin;

impl Plugin for BreakoutPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((SchedulePlugin, StatePlugin, BarPlugin, MovementPlugin));

        app.add_systems(Startup, setup);
    }
}

const WORLD_WIDTH: f32 = 1280.0;
const WORLD_HEIGHT: f32 = 1280.0;

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::Fixed {
                width: WORLD_WIDTH,
                height: WORLD_HEIGHT,
            },
            ..OrthographicProjection::default_2d()
        }),
    ));
}
