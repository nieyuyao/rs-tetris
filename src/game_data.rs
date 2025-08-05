use std::time::Duration;
use bevy::{ecs::system::Resource, time::{Timer, TimerMode}};
use crate::{brick::BrickShape, constants::{BOARD_BRICK_NODE_COLS, BOARD_BRICK_NODE_ROWS}};

#[derive(Debug)]
struct Board {
    pub occupied: Vec<bool>
}

impl Default for Board {
    fn default() -> Self {
        let mut occupied = vec![];
        (0..BOARD_BRICK_NODE_ROWS).for_each(|r| {
            (0..BOARD_BRICK_NODE_COLS).for_each(|c| {
                occupied.push(false);
            });
        });
        Self { occupied }
    }
}




#[derive(Default, Debug, Resource)]
pub struct GameData {
    board: Board,
    level: u8,
    score: u32,
    next_brick: BrickShape,
    moving_brick: BrickShape,
    duration: Duration,
    cleaned_lines: u32,
    falling_timer: Timer,
    pub clock_timer: Timer
}

impl GameData {
    pub fn default() -> Self {
        GameData {
            board: Board::default(),
            level: 1,
            score: 0,
            next_brick: BrickShape::default(),
            moving_brick: BrickShape::default(),
            duration: Duration::default(),
            cleaned_lines: 0,
            falling_timer: Timer::from_seconds(0.1, TimerMode::Repeating),
            clock_timer: Timer::from_seconds(1., TimerMode::Repeating)
        }
    }
}