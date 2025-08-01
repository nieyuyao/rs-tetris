use std::time::Duration;

use bevy::ecs::resource::Resource;

use crate::board::Board;


#[derive(Default, Debug, Resource)]
pub struct GameData {
    board: Board,
    level: u8,
    score: u32,
    next_brick: u8,
    moving_brick: u8,
    duration: Duration,
    cleaned_lines: u32,
}