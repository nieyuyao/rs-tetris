use bevy::math::Vec2;
use lazy_static::lazy_static;

use crate::{brick::Brick, brick_node::BrickNode};

pub const DESIGN_SIZE: Vec2 = Vec2 { x: 360.0, y: 540.0 };

pub const BOARD_BRICK_NODE_ROWS: i8 = 20;

pub const BOARD_BRICK_NODE_COLS: i8 = 10;

pub const BRICK_NODE_WIDTH: f32 = 12.;

pub const BRICK_NODE_INNER_WIDTH: f32 = 8.;

pub const BRICK_NODE_GAP: f32 = 2.;

pub const BRICKS_CONTAINER_WIDTH: f32 = 144.0;

pub const BRICKS_CONTAINER_HEIGHT: f32 = 284.0;

pub const TIMER_FALLING_SECS: f32 = 0.725;

pub const DINO_ANIMATION_SECONDS: u8 = 10;

lazy_static! {
    pub static ref BRICKS_DICT: Vec<Vec<Brick>> = vec![
        vec![Brick {
            nodes: [
                BrickNode(1, 1),
                BrickNode(1, 2),
                BrickNode(2, 1),
                BrickNode(2, 2),
            ],
        }],
        vec![
            Brick {
                nodes: [
                    BrickNode(0, 1),
                    BrickNode(1, 1),
                    BrickNode(2, 1),
                    BrickNode(3, 1),
                ],
            },
            Brick {
                nodes: [
                    BrickNode(2, 0),
                    BrickNode(2, 1),
                    BrickNode(2, 2),
                    BrickNode(2, 3),
                ],
            },
        ],
        vec![
            Brick {
                nodes: [
                    BrickNode(0, 1),
                    BrickNode(1, 1),
                    BrickNode(2, 1),
                    BrickNode(2, 0),
                ],
            },
            Brick {
                nodes: [
                    BrickNode(1, 0),
                    BrickNode(1, 1),
                    BrickNode(1, 2),
                    BrickNode(0, 0),
                ],
            },
            Brick {
                nodes: [
                    BrickNode(0, 1),
                    BrickNode(1, 1),
                    BrickNode(2, 1),
                    BrickNode(0, 2),
                ],
            },
            Brick {
                nodes: [
                    BrickNode(1, 0),
                    BrickNode(1, 1),
                    BrickNode(1, 2),
                    BrickNode(2, 2),
                ],
            },
        ],
        vec![
            Brick {
                nodes: [
                    BrickNode(0, 1),
                    BrickNode(1, 1),
                    BrickNode(2, 1),
                    BrickNode(0, 0),
                ],
            },
            Brick {
                nodes: [
                    BrickNode(1, 0),
                    BrickNode(1, 1),
                    BrickNode(1, 2),
                    BrickNode(0, 2),
                ],
            },
            Brick {
                nodes: [
                    BrickNode(0, 1),
                    BrickNode(1, 1),
                    BrickNode(2, 1),
                    BrickNode(2, 2),
                ],
            },
            Brick {
                nodes: [
                    BrickNode(1, 0),
                    BrickNode(1, 1),
                    BrickNode(1, 2),
                    BrickNode(2, 0),
                ],
            },
        ],
        vec![
            Brick {
                nodes: [
                    BrickNode(0, 0),
                    BrickNode(1, 0),
                    BrickNode(1, 1),
                    BrickNode(2, 1),
                ],
            },
            Brick {
                nodes: [
                    BrickNode(1, 2),
                    BrickNode(1, 1),
                    BrickNode(2, 1),
                    BrickNode(2, 0),
                ],
            },
        ],
        vec![
            Brick {
                nodes: [
                    BrickNode(0, 1),
                    BrickNode(1, 1),
                    BrickNode(1, 0),
                    BrickNode(2, 0),
                ],
            },
            Brick {
                nodes: [
                    BrickNode(2, 2),
                    BrickNode(2, 1),
                    BrickNode(1, 1),
                    BrickNode(1, 0),
                ],
            },
        ],
        //T:
        vec![
            Brick {
                nodes: [
                    BrickNode(0, 1),
                    BrickNode(1, 1),
                    BrickNode(2, 1),
                    BrickNode(1, 0),
                ],
            },
            Brick {
                nodes: [
                    BrickNode(1, 0),
                    BrickNode(1, 1),
                    BrickNode(1, 2),
                    BrickNode(0, 1),
                ],
            },
            Brick {
                nodes: [
                    BrickNode(0, 1),
                    BrickNode(1, 1),
                    BrickNode(2, 1),
                    BrickNode(1, 2),
                ],
            },
            Brick {
                nodes: [
                    BrickNode(1, 0),
                    BrickNode(1, 1),
                    BrickNode(1, 2),
                    BrickNode(2, 1),
                ],
            },
        ]
    ];
}
