use bevy::color::Color;
use bevy::ecs::component::Component;
use bevy::ecs::system::Commands;
use bevy::math::Vec2;
use bevy::transform::components::Transform;
use bevy::utils::default;
use bevy::window::Window;
use bevy::{color::Srgba, ecs::system::Single};
use bevy_prototype_lyon::{
    prelude::*,
    shapes::{self, BorderRadii, RectangleOrigin},
};

use crate::constants::DESIGN_SIZE;

#[derive(Component)]
pub struct Decorate;

fn calc_decorate_size(design_size: &Vec2, window_size: &Vec2) -> Vec2 {
    let rx: f32 = window_size.x / design_size.x;
    let ry = window_size.y / design_size.y;
    let size = if rx > ry {
        Vec2 {
            x: ry * design_size.x,
            y: window_size.y,
        }
    } else {
        Vec2 {
            x: window_size.x,
            y: rx * design_size.y,
        }
    };
    size
}

pub fn decorate_setup(mut commands: Commands, window: Single<&Window>) {
    let final_size = calc_decorate_size(&DESIGN_SIZE, &window.size());
    let rect = shapes::Rectangle {
        extents: final_size,
        radii: Some(BorderRadii::single(10.0)),
        ..shapes::Rectangle::default()
    };
    commands.spawn((
        ShapeBundle {
            path: GeometryBuilder::build_as(&rect),
            ..default()
        },
        Fill::color(Srgba::hex("#efcc19").unwrap()),
        Decorate,
    ));

    let board_border_rect = shapes::Rectangle {
        extents: Vec2::new(270.0, 310.0),
        ..shapes::Rectangle::default()
    };

    commands.spawn((
        ShapeBundle {
            path: GeometryBuilder::build_as(&board_border_rect),
            transform: Transform::from_xyz(0.0, 90.0, 0.0),
            ..default()
        },
        Stroke::new(Color::BLACK, 4.0),
    ));
}