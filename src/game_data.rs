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
    cleans: u32,
    falling_timer: Timer,
    pub clock_timer: Timer,
    pub playing_ready_animation_duration: Duration,
    pub is_playing_dino_running_animation: bool,
}

impl GameData {
    pub fn default() -> Self {
        GameData {
            board: Board::default(),
            level: 1,
            score: 0,
            next_brick: BrickShape::default(),
            moving_brick: BrickShape::default(),
            cleans: 0,
            falling_timer: Timer::from_seconds(0.1, TimerMode::Repeating),
            clock_timer: Timer::from_seconds(60., TimerMode::Repeating),
            playing_ready_animation_duration: Duration::default(),
            is_playing_dino_running_animation: true,
        }
    }
}