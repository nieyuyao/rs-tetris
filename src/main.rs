mod animation;
mod board;
mod brick;
mod constants;
mod control;
mod decorate;
mod game_data;
mod state;

use bevy::{prelude::*, window::WindowResolution};
use bevy_prototype_lyon::plugin::ShapePlugin;
use game_data::GameData;
use state::GameSate;
use std::time::Duration;

use crate::{
    animation::{play_ready_animation, AnimationIndices, AnimationTimer},
    board::{
        board_setup, clock_update_system, falling_brick_system, game_over_system, score_board_system, spawn_falling_brick, spawn_next_brick
    },
    brick::BrickShape,
    constants::DESIGN_SIZE,
    control::{control_drop_to_start_game, control_game_system, control_on_click, control_setup},
    decorate::decorate_setup,
};

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
    volume: Handle<Image>,
    pause: Handle<Image>,
    dino: Handle<Image>,
    font: Handle<Font>,
}

fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut next_state: ResMut<NextState<GameSate>>,
) {
    commands.insert_resource(GameAssets {
        move_button: asset_server.load("../assets/move_button.png"),
        move_button_pressed: asset_server.load("../assets/move_button_pressed.png"),
        replay_button: asset_server.load("../assets/replay_button.png"),
        replay_button_pressed: asset_server.load("../assets/replay_button_pressed.png"),
        effect_button: asset_server.load("../assets/effect_button.png"),
        effect_button_pressed: asset_server.load("../assets/effect_button_pressed.png"),
        pause: asset_server.load("../assets/pause.png"),
        volume: asset_server.load("../assets/volume.png"),
        dino: asset_server.load("../assets/dino.png"),
        font: asset_server.load("../assets/digital7mono.ttf"),
    });

    next_state.set(GameSate::Ready);
}

fn start_game(
    mut commands: Commands,
    query: Single<Entity, (With<Sprite>, With<AnimationIndices>)>,
    mut game_data: ResMut<GameData>,
) {
    println!("start game!!!");
    let ready_animation_entity = query.into_inner();
    commands.entity(ready_animation_entity).despawn();
    game_data.falling_brick_shape = BrickShape::rand();
    game_data.next_brick_shape = BrickShape::rand();
    spawn_falling_brick(
        &mut commands,
        game_data.falling_brick_shape.into(),
        game_data.falling_brick_node,
    );
    spawn_next_brick(&mut commands, game_data.next_brick_shape.into());
}

fn ready_game(mut game_data: ResMut<GameData>) {
    game_data.playing_ready_animation_duration = Duration::default();
    game_data.is_playing_dino_running_animation = true;
}

fn spawn_ready_animation_sprite(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = game_assets.dino.clone();
    let layout = TextureAtlasLayout::from_grid(UVec2::new(80, 86), 4, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let animation_indices = AnimationIndices { first: 2, last: 3 };
    commands.spawn((
        Sprite::from_atlas_image(
            texture,
            TextureAtlas {
                layout: texture_atlas_layout,
                index: animation_indices.first,
            },
        ),
        Transform::from_xyz(-40., 120., 300.),
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    ));
}

fn pause_game_system() {
    // TODO:
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
        .add_systems(
            Startup,
            (scene_setup, decorate_setup, control_setup, board_setup).chain(),
        )
        .add_systems(
            OnEnter(GameSate::Ready),
            (ready_game, spawn_ready_animation_sprite),
        )
        .add_systems(OnEnter(GameSate::Playing), start_game)
        .add_systems(Update, control_on_click)
        .add_systems(
            Update,
            (
                control_game_system,
                falling_brick_system,
                score_board_system,
            )
                .run_if(in_state(GameSate::Playing)),
        )
        .add_systems(Update, clock_update_system)
        .add_systems(
            Update,
            (play_ready_animation, control_drop_to_start_game).run_if(in_state(GameSate::Ready)),
        )
        .add_systems(OnEnter(GameSate::Paused), pause_game_system)
        .add_systems(OnEnter(GameSate::GameOver), game_over_system)
        .run();
}
