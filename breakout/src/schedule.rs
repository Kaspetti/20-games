use bevy::prelude::*;

pub struct SchedulePlugin;

impl Plugin for SchedulePlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (
                GameSet::Input,
                GameSet::Movement,
                GameSet::Bounds,
                GameSet::Collision,
            )
                .chain(),
        );
    }
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameSet {
    Input,
    Movement,
    Bounds,
    Collision,
}
