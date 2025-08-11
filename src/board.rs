use std::time::Duration;

use bevy::{
    asset::Handle,
    color::{Color, Srgba},
    ecs::{
        bundle::Bundle,
        component::Component,
        entity::Entity,
        query::{With, Without},
        system::{Commands, ParamSet, Query, Res, ResMut, Single},
    },
    hierarchy::{BuildChildren, ChildBuild, ChildBuilder, Children, DespawnRecursiveExt},
    math::Vec2,
    render::view::Visibility,
    sprite::{Anchor, Sprite},
    state::state::NextState,
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

use crate::{
    GameAssets,
    brick::{Brick, BrickNode, BrickShape, get_brick_node_position},
    constants::TIMER_FALLING_SECS,
    state::GameSate,
};
use crate::{
    constants::{
        BOARD_BRICK_NODE_COLS, BOARD_BRICK_NODE_ROWS, BRICK_NODE_WIDTH,
        BRICKS_CONTAINER_BOUNDING_LEFT, BRICKS_CONTAINER_BOUNDING_TOP, BRICKS_CONTAINER_HEIGHT,
        BRICKS_CONTAINER_WIDTH,
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
pub struct FallingBrickNode;

#[derive(Component)]
pub struct FallingBrick;

#[derive(Component)]
pub struct NextBrickBoard;

#[derive(Component)]
pub struct NextBrick;

#[derive(Component)]
pub struct BoardBrickNode;

#[derive(Component)]
pub struct TimeText;

fn spawn_next_brick_board(commands: &mut ChildBuilder) {
    (0..4)
        .flat_map(|i| (0..4).map(move |j| BrickNode(i, j)))
        .for_each(|node| {
            let x = 60.0 + (node.0 as f32 + 0.5) * BRICK_NODE_WIDTH;
            let y = 43.0 - (node.1 as f32 + 0.5) * BRICK_NODE_WIDTH;
            spawn_brick_node(commands, x, y, "#9ead86", "#879372", (), || false);
        });
}

fn spawn_brick_node<F>(
    commands: &mut ChildBuilder,
    x: f32,
    y: f32,
    fill_color: &str,
    stroke_color: &str,
    bundle: impl Bundle,
    condition: F,
) where
    F: FnOnce() -> bool,
{
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
                transform: Transform::from_xyz(x, y, 80.0),
                ..default()
            },
            Fill::color(Srgba::hex(fill_color).unwrap()),
        ))
        .with_child((
            ShapeBundle {
                path: GeometryBuilder::build_as(&shapes::Rectangle {
                    extents: Vec2 {
                        x: BRICK_NODE_WIDTH - 2. as f32,
                        y: BRICK_NODE_WIDTH - 2. as f32,
                    },
                    ..shapes::Rectangle::default()
                }),
                transform: Transform::from_xyz(0., 0., 100.0),
                ..default()
            },
            Stroke::new(Srgba::hex(stroke_color).unwrap(), 1.),
        ))
        .with_child((
            ShapeBundle {
                path: GeometryBuilder::build_as(&shapes::Rectangle {
                    extents: Vec2 {
                        x: BRICK_NODE_WIDTH - 6. as f32,
                        y: BRICK_NODE_WIDTH - 6. as f32,
                    },
                    ..shapes::Rectangle::default()
                }),
                transform: Transform::from_xyz(0., 0., 120.0),
                ..default()
            },
            Fill::color(Srgba::hex(stroke_color).unwrap()),
        ))
        .insert_if(bundle, condition);
}

fn spawn_board(commands: &mut ChildBuilder) {
    (0..BOARD_BRICK_NODE_COLS)
        .flat_map(|i| return (0..BOARD_BRICK_NODE_ROWS).map(move |j| BrickNode(i as i8, j as i8)))
        .for_each(|node| {
            let x = BRICKS_CONTAINER_BOUNDING_LEFT + (node.0 as f32 + 0.5) * BRICK_NODE_WIDTH;
            let y = BRICKS_CONTAINER_BOUNDING_TOP - (20. - node.1 as f32 - 0.5) * BRICK_NODE_WIDTH;
            spawn_brick_node(
                commands,
                x,
                y,
                "#9ead86",
                "#879372",
                (BoardBrickNode, node.clone()),
                || true,
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

pub fn spawn_next_brick(commands: &mut Commands, brick: Brick) {
    commands
        .spawn((NextBrick, Sprite { ..default() }))
        .with_children(|child_builder| {
            brick.nodes.iter().for_each(|node| {
                let x = 60.0 + (node.0 as f32 + 0.5) * BRICK_NODE_WIDTH;
                let y = 43.0 - (node.1 as f32 + 0.5) * BRICK_NODE_WIDTH;
                spawn_brick_node(child_builder, x, y, "#9ead86", "#000000", (), || false);
            });
        });
}

pub fn spawn_falling_brick(
    commands: &mut Commands,
    mut brick: Brick,
    falling_brick_node: BrickNode,
) {
    brick.nodes.iter_mut().for_each(|node| {
        node.0 = falling_brick_node.0 + node.0;
        node.1 = falling_brick_node.1 - node.1;
    });
    println!("spawn falling brick");
    commands
        .spawn((
            Sprite {
                color: Color::NONE,
                ..default()
            },
            Transform::from_xyz(0., 0., 400.),
            FallingBrick,
        ))
        .with_children(|child_builder| {
            brick.nodes.iter().for_each(|node| {
                let pos = get_brick_node_position(node);
                spawn_brick_node(
                    child_builder,
                    pos.x,
                    pos.y,
                    "#9ead86",
                    "#000000",
                    (FallingBrickNode, node.clone(), Visibility::Hidden),
                    || true,
                );
            });
        });
}

pub fn board_setup(mut commands: Commands, game_assets: Res<GameAssets>) {
    commands
        .spawn((
            ShapeBundle {
                path: GeometryBuilder::build_as(&shapes::Rectangle {
                    extents: Vec2 { x: 248., y: 302. },
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
                        extents: Vec2 { x: 246., y: 298. },
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
        });

    commands
        .spawn((
            Sprite {
                color: Color::NONE,
                ..default()
            },
            Transform::from_xyz(0.0, 0.0, 40.0),
        ))
        .with_children(spawn_board);

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

pub fn get_speed(level: u32) -> f32 {
    TIMER_FALLING_SECS * (0.85_f32).powi(level as i32) + level as f32 / 1000.0
}

pub fn get_score(level: u32, erase_lines: u32) -> u32 {
    assert!(0 < erase_lines);
    assert!(erase_lines <= 4);
    vec![40, 100, 300, 1200][(erase_lines - 1) as usize] * (level + 1)
}

pub fn get_level(clean_lines: u32) -> u32 {
    (clean_lines / 10).min(99)
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

fn spawn_new_falling_brick(
    commands: &mut Commands,
    game_data: &mut ResMut<GameData>,
    next_brick_entity: Entity,
    falling_brick_entity: Entity,
) {
    commands
        .entity(falling_brick_entity)
        .try_despawn_recursive();
    commands.entity(next_brick_entity).try_despawn_recursive();
    game_data.falling_brick_node = game_data.new_falling_brick_node();
    game_data.falling_brick_shape = game_data.next_brick_shape;
    game_data.next_brick_shape = BrickShape::next();
    spawn_falling_brick(
        commands,
        game_data.falling_brick_shape.into(),
        game_data.falling_brick_node,
    );
    spawn_next_brick(commands, game_data.next_brick_shape.into());
}

pub fn falling_brick_system(
    mut commands: Commands,
    time: Res<Time>,
    mut game_data: ResMut<GameData>,
    mut query: Query<
        (Entity, &mut Transform, &mut BrickNode, &mut Visibility),
        With<FallingBrickNode>,
    >,
    next_brick_entity: Single<Entity, With<NextBrick>>,
    falling_brick_entity: Single<Entity, With<FallingBrick>>,
    mut board_brick_nodes_query: Query<
        (&mut Children, &BrickNode),
        (With<BoardBrickNode>, Without<FallingBrickNode>),
    >,
    mut fill_query: Query<&mut Fill>,
    mut stroke_query: Query<&mut Stroke>,
) {
    let falling_brick_shape = game_data.falling_brick_shape;
    let mut falling_brick: Brick = falling_brick_shape.into();
    falling_brick.nodes.iter_mut().for_each(|node| {
        node.0 = game_data.falling_brick_node.0 + node.0;
        node.1 = game_data.falling_brick_node.1 - node.1;
    });
    let ticked = game_data.falling_timer.tick(time.delta()).finished();
    if ticked {
        if game_data.freeze {
            return;
        }
        let is_hit_bottom = query
            .iter()
            .any(|(_, __, node, ..)| game_data.board.is_move_to_bottom(node));
        if is_hit_bottom {
            game_data.freeze = true;
            game_data.is_speed_up_falling = false;
            game_data
                .falling_timer
                .set_duration(Duration::from_secs_f32(TIMER_FALLING_SECS));

            let is_hit_top = query
                .iter()
                .any(|(_, __, node, ..)| game_data.board.is_move_to_top(node));

            if is_hit_top {
                game_data.is_game_over = true;
                return;
            }

            game_data.board.update_occupied_by_brick(&falling_brick);

            let clean_lines = game_data.board.get_clean_lines();

            if clean_lines.1 > 0 {
                let start = clean_lines.0 as i8;
                let lines = clean_lines.1 as i8;
                for (children, node) in &mut board_brick_nodes_query {
                    if node.1 < start {
                        continue;
                    }
                    let is_occupied = if node.1 + lines >= BOARD_BRICK_NODE_ROWS as i8 {
                        false
                    } else {
                        game_data
                            .board
                            .is_brick_node_occupied(&BrickNode(node.0, node.1 + lines))
                    };
                    for child in children.iter() {
                        if let Ok(mut fill) = fill_query.get_mut(*child) {
                            fill.color = if is_occupied {
                                Srgba::hex("#000000").unwrap().into()
                            } else {
                                Srgba::hex("#879372").unwrap().into()
                            }
                        }
                        if let Ok(mut stroke) = stroke_query.get_mut(*child) {
                            stroke.color = if is_occupied {
                                Srgba::hex("#000000").unwrap().into()
                            } else {
                                Srgba::hex("#879372").unwrap().into()
                            }
                        }
                    }
                }
                game_data.board.clean(clean_lines);
                game_data.cleans += clean_lines.1 as u32;
            } else {
                // update board
                for (children, node) in &mut board_brick_nodes_query {
                    if game_data.board.is_brick_node_occupied(node) {
                        for child in children.iter() {
                            if let Ok(mut fill) = fill_query.get_mut(*child) {
                                fill.color = Color::BLACK;
                            }
                            if let Ok(mut stroke) = stroke_query.get_mut(*child) {
                                stroke.color = Color::BLACK;
                            }
                        }
                    }
                }
            }

            spawn_new_falling_brick(
                &mut commands,
                &mut game_data,
                falling_brick_entity.into_inner(),
                next_brick_entity.into_inner(),
            );
        } else {
            game_data.falling_brick_node.move_down();
            for (_, mut transform, mut brick_node, mut visibility) in query.iter_mut() {
                transform.translation.y -= BRICK_NODE_WIDTH;

                brick_node.move_down();

                *visibility = if game_data.board.is_brick_node_in_board(&brick_node) {
                    Visibility::Visible
                } else {
                    Visibility::Hidden
                };
            }
        }
    }
}

pub fn score_board_system(
    mut game_data: ResMut<GameData>,
    mut next_state: ResMut<NextState<GameSate>>,
    mut query: ParamSet<(
        Single<&mut Text2d, With<LevelText>>,
        Single<&mut Text2d, With<ScoreText>>,
        Single<&mut Text2d, With<CleansText>>,
    )>,
) {
    if !game_data.freeze {
        return;
    }
    game_data.freeze = false;

    let level = get_level(game_data.cleans);

    if game_data.level != level {
        let mut level_text = query.p0().into_inner();
        level_text.clear();
        level_text.push_str(format!("{}", level).as_str());
        game_data
            .falling_timer
            .set_duration(Duration::from_secs_f32(get_speed(level)));
    }
    if game_data.cleans > 0 {
        let score = get_score(level, game_data.cleans);
        let mut score_text = query.p1().into_inner();
        score_text.clear();
        score_text.push_str(format!("{}", score).as_str());

        let mut cleans_text = query.p2().into_inner();
        cleans_text.clear();
        cleans_text.push_str(format!("{}", game_data.cleans).as_str());
    }

    if game_data.is_game_over {
        //
        next_state.set(GameSate::GameOver);
    }
}

pub fn game_over_system(
    mut commands: Commands,
    mut game_data: ResMut<GameData>,
    next_brick_entity: Single<Entity, With<NextBrick>>,
    falling_brick_entity: Single<Entity, With<FallingBrick>>,
    mut board_brick_nodes_query: Query<
        &mut Children,
        (With<BoardBrickNode>, Without<FallingBrickNode>),
    >,
    mut fill_query: Query<&mut Fill>,
    mut stroke_query: Query<&mut Stroke>,
    mut next_state: ResMut<NextState<GameSate>>,
) {
    println!("Game Over");

    commands
        .entity(falling_brick_entity.into_inner())
        .try_despawn_recursive();
    commands
        .entity(next_brick_entity.into_inner())
        .try_despawn_recursive();

    // reset board
    for children in &mut board_brick_nodes_query {
        for child in children.iter() {
            if let Ok(mut fill) = fill_query.get_mut(*child) {
                fill.color = Srgba::hex("#879372").unwrap().into();
            }
            if let Ok(mut stroke) = stroke_query.get_mut(*child) {
                stroke.color = Srgba::hex("#879372").unwrap().into();
            }
        }
    }

    game_data.reset();
    next_state.set(GameSate::Ready);
}
