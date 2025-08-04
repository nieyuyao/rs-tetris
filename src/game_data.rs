use std::time::Duration;
use bevy::ecs::system::Resource;

#[derive(Default, Debug)]
struct Board {
    occupied: Vec<bool>
}


#[derive(Default, Debug, Resource)]
pub struct GameData {
    board: Board,
    level: u8,
    score: u32,
    next_brick: u8,
    moving_brick: u8,
    duration: Duration,
    cleaned_lines: u32
}


impl GameData {
    pub fn default() -> Self {
        GameData {
            board: Board::default(),
            level: 1,
            score: 0,
            next_brick: 0,
            moving_brick: 0,
            duration: Duration::default(),
            cleaned_lines: 0,
          
        }
    }
}