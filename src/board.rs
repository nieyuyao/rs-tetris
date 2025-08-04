use bevy::{
    color::{Color, Srgba}, ecs::{
        component::Component,
        system::{Commands, Res},
    }, hierarchy::BuildChildren, math::Vec2, sprite::Anchor, text::{FontSmoothing, Text2d, TextColor, TextFont}, transform::components::Transform, utils::default
};
use bevy_prototype_lyon::{
    draw::Fill,
    entity::ShapeBundle,
    prelude::GeometryBuilder,
    shapes::{self, BorderRadii, RectangleOrigin},
};

use crate::GameAssets;

#[derive(Component)]
pub struct BoardBackground;

#[derive(Component)]
pub struct ScoreLabel;

#[derive(Component)]
pub struct Score;

pub fn board_setup(mut commands: Commands, game_assets: Res<GameAssets>) {
    commands
        .spawn((
            ShapeBundle {
                path: GeometryBuilder::build_as(&shapes::Rectangle {
                    extents: Vec2 { x: 212., y: 262. },
                    origin: RectangleOrigin::Center,
                    ..shapes::Rectangle::default()
                }),
                transform: Transform::from_xyz(0.0, 100.0, 30.0),
                ..default()
            },
            Fill::color(Srgba::hex("#fae36c").unwrap()),
            BoardBackground,
        ))
        .with_child((
            ShapeBundle {
                path: GeometryBuilder::build_as(&shapes::Rectangle {
                    extents: Vec2 { x: 210., y: 260. },
                    origin: RectangleOrigin::Center,
                    radii: Some(BorderRadii::single(2.0)),
                    ..shapes::Rectangle::default()
                }),
                transform: Transform::from_xyz(0.0, 0.0, 40.0),
                ..default()
            },
            Fill::color(Srgba::hex("#9ead86").unwrap()),
            BoardBackground,
        ));
    // score label
    commands.spawn((
        Text2d::new("Score"),
        TextColor(Color::BLACK),
        TextFont {
            font_size: 14.0,
            font_smoothing: FontSmoothing::AntiAliased,
            font: game_assets.font.clone(),
            ..TextFont::default()
        },
        Transform::from_xyz(70., 200., 100.),
        Anchor::TopRight,
        ScoreLabel,
    ));
    commands.spawn((
        Text2d::new("00000000"),
        TextColor(Color::BLACK),
        TextFont {
            font_size: 18.0,
            font_smoothing: FontSmoothing::AntiAliased,
            font: game_assets.font.clone(),
            ..TextFont::default()
        },
        Transform::from_xyz(70., 184., 100.),
        Anchor::TopRight,
        Score,
    ));
}
