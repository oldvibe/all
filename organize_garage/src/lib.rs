#[derive(Debug, PartialEq)]
pub struct Garage<T> {
   pub left: Option<T>,
   pub right: Option<T>,
}
use std::ops::Add;
impl<T: Add<Output = T> + Copy> Garage<T> {
    pub fn move_to_right(&mut self) {
        if let Some(left) = self.left.take() {
            if let Some(right) = self.right {
                self.right = Some(right + left);
            } else {
                self.right = Some(left);
            }
        }
    }
    pub fn move_to_left(&mut self) {
        if let Some(right) = self.right.take() {
            if let Some(left) = self.left {
                self.left = Some(right + left);
            } else {
                self.left = Some(right);
            }
        }
    }
}
