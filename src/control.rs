use std::time::Duration;

use bevy::{
    color::Color,
    ecs::{
        component::Component,
        query::{With, Without},
        system::{Commands, Query, Res, ResMut, Single},
    },
    hierarchy::BuildChildren,
    input::{ButtonInput, mouse::MouseButton},
    math::{Vec2, Vec3},
    render::view::Visibility,
    sprite::Sprite,
    state::state::NextState,
    text::{FontSmoothing, JustifyText, Text2d, TextColor, TextFont, TextLayout},
    transform::components::Transform,
    window::Window,
};

use crate::{
    board::FallingBrickNode, brick::{get_brick_node_position, Brick, BrickNode}, constants::{BOARD_BRICK_NODE_COLS, BRICK_NODE_WIDTH, TIMER_FALLING_SECS, TIMER_FALLING_SPEED_UP_SECS}, game_data::GameData, state::GameSate, GameAssets
};

#[derive(PartialEq, Eq)]
pub enum ButtonName {
    RotateButton,
    RightButton,
    DownButton,
    LeftButton,
    DropButton,
    PauseButton,
    SoundButton,
    ReplayButton,
}

#[derive(Component)]
pub struct ControlButton(ButtonName);

fn is_hit_button(button_center: Vec2, point: Vec2, r: f32) -> bool {
    button_center.distance(point) <= r
}

fn get_world_mouse_pos(mouse_pos: Vec2, win_width: f32, win_height: f32) -> Vec2 {
    Vec2::new(
        mouse_pos.x - win_width / 2.0,
        win_height / 2.0 - mouse_pos.y,
    )
}

pub fn control_setup(mut commands: Commands, game_assets: Res<GameAssets>) {
    let text_justification = JustifyText::Center;
    // rotate
    commands
        .spawn((
            Sprite {
                image: game_assets.move_button.clone(),
                custom_size: Some(Vec2::new(50.0, 50.)),
                ..Sprite::default()
            },
            Transform {
                translation: Vec3::new(70.0, -113.0, 1.),
                ..Transform::default()
            },
            ControlButton(ButtonName::RotateButton),
        ))
        .with_child((
            Text2d::new("Rotate"),
            TextColor(Color::BLACK),
            TextLayout::new_with_justify(text_justification),
            TextFont {
                font_size: 12.0,
                font_smoothing: FontSmoothing::AntiAliased,
                ..TextFont::default()
            },
            Transform::from_xyz(46., 20., 10.),
        ));
    // move to right
    commands
        .spawn((
            Sprite {
                image: game_assets.move_button.clone(),
                custom_size: Some(Vec2::new(50.0, 50.)),
                ..Sprite::default()
            },
            Transform {
                translation: Vec3::new(120.0, -160.0, 1.),
                ..Transform::default()
            },
            ControlButton(ButtonName::RightButton),
        ))
        .with_child((
            Text2d::new("Right"),
            TextColor(Color::BLACK),
            TextLayout::new_with_justify(text_justification),
            TextFont {
                font_size: 12.0,
                font_smoothing: FontSmoothing::AntiAliased,
                ..TextFont::default()
            },
            Transform::from_xyz(0., -32., 10.),
        ));
    // move to down
    commands
        .spawn((
            Sprite {
                image: game_assets.move_button.clone(),
                custom_size: Some(Vec2::new(50.0, 50.)),
                ..Sprite::default()
            },
            Transform {
                translation: Vec3::new(70.0, -212.0, 1.),
                ..Transform::default()
            },
            ControlButton(ButtonName::DownButton),
        ))
        .with_child((
            Text2d::new("Down"),
            TextColor(Color::BLACK),
            TextLayout::new_with_justify(text_justification),
            TextFont {
                font_size: 12.0,
                font_smoothing: FontSmoothing::AntiAliased,
                ..TextFont::default()
            },
            Transform::from_xyz(0., -32., 10.),
        ));
    // move to left
    commands
        .spawn((
            Sprite {
                image: game_assets.move_button.clone(),
                custom_size: Some(Vec2::new(50.0, 50.)),
                ..Sprite::default()
            },
            Transform {
                translation: Vec3::new(24.0, -160.0, 1.),
                ..Transform::default()
            },
            ControlButton(ButtonName::LeftButton),
        ))
        .with_child((
            Text2d::new("Left"),
            TextColor(Color::BLACK),
            TextLayout::new_with_justify(text_justification),
            TextFont {
                font_size: 12.0,
                font_smoothing: FontSmoothing::AntiAliased,
                ..TextFont::default()
            },
            Transform::from_xyz(0., -32., 10.),
        ));
    // drop
    commands
        .spawn((
            Sprite {
                image: game_assets.move_button.clone(),
                custom_size: Some(Vec2::new(50.0, 50.0)),
                ..Sprite::default()
            },
            Transform {
                translation: Vec3::new(-86.0, -184.0, 1.),
                ..Transform::default()
            },
            ControlButton(ButtonName::DropButton),
        ))
        .with_child((
            Text2d::new("Drop"),
            TextColor(Color::BLACK),
            TextLayout::new_with_justify(text_justification),
            TextFont {
                font_size: 12.0,
                font_smoothing: FontSmoothing::AntiAliased,
                ..TextFont::default()
            },
            Transform::from_xyz(0., -32., 10.),
        ));
    // pause
    commands
        .spawn((
            Sprite {
                image: game_assets.effect_button.clone(),
                custom_size: Some(Vec2::new(30.0, 30.0)),
                ..Sprite::default()
            },
            Transform {
                translation: Vec3 {
                    x: -120.0,
                    y: -100.0,
                    z: 1.,
                },
                ..Transform::default()
            },
            ControlButton(ButtonName::PauseButton),
        ))
        .with_child((
            Text2d::new("Pause"),
            TextColor(Color::BLACK),
            TextLayout::new_with_justify(text_justification),
            TextFont {
                font_size: 10.0,
                font_smoothing: FontSmoothing::AntiAliased,
                ..TextFont::default()
            },
            Transform::from_xyz(0., -24., 10.),
        ));
    // sound
    commands
        .spawn((
            Sprite {
                image: game_assets.effect_button.clone(),
                custom_size: Some(Vec2::new(30.0, 30.0)),
                ..Sprite::default()
            },
            Transform {
                translation: Vec3 {
                    x: -70.0,
                    y: -100.0,
                    z: 1.,
                },
                ..Transform::default()
            },
            ControlButton(ButtonName::SoundButton),
        ))
        .with_child((
            Text2d::new("Sound"),
            TextColor(Color::BLACK),
            TextLayout::new_with_justify(text_justification),
            TextFont {
                font_size: 10.0,
                font_smoothing: FontSmoothing::AntiAliased,
                ..TextFont::default()
            },
            Transform::from_xyz(0., -24., 10.),
        ));
    // replay
    commands
        .spawn((
            Sprite {
                image: game_assets.replay_button.clone(),
                custom_size: Some(Vec2::new(30.0, 30.0)),
                ..Sprite::default()
            },
            Transform {
                translation: Vec3 {
                    x: -20.0,
                    y: -100.0,
                    z: 1.,
                },
                ..Transform::default()
            },
            ControlButton(ButtonName::ReplayButton),
        ))
        .with_child((
            Text2d::new("Replay"),
            TextColor(Color::BLACK),
            TextLayout::new_with_justify(text_justification),
            TextFont {
                font_size: 10.0,
                font_smoothing: FontSmoothing::AntiAliased,
                ..TextFont::default()
            },
            Transform::from_xyz(0., -24., 10.),
        ));
}

pub fn control_on_click(
    mut query: Query<(&ControlButton, &mut Sprite, &Transform)>,
    window: Single<&Window>,
    game_assets: Res<GameAssets>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        let mouse_world_pos = get_world_mouse_pos(
            window.cursor_position().unwrap(),
            window.width(),
            window.height(),
        );
        for (control_button, mut sprite, transform) in query.iter_mut() {
            let button_size = sprite.custom_size.unwrap();
            let is_hit = is_hit_button(
                Vec2 {
                    x: transform.translation.x,
                    y: transform.translation.y,
                },
                mouse_world_pos,
                button_size.x / 2.0,
            );
            if !is_hit {
                continue;
            }
            match control_button.0 {
                ButtonName::DropButton
                | ButtonName::RotateButton
                | ButtonName::RightButton
                | ButtonName::DownButton
                | ButtonName::LeftButton => {
                    sprite.image = game_assets.move_button_pressed.clone();
                }
                ButtonName::PauseButton | ButtonName::SoundButton => {
                    sprite.image = game_assets.effect_button_pressed.clone();
                }
                _ => {
                    sprite.image = game_assets.replay_button_pressed.clone();
                }
            }
        }
    } else if mouse_button_input.just_released(MouseButton::Left) {
        let mouse_world_pos = get_world_mouse_pos(
            window.cursor_position().unwrap(),
            window.width(),
            window.height(),
        );
        for (control_button, mut sprite, transform) in query.iter_mut() {
            let button_size = sprite.custom_size.unwrap();
            let is_hit = is_hit_button(
                Vec2 {
                    x: transform.translation.x,
                    y: transform.translation.y,
                },
                mouse_world_pos,
                button_size.x / 2.0,
            );
            if !is_hit {
                continue;
            }
            match control_button.0 {
                ButtonName::DropButton
                | ButtonName::RotateButton
                | ButtonName::RightButton
                | ButtonName::DownButton
                | ButtonName::LeftButton => {
                    sprite.image = game_assets.move_button.clone();
                }
                ButtonName::PauseButton | ButtonName::SoundButton => {
                    sprite.image = game_assets.effect_button.clone();
                }
                _ => {
                    sprite.image = game_assets.replay_button.clone();
                }
            }
        }
    }
}

pub fn control_drop_to_start_game(
    mut next_state: ResMut<NextState<GameSate>>,
    window: Single<&Window>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    query: Query<(&Transform, &ControlButton, &Sprite)>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        for (transform, control_button, sprite) in query.iter() {
            if control_button.0 == ButtonName::DropButton {
                let mouse_pos = window.cursor_position().unwrap();
                let mouse_world_pos = Vec2::new(
                    mouse_pos.x - window.width() / 2.0,
                    window.height() / 2.0 - mouse_pos.y,
                );
                if is_hit_button(
                    Vec2 {
                        x: transform.translation.x,
                        y: transform.translation.y,
                    },
                    mouse_world_pos,
                    sprite.custom_size.unwrap().x / 2.0,
                ) {
                    next_state.set(GameSate::Playing);
                }
            }
        }
    }
}

fn falling_brick_nodes_any<F>(brick: &Brick, condition: F) -> bool
where
    F: FnMut(&BrickNode) -> bool,
{
    brick.nodes.iter().any(condition)
}

pub fn control_game_system(
    mut query: Query<(&ControlButton, &mut Sprite, &Transform), Without<FallingBrickNode>>,
    window: Single<&Window>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut game_data: ResMut<GameData>,
    mut falling_brick_query: Query<
        (&mut Transform, &mut BrickNode, &mut Visibility),
        With<FallingBrickNode>,
    >,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        let mouse_world_pos = get_world_mouse_pos(
            window.cursor_position().unwrap(),
            window.width(),
            window.height(),
        );
        for (control_button, sprite, transform) in query.iter_mut() {
            let button_size = sprite.custom_size.unwrap();
            let is_hit = is_hit_button(
                Vec2 {
                    x: transform.translation.x,
                    y: transform.translation.y,
                },
                mouse_world_pos,
                button_size.x / 2.0,
            );
            if !is_hit {
                continue;
            }
            let mut falling_brick: Brick = game_data.falling_brick_shape.into();
            falling_brick.nodes.iter_mut().for_each(|node| {
                node.0 = game_data.falling_brick_node.0 + node.0;
                node.1 = game_data.falling_brick_node.1 - node.1;
            });
            match control_button.0 {
                ButtonName::DownButton => {
                    let disabled = falling_brick_nodes_any(&falling_brick, |node| {
                        game_data.board.is_move_to_bottom(node)
                    });
                    if !disabled {
                        game_data.falling_brick_node.move_down();
                        for (mut transform, mut brick_node, mut visibility) in
                            falling_brick_query.iter_mut()
                        {
                            brick_node.move_down();
                            transform.translation.y -= BRICK_NODE_WIDTH;
                            *visibility = if game_data.board.is_brick_node_in_board(&brick_node) {
                                Visibility::Visible
                            } else {
                                Visibility::Hidden
                            };
                        }
                    }
                }
                ButtonName::RightButton => {
                    let disabled = falling_brick_nodes_any(&falling_brick, |node| {
                        game_data.board.is_move_to_right(node)
                    });
                    if !disabled {
                        game_data.falling_brick_node.move_right();
                        for (mut transform, mut brick_node, ..) in falling_brick_query.iter_mut() {
                            brick_node.move_right();
                            transform.translation.x += BRICK_NODE_WIDTH;
                        }
                    }
                }
                ButtonName::LeftButton => {
                    let disabled = falling_brick_nodes_any(&falling_brick, |node| {
                        game_data.board.is_move_to_left(node)
                    });
                    if !disabled {
                        game_data.falling_brick_node.move_left();
                        for (mut transform, mut brick_node, ..) in falling_brick_query.iter_mut() {
                            brick_node.move_left();
                            transform.translation.x -= BRICK_NODE_WIDTH;
                        }
                    }
                }
                ButtonName::RotateButton => {
                    let rotated = game_data.falling_brick_shape.rotate();
                    game_data.falling_brick_shape = rotated;
                    let mut rotated_falling_brick: Brick = game_data.falling_brick_shape.into();
                    let mut bounding = (0, 0, 0, 0);
                    rotated_falling_brick.nodes.iter_mut().for_each(|node| {
                        node.0 = game_data.falling_brick_node.0 + node.0;
                        node.1 = game_data.falling_brick_node.1 - node.1;
                        if node.0 < bounding.0 {
                            bounding.0 = node.0;
                        };
                        if node.1 < bounding.3 {
                            bounding.3 = node.1;
                        };
                        if node.0 > bounding.1 {
                            bounding.1 = node.0;
                        };
                    });

                    if bounding.0 < 0 {
                        game_data.falling_brick_node.move_left_steps(-bounding.0);
                    } else if (bounding.1 as usize) >= BOARD_BRICK_NODE_COLS {
                        game_data
                            .falling_brick_node
                            .move_right_steps(BOARD_BRICK_NODE_COLS as i8 - bounding.1);
                    }
                    if bounding.3 < 0 {
                        game_data.falling_brick_node.move_up_steps(-bounding.3);
                    }

                    let mut new_falling_brick: Brick = game_data.falling_brick_shape.into();
                    new_falling_brick.nodes.iter_mut().for_each(|node| {
                        node.0 = game_data.falling_brick_node.0 + node.0;
                        node.1 = game_data.falling_brick_node.1 - node.1;
                    });

                    for (i, (mut transform, mut brick_node, mut visibility)) in
                        falling_brick_query.iter_mut().enumerate()
                    {
                        let pos = get_brick_node_position(&new_falling_brick.nodes[i]);
                        transform.translation.x = pos.x;
                        transform.translation.y = pos.y;
                        brick_node.0 = new_falling_brick.nodes[i].0;
                        brick_node.1 = new_falling_brick.nodes[i].1;

                        *visibility = if game_data.board.is_brick_node_in_board(&brick_node) {
                            Visibility::Visible
                        } else {
                            Visibility::Hidden
                        };
                    }
                }
                ButtonName::DropButton => {
                    if game_data.is_speed_up_falling {
                        return;
                    }
                    println!("click drop button");
                    game_data.is_speed_up_falling = true;
                    game_data
                        .falling_timer
                        .set_duration(Duration::from_secs_f32(TIMER_FALLING_SPEED_UP_SECS));
                }
                _ => {}
            }
        }
    }
}
