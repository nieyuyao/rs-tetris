use rand::{rng, Rng};
use crate::brick_node::BrickNode;
use crate::constants::BRICKS_DICT;

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