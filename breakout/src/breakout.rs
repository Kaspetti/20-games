use crate::bar::BarPlugin;

use bevy::prelude::{App, Plugin};

pub struct BreakoutPlugin;

impl Plugin for BreakoutPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(BarPlugin);
    }
}
