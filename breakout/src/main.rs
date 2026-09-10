mod arena;
mod ball;
mod bar;
mod breakout;
mod brick;
mod collision;
mod input;
mod movement;
mod schedule;
mod state;

use crate::breakout::BreakoutPlugin;

use bevy::prelude::{
    App, ClearColor, Color, DefaultPlugins, PluginGroup, Window, WindowPlugin, default,
};
use bevy_dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin};

const WINDOW_WIDTH: u32 = 1000;
const WINDOW_HEIGHT: u32 = 500;

const WINDOW_COLOR: Color = Color::srgb(0.369, 0.788, 0.969);

fn main() {
    App::new()
        .insert_resource(ClearColor(WINDOW_COLOR))
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                    ..default()
                }),
                ..default()
            }),
            FpsOverlayPlugin {
                config: FpsOverlayConfig { ..default() },
            },
        ))
        .add_plugins(BreakoutPlugin)
        .run();
}
