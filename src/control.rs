use bevy::{
    color::Color,
    ecs::{
        component::Component,
        system::{Commands, Query, Res, Single},
    },
    hierarchy::BuildChildren,
    input::{mouse::MouseButton, ButtonInput},
    math::{bounding, Rect, Vec2, Vec3},
    sprite::Sprite,
    text::{FontSmoothing, JustifyText, Text2d, TextColor, TextFont, TextLayout},
    transform::components::Transform, window::Window,
};

use crate::GameAssets;

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
        let mouse_pos = window.cursor_position().unwrap();
        let mouse_world_pos = Vec2::new(
            mouse_pos.x - window.width() / 2.0,
            window.height() / 2.0 - mouse_pos.y,
        );
        for (control_button, mut sprite, transform) in query.iter_mut() {
            let button_size = sprite.custom_size.unwrap();
            let is_hit = is_hit_button(
                Vec2 {x: transform.translation.x, y:  transform.translation.y },
                mouse_world_pos,
                button_size.x,
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
        let mouse_pos = window.cursor_position().unwrap();
        let mouse_world_pos = Vec2::new(
            mouse_pos.x - window.width() / 2.0,
            window.height() / 2.0 - mouse_pos.y,
        );
        for (control_button, mut sprite, transform) in query.iter_mut() {
            let button_size = sprite.custom_size.unwrap();
            let is_hit = is_hit_button(
                Vec2 {x: transform.translation.x, y:  transform.translation.y },
            mouse_world_pos,
                button_size.x,
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
