use std::ops::{Add, AddAssign, SubAssign};

use crate::interface::request::Direction;

pub mod character;
mod dungeon;
pub mod grid;

impl Add<Direction> for (usize, usize) {
    type Output = (usize, usize);
    fn add(mut self, rhs: Direction) -> (usize, usize) {
        self += rhs;
        self
    }
}
impl AddAssign<Direction> for (usize, usize) {
    fn add_assign(&mut self, rhs: Direction) {
        match rhs {
            Direction::Up => self.1 -= 1,
            Direction::Down => self.1 += 1,
            Direction::Left => self.0 -= 1,
            Direction::Right => self.0 += 1,
        }
    }
}
impl SubAssign<Direction> for (usize, usize) {
    fn sub_assign(&mut self, rhs: Direction) {
        match rhs {
            Direction::Up => self.1 += 1,
            Direction::Down => self.1 -= 1,
            Direction::Left => self.0 += 1,
            Direction::Right => self.0 -= 1,
        }
    }
}
