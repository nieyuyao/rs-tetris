use bevy::math::Vec2;
use lazy_static::lazy_static;

use crate::brick::{Brick, BrickNode};

pub const DESIGN_SIZE: Vec2 = Vec2 { x: 360.0, y: 540.0 };

pub const BOARD_BRICK_NODE_ROWS: usize = 20;

pub const BOARD_BRICK_NODE_COLS: usize = 10;

pub const BOARD_BRICK_NODE_TOTAL: usize = 200;

pub const BRICK_NODE_WIDTH: f32 = 14.;

pub const BRICKS_CONTAINER_WIDTH: f32 = 144.0;

pub const BRICKS_CONTAINER_HEIGHT: f32 = 284.0;

pub const TIMER_FALLING_SECS: f32 = 0.725;

pub const TIMER_FALLING_SPEED_UP_SECS: f32 = 1. / 60.;

pub const BRICKS_CONTAINER_BOUNDING_LEFT: f32 = -109.;

pub const BRICKS_CONTAINER_BOUNDING_TOP: f32 = 229.;

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
