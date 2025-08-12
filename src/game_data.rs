use crate::{
    brick::{Brick, BrickNode, BrickShape},
    constants::{
        BOARD_BRICK_NODE_COLS, BOARD_BRICK_NODE_ROWS, BOARD_BRICK_NODE_TOTAL, TIMER_FALLING_SECS,
    },
};
use bevy::{
    ecs::system::Resource,
    time::{Timer, TimerMode},
};
use std::time::Duration;

#[derive(Debug, PartialEq, Eq, Default)]
pub enum EraseAnimationStep {
    #[default]
    NotStart,
    Playing,
    End,
}

#[derive(Debug, Clone)]
pub struct Board {
    pub occupied: Vec<bool>,
}

impl Default for Board {
    fn default() -> Self {
        let mut occupied = vec![];
        (0..BOARD_BRICK_NODE_ROWS).for_each(|_| {
            (0..BOARD_BRICK_NODE_COLS).for_each(|_| {
                occupied.push(false);
            });
        });
        Self { occupied }
    }
}

impl Board {
    pub fn is_brick_node_occupied(&self, brick_node: &BrickNode) -> bool {
        let index = (brick_node.0 as usize) + (brick_node.1 as usize) * BOARD_BRICK_NODE_COLS;
        if index >= self.occupied.len() {
            return false;
        }
        assert!(index >= 0);
        assert!(index < BOARD_BRICK_NODE_TOTAL);
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
        node.0 as usize >= BOARD_BRICK_NODE_COLS - 1
            || self.is_brick_hit_to_occupied(&BrickNode(node.0 + 1, node.1))
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
            let index = (node.0 as usize) + (node.1 as usize) * BOARD_BRICK_NODE_COLS;
            assert!(index >= 0);
            assert!(index < BOARD_BRICK_NODE_TOTAL);
            self.occupied[index] = true;
        });
    }

    pub fn get_clean_lines(&self) -> (usize, usize) {
        let mut start: i8 = -1;
        let mut lines: i8 = 0;

        for (i, _) in (0..BOARD_BRICK_NODE_ROWS).into_iter().rev().enumerate() {
            let is_full = (&self.occupied
                [(BOARD_BRICK_NODE_COLS * i)..(BOARD_BRICK_NODE_COLS * (i + 1))])
                .iter()
                .all(|v| *v);
            if is_full {
                if start < 0 {
                    start = i as i8;
                }
                lines += 1;
            } else if start >= 0 {
                break;
            }
        }

        if start >= 0 {
            (start as usize, lines as usize)
        } else {
            (0, 0)
        }
    }

    pub fn clean(&mut self, range: (usize, usize)) {
        (range.0..BOARD_BRICK_NODE_ROWS).for_each(|i| {
            (0..BOARD_BRICK_NODE_COLS).for_each(|j| {
                let index = i * BOARD_BRICK_NODE_COLS + j;
                let above_index = (i + 1) * BOARD_BRICK_NODE_COLS + j;
                self.occupied[index] = if above_index >= BOARD_BRICK_NODE_TOTAL {
                    false
                } else {
                    self.occupied[above_index]
                }
            });
        });
    }
}

#[derive(Default, Debug, Resource)]
pub struct GameData {
    pub board: Board,
    pub level: u32,
    pub score: u32,
    pub next_brick_shape: BrickShape,
    pub falling_brick_shape: BrickShape,
    pub cleans: u32,
    pub falling_timer: Timer,
    pub clock_timer: Timer,
    pub ready_animation_duration: Duration,
    pub is_playing_dino_running_animation: bool,
    pub falling_brick_node: BrickNode,
    pub freeze: bool,
    pub is_game_over: bool,
    pub is_speed_up_falling: bool,
    pub paused: bool,
    pub erase_animation_step: EraseAnimationStep,
    pub erase_animation_duration: Duration,
    pub erase_animation_timer: Timer,
    pub erase_animation_index: i8,
    pub clean_lines: (usize, usize)
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
            falling_timer: Timer::from_seconds(TIMER_FALLING_SECS, TimerMode::Repeating),
            clock_timer: Timer::from_seconds(60., TimerMode::Repeating),
            ready_animation_duration: Duration::default(),
            is_playing_dino_running_animation: true,
            falling_brick_node: BrickNode(5, 23),
            freeze: false,
            is_game_over: false,
            is_speed_up_falling: false,
            paused: false,
            erase_animation_step: EraseAnimationStep::NotStart,
            erase_animation_duration: Duration::default(),
            erase_animation_timer: Timer::from_seconds(0.5, TimerMode::Repeating),
            erase_animation_index: 0,
            clean_lines: (0, 0)
        }
    }

    pub fn new_falling_brick_node(&mut self) -> BrickNode {
        BrickNode(5, 23)
    }

    pub fn reset(&mut self) {
        self.board = Board::default();
        self.level = 1;
        self.score = 0;
        self.next_brick_shape = BrickShape::default();
        self.falling_brick_shape = BrickShape::default();
        self.cleans = 0;
        self.falling_brick_node = BrickNode(5, 23);
        self.freeze = false;
        self.is_game_over = false;
        self.is_playing_dino_running_animation = true;
        self.falling_timer = Timer::from_seconds(TIMER_FALLING_SECS, TimerMode::Repeating);
        self.is_speed_up_falling = false;
        self.paused = false;
        self.erase_animation_step = EraseAnimationStep::NotStart;
        self.clean_lines = (0, 0);
        self.erase_animation_index = 0
    }
}

#[cfg(test)]
mod tests {

    use super::Board;

    use crate::brick::{Brick, BrickNode};

    #[test]
    fn test_board_update_occupied() {
        let mut board: Board = Board::default();

        board.update_occupied_by_brick(&Brick {
            nodes: [
                BrickNode(0, 0),
                BrickNode(1, 0),
                BrickNode(2, 0),
                BrickNode(3, 0),
            ],
        });

        assert!(!&board.occupied[0..4].iter().any(|b| { !b }));
    }

    #[test]
    fn test_board_clean_lines() {
        let mut board: Board = Board::default();

        board.update_occupied_by_brick(&Brick {
            nodes: [
                BrickNode(0, 0),
                BrickNode(1, 0),
                BrickNode(2, 0),
                BrickNode(3, 0),
            ],
        });

        board.update_occupied_by_brick(&Brick {
            nodes: [
                BrickNode(4, 0),
                BrickNode(5, 0),
                BrickNode(6, 0),
                BrickNode(7, 0),
            ],
        });

        board.update_occupied_by_brick(&Brick {
            nodes: [
                BrickNode(8, 0),
                BrickNode(9, 0),
                BrickNode(8, 1),
                BrickNode(9, 1),
            ],
        });

        assert_eq!(board.get_clean_lines(), (0, 1));

        board.clean((0, 1));

        let has_occupied = board.occupied.iter().any(|v| *v);
        assert!(has_occupied);

        board.clean((0, 1));

        let has_occupied = board.occupied.iter().any(|v| *v);
        assert!(!has_occupied);
    }
}
