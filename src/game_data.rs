use crate::{
    brick::{Brick, BrickNode, BrickShape},
    constants::{
        BOARD_BRICK_NODE_COLS, BOARD_BRICK_NODE_ROWS
    },
};
use bevy::{
    ecs::system::Resource,
    time::{Timer, TimerMode},
};
use std::time::Duration;

#[derive(Debug)]
pub struct Board {
    pub occupied: Vec<bool>,
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

impl Board {
    pub fn is_brick_node_occupied(&self, brick_node: &BrickNode) -> bool {
        let index= (brick_node.0 as usize) + (brick_node.1 as usize) * BOARD_BRICK_NODE_COLS;
        if  index >= self.occupied.len() {
            return false
        }
        assert!(index >= 0);
        self.occupied[index]
    }

    pub fn is_brick_node_in_board(&self, node: &BrickNode) -> bool {
        node.0 >= 0
            && (node.0 as usize) < BOARD_BRICK_NODE_COLS
            && node.1 >= 0
            && (node.1 as usize) < BOARD_BRICK_NODE_ROWS
    }

    pub fn is_move_to_left(&self, node: &BrickNode) -> bool {
        node.0 <= 0 || self.is_brick_hit_to_occupied(&BrickNode(node.0 - 1, node.1))
    }

    pub fn is_move_to_right(&self, node: &BrickNode) -> bool {
        node.0 as usize >= BOARD_BRICK_NODE_COLS - 1 || self.is_brick_hit_to_occupied(&BrickNode(node.0 + 1, node.1))
    }

    pub fn is_move_to_bottom(&self, node: &BrickNode) -> bool {
        node.1 <= 0 || self.is_brick_hit_to_occupied(&BrickNode(node.0, node.1 - 1))
    }

    pub fn is_move_to_top(&self, node: &BrickNode) -> bool {
        (node.1 as usize) >= BOARD_BRICK_NODE_ROWS - 1
    }

    pub fn is_brick_hit_to_occupied(&self, node: &BrickNode) -> bool {
        self.is_brick_node_occupied(node)
    }

    pub fn update_occupied_by_brick(&mut self, brick: &Brick) {
        brick.nodes.iter().for_each(|node| {
            let index= (node.0 as usize) + (node.1 as usize) * BOARD_BRICK_NODE_COLS;
            assert!(index >= 0);
            assert!(index < BOARD_BRICK_NODE_ROWS * BOARD_BRICK_NODE_COLS);
            self.occupied[index] = true;
        });
    }
}

#[derive(Default, Debug, Resource)]
pub struct GameData {
    pub board: Board,
    pub level: u8,
    pub score: u32,
    pub next_brick_shape: BrickShape,
    pub falling_brick_shape: BrickShape,
    pub cleans: u32,
    pub falling_timer: Timer,
    pub clock_timer: Timer,
    pub playing_ready_animation_duration: Duration,
    pub is_playing_dino_running_animation: bool,
    pub falling_brick_node: BrickNode,
    pub freeze: bool,
    pub is_game_over: bool,
}

impl GameData {
    pub fn default() -> Self {
        GameData {
            board: Board::default(),
            level: 1,
            score: 0,
            next_brick_shape: BrickShape::default(),
            falling_brick_shape: BrickShape::default(),
            cleans: 0,
            falling_timer: Timer::from_seconds(0.1, TimerMode::Repeating),
            clock_timer: Timer::from_seconds(60., TimerMode::Repeating),
            playing_ready_animation_duration: Duration::default(),
            is_playing_dino_running_animation: true,
            falling_brick_node: BrickNode(5, 23),
            freeze: false,
            is_game_over: false,
        }
    }
}


#[cfg(test)]
mod tests {

    use super::Board;

    use crate::brick::{Brick, BrickNode};
    
    #[test]
    fn test_board() {
        let mut board: Board = Board::default();

        board.update_occupied_by_brick(&Brick { nodes: [
            BrickNode(0, 0),
            BrickNode(1, 0),
            BrickNode(2, 0),
            BrickNode(3, 0),
        ] });

        assert!(!&board.occupied[0..4].iter().any(|b| {!b}));
    }

}