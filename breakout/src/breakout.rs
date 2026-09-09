use crate::{
    arena::ArenaPlugin, ball::BallPlugin, bar::BarPlugin, input::InputPlugin,
    movement::MovementPlugin, schedule::SchedulePlugin, state::StatePlugin,
};

use bevy::{camera::ScalingMode, prelude::*};

pub struct BreakoutPlugin;

impl Plugin for BreakoutPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            SchedulePlugin,
            StatePlugin,
            InputPlugin,
            BarPlugin,
            BallPlugin,
            ArenaPlugin,
            MovementPlugin,
        ));

        app.add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: crate::arena::ARENA_HEIGHT,
            },
            ..OrthographicProjection::default_2d()
        }),
    ));
}
