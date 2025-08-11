use crate::constants::{BRICKS_CONTAINER_BOUNDING_LEFT, BRICKS_CONTAINER_BOUNDING_TOP, BRICKS_DICT, BRICK_NODE_WIDTH};
use bevy::{ecs::component::Component, math::Vec2};
use rand::{Rng, rng};

#[derive(Clone, Copy, Default, Debug, Component, PartialEq, Eq)]
pub struct BrickNode(pub i8, pub i8);

impl BrickNode {
    pub fn move_left(&mut self) {
        self.0 -= 1;
    }

    pub fn move_right(&mut self) {
        self.0 += 1
    }

    pub fn move_down(&mut self) {
        self.1 -= 1;
    }

    pub fn move_left_steps(&mut self, steps: i8) {
        self.0 -= steps;
    }

    pub fn move_right_steps(&mut self, steps: i8) {
        self.0 += steps;
    }

    pub fn move_down_steps(&mut self, steps: i8) {
        self.1 -= steps;
    }

    pub fn move_up_steps(&mut self, steps: i8) {
        self.1 += steps;
    }
}

#[derive(Clone, Copy, Default, Debug)]
pub struct Brick {
    pub nodes: [BrickNode; 4],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct BrickShape(usize, usize);

impl BrickShape {
    pub fn rand() -> Self {
        let index = rng().random_range(0..7);
        BrickShape(index, 0)
    }
    pub fn rotate(&self) -> Self {
        Self(self.0, (self.1 + 1) % BRICKS_DICT[self.0].len())
    }
    pub fn next() -> Self {
        Self::rand()
    }
}

impl From<BrickShape> for Brick {
    fn from(value: BrickShape) -> Self {
        BRICKS_DICT[value.0][value.1].clone()
    }
}

pub fn get_brick_node_position(node: &BrickNode) -> Vec2 {
    let x = BRICKS_CONTAINER_BOUNDING_LEFT + (node.0 as f32 + 0.5) * BRICK_NODE_WIDTH;
    let y = BRICKS_CONTAINER_BOUNDING_TOP - (20. - node.1 as f32 - 0.5) * BRICK_NODE_WIDTH;
    return Vec2 { x, y };
}

#[cfg(test)]
mod tests {

    use super::BrickNode;

    #[test]
    fn test_brick_node() {
        let node1 = BrickNode(1, 1);
        let node2 = BrickNode(1, 1);
        let node3 = BrickNode(1, 2);
        assert_eq!(node1, node2);
        assert_ne!(node1, node3);
    }
}
