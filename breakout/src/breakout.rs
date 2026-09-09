use crate::state::StatePlugin;
use crate::{bar::BarPlugin, movement::MovementPlugin};

use bevy::{camera::ScalingMode, prelude::*};

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum MovementSet {
    Input,
    Movement,
    Bounds,
    Collision,
}

pub struct BreakoutPlugin;

impl Plugin for BreakoutPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(StatePlugin);
        app.add_plugins(BarPlugin);

        app.configure_sets(
            Update,
            (
                MovementSet::Input,
                MovementSet::Movement,
                MovementSet::Bounds,
                MovementSet::Collision,
            )
                .chain(),
        );

        app.add_plugins(MovementPlugin);

        app.add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::Fixed {
                width: crate::WINDOW_WIDTH as f32,
                height: crate::WINDOW_HEIGHT as f32,
            },
            ..OrthographicProjection::default_2d()
        }),
    ));
}
