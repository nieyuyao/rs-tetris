use bevy::{
    asset::Handle,
    color::{Color, Srgba},
    ecs::{
        bundle::Bundle,
        component::Component,
        query::With,
        system::{Commands, Query, Res, ResMut, Single},
    },
    hierarchy::{BuildChildren, ChildBuild, ChildBuilder},
    math::Vec2,
    sprite::Anchor,
    text::{Font, FontSmoothing, Text2d, TextColor, TextFont},
    time::Time,
    transform::components::Transform,
    ui::Node,
    utils::default,
};
use bevy_prototype_lyon::{
    draw::{Fill, Stroke},
    entity::ShapeBundle,
    prelude::GeometryBuilder,
    shapes::{self, BorderRadii},
};
use chrono::{Local, Timelike};

use crate::{GameAssets, brick_node::BrickNode};
use crate::{
    constants::{
        BOARD_BRICK_NODE_COLS, BOARD_BRICK_NODE_ROWS, BRICK_NODE_GAP, BRICK_NODE_INNER_WIDTH,
        BRICK_NODE_WIDTH, BRICKS_CONTAINER_HEIGHT, BRICKS_CONTAINER_WIDTH,
    },
    game_data::GameData,
};

#[derive(Component)]
pub struct ScoreLabel;

#[derive(Component)]
pub struct ScoreText;

#[derive(Component)]
pub struct CleansLabel;

#[derive(Component)]
pub struct CleansText;

#[derive(Component)]
pub struct LevelLabel;

#[derive(Component)]
pub struct LevelText;

#[derive(Component)]
pub struct NextLabel;

#[derive(Component)]
pub struct MovingBrick;

#[derive(Component)]
pub struct NextBrickBoard;

#[derive(Component)]
pub struct NextBrick;

#[derive(Component)]
pub struct TimeText;

fn spawn_next_brick_board(commands: &mut ChildBuilder) {
    (0..4)
        .flat_map(|i| (0..4).map(move |j| BrickNode(i, j)))
        .for_each(|node| {
            spawn_brick_node(commands, node, 60., 46.);
        });
}

fn spawn_brick_node(commands: &mut ChildBuilder, node: BrickNode, offset_x: f32, offset_y: f32) {
    let x = offset_x + (node.0 as f32 + 0.5) * BRICK_NODE_WIDTH + (node.0 as f32) * BRICK_NODE_GAP;
    let y = offset_y
        - (node.1 as f32 + 0.5) * BRICK_NODE_WIDTH
        - (node.1 as f32) * BRICK_NODE_GAP
        - 3.0;
    commands
        .spawn((
            ShapeBundle {
                path: GeometryBuilder::build_as(&shapes::Rectangle {
                    extents: Vec2 {
                        x: BRICK_NODE_WIDTH as f32,
                        y: BRICK_NODE_WIDTH as f32,
                    },
                    ..shapes::Rectangle::default()
                }),
                transform: Transform::from_xyz(x, y, 100.0),
                ..default()
            },
            Stroke::new(Srgba::hex("#879372").unwrap(), 1.),
        ))
        .with_child((
            ShapeBundle {
                path: GeometryBuilder::build_as(&shapes::Rectangle {
                    extents: Vec2 {
                        x: BRICK_NODE_INNER_WIDTH as f32,
                        y: BRICK_NODE_INNER_WIDTH as f32,
                    },
                    ..shapes::Rectangle::default()
                }),
                ..default()
            },
            Fill::color(Srgba::hex("#879372").unwrap()),
        ));
}

fn spawn_board_brick_nodes(commands: &mut ChildBuilder) {
    (0..BOARD_BRICK_NODE_COLS)
        .flat_map(|i| return (0..BOARD_BRICK_NODE_ROWS).map(move |j| BrickNode(i, j)))
        .for_each(|node| {
            spawn_brick_node(
                commands,
                node,
                -BRICKS_CONTAINER_WIDTH / 2. - 37.0,
                BRICKS_CONTAINER_HEIGHT / 2.,
            )
        });
}

fn spawn_label(text: String, x: f32, y: f32) -> impl Bundle {
    (
        Text2d::new(text),
        TextColor(Color::BLACK),
        TextFont {
            font_size: 14.0,
            font_smoothing: FontSmoothing::AntiAliased,
            ..TextFont::default()
        },
        Transform::from_xyz(x, y, 100.),
        Anchor::TopRight,
    )
}

fn spawn_text(text: String, x: f32, y: f32, font: Handle<Font>) -> impl Bundle {
    (
        Text2d::new(text),
        TextColor(Color::BLACK),
        TextFont {
            font_size: 20.0,
            font_smoothing: FontSmoothing::AntiAliased,
            font,
            ..TextFont::default()
        },
        Transform::from_xyz(x, y, 100.),
        Anchor::TopRight,
    )
}

pub fn board_setup(mut commands: Commands, game_assets: Res<GameAssets>) {
    commands
        .spawn((
            ShapeBundle {
                path: GeometryBuilder::build_as(&shapes::Rectangle {
                    extents: Vec2 { x: 232., y: 302. },
                    radii: Some(BorderRadii::single(2.0)),
                    ..shapes::Rectangle::default()
                }),
                transform: Transform::from_xyz(0.0, 90.0, 0.0),
                ..default()
            },
            Fill::color(Srgba::hex("#fae36c").unwrap()),
        ))
        .with_children(|child_builder| {
            child_builder.spawn((
                ShapeBundle {
                    path: GeometryBuilder::build_as(&shapes::Rectangle {
                        extents: Vec2 { x: 240., y: 298. },
                        radii: Some(BorderRadii::single(2.0)),
                        ..shapes::Rectangle::default()
                    }),
                    transform: Transform::from_xyz(0.0, 0.0, 40.0),
                    ..default()
                },
                Fill::color(Srgba::hex("#9ead86").unwrap()),
            ));
            // bricks container
            child_builder.spawn((
                ShapeBundle {
                    path: GeometryBuilder::build_as(&shapes::Rectangle {
                        extents: Vec2 {
                            x: BRICKS_CONTAINER_WIDTH,
                            y: BRICKS_CONTAINER_HEIGHT,
                        },
                        ..shapes::Rectangle::default()
                    }),
                    transform: Transform::from_xyz(-40.0, 0.0, 60.0),
                    ..default()
                },
                Stroke::new(Color::BLACK, 2.0),
            ));
            spawn_board_brick_nodes(child_builder);
        });
    // score
    let padding_x: f32 = 110.0;
    commands
        .spawn(spawn_label("Score".into(), padding_x, 220.))
        .insert(ScoreLabel);

    commands
        .spawn(spawn_text(
            "0".into(),
            padding_x,
            200.0,
            game_assets.font.clone(),
        ))
        .insert(ScoreText);

    // Level
    commands
        .spawn(spawn_label("Level".into(), padding_x, 170.))
        .insert(LevelLabel);

    commands
        .spawn(spawn_text(
            "0".into(),
            padding_x,
            150.0,
            game_assets.font.clone(),
        ))
        .insert(LevelText);

    // cleans
    commands
        .spawn(spawn_label("Cleans".into(), padding_x, 120.))
        .insert(CleansLabel);

    commands
        .spawn(spawn_text(
            "0".into(),
            padding_x,
            100.0,
            game_assets.font.clone(),
        ))
        .insert(CleansText);

    // next
    commands
        .spawn(spawn_label("Next".into(), padding_x, 70.))
        .insert(NextLabel);

    // next brick board
    commands
        .spawn((NextBrickBoard, Node { ..default() }))
        .with_children(|child_builder| {
            spawn_next_brick_board(child_builder);
        });

    // time
    let now = Local::now();
    let hours = now.hour();
    let minutes = now.minute();
    commands
        .spawn(spawn_text(
            format!("{}:{}", hours, minutes),
            padding_x,
            -26.,
            game_assets.font.clone(),
        ))
        .insert(TimeText);
}

pub fn move_brick_system(time: Res<Time>, mut query: Query<(&MovingBrick)>) {}

pub fn get_speed() {}

pub fn get_score(level: u32, erase_lines: u32) -> u32 {
    assert!(0 < erase_lines);
    assert!(erase_lines <= 4);
    vec![40, 100, 300, 1200][(erase_lines - 1) as usize] * (level + 1)
}

pub fn clock_update_system(
    time: Res<Time>,
    mut game_data: ResMut<GameData>,
    time_text: Single<&mut Text2d, With<TimeText>>,
) {
    let ticked = game_data.clock_timer.tick(time.delta()).finished();
    if ticked {
        let now = Local::now();
        let hours = now.hour();
        let minutes = now.minute();
        let mut text = time_text.into_inner();
        text.clear();
        text.push_str(format!("{}:{}", hours, minutes).as_str());
    }
}
