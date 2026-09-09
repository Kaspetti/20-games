use bevy::prelude::*;

use crate::state::SceneState;

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
                .chain()
                .run_if(in_state(SceneState::InGame)),
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
