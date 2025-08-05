#[derive(Clone, Copy, Default, Debug)]
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
}