mod board;
mod brick_node;
mod constants;
mod control;
mod decorate;
mod game_data;
mod state;

use bevy::{prelude::*, window::WindowResolution};
use bevy_prototype_lyon::plugin::ShapePlugin;
use game_data::GameData;
use state::GameSate;

use crate::{board::board_setup, constants::DESIGN_SIZE, control::{control_on_click, control_setup}, decorate::decorate_setup};

fn scene_setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

#[derive(Resource)]
pub struct GameAssets {
    move_button: Handle<Image>,
    move_button_pressed: Handle<Image>,
    replay_button: Handle<Image>,
    replay_button_pressed: Handle<Image>,
    effect_button: Handle<Image>,
    effect_button_pressed: Handle<Image>,
    font: Handle<Font>
}

fn load_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(GameAssets {
        move_button: asset_server.load("../assets/move_button.png"),
        move_button_pressed: asset_server.load("../assets/move_button_pressed.png"),
        replay_button: asset_server.load("../assets/replay_button.png"),
        replay_button_pressed: asset_server.load("../assets/replay_button_pressed.png"),
        effect_button: asset_server.load("../assets/effect_button.png"),
        effect_button_pressed: asset_server.load("../assets/effect_button_pressed.png"),
        font: asset_server.load("../assets/digital7mono.ttf")
    });
}

fn main() {
    let game_data = GameData::default();
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "俄罗斯方块".into(),
                resizable: false,
                resolution: WindowResolution::new(DESIGN_SIZE.x, DESIGN_SIZE.y),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(ShapePlugin)
        .init_state::<GameSate>()
        .insert_resource(game_data)
        .add_systems(PreStartup, load_assets)
        .add_systems(Startup, scene_setup)
        .add_systems(Startup, (decorate_setup, control_setup, board_setup))
        .add_systems(Update, control_on_click)
        .run();
}
