mod board;
mod brick_node;
mod control;
mod decorate;
mod game_data;
mod state;
mod constants;

use bevy::{prelude::*, window::WindowResolution};
use bevy_prototype_lyon::plugin::ShapePlugin;
use game_data::GameData;
use state::GameSate;

use crate::{control::control_setup, decorate::decorate_setup, constants::DESIGN_SIZE};

fn scene_setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn main() {
    let game_data = GameData::default();
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "俄罗斯方块".into(),
                resizable: false,
                resolution: WindowResolution::new(
                    DESIGN_SIZE.x,
                    DESIGN_SIZE.y,
                ),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(ShapePlugin)
        .init_state::<GameSate>()
        .insert_resource(game_data)
        .add_systems(Startup, scene_setup)
        .add_systems(Startup, (control_setup, decorate_setup))
        .run();
}
