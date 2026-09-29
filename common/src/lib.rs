use std::{
    fmt::Display,
    ops::{Add, AddAssign, SubAssign},
    str::Split,
};

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Direction::Up => "up",
            Direction::Down => "down",
            Direction::Left => "left",
            Direction::Right => "right",
        })
    }
}

impl TryFrom<&str> for Direction {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, ()> {
        match value {
            "up" => Ok(Self::Up),
            "down" => Ok(Self::Down),
            "left" => Ok(Self::Left),
            "right" => Ok(Self::Right),
            _ => Err(()),
        }
    }
}

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

#[derive(Debug)]
pub enum Request {
    Move(Direction),
    Attack(Direction),
    Scan(i8, i8),
}

impl Display for Request {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Request::Move(direction) => write!(f, "move {direction}"),
            Request::Attack(direction) => write!(f, "attack {direction}"),
            Request::Scan(x, y) => write!(f, "scan {x} {y}"),
        }
    }
}

impl<'a> TryFrom<&'a mut Split<'a, &'a str>> for Request {
    type Error = ();

    fn try_from(words: &mut Split<'a, &'a str>) -> Result<Self, ()> {
        match words.next().ok_or(())? {
            "move" => Ok(Self::Move(Direction::try_from(words.next().ok_or(())?)?)),
            "attack" => Ok(Self::Attack(Direction::try_from(words.next().ok_or(())?)?)),
            "scan" => {
                let x = words.next().ok_or(())?.parse().map_err(|_| ())?;
                let y = words.next().ok_or(())?.parse().map_err(|_| ())?;
                Ok(Self::Scan(x, y))
            }
            _ => Err(()),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Empty,
    Wall,
    Character,
    Coins,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Empty => f.write_str("empty"),
            Tile::Wall => f.write_str("wall"),
            Tile::Character => f.write_str("robot"),
            Tile::Coins => f.write_str("coins"),
        }
    }
}

impl<'a> TryFrom<&'a str> for Tile {
    type Error = &'a str;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        match value {
            "empty" => Ok(Self::Empty),
            "wall" => Ok(Self::Wall),
            "robot" => Ok(Self::Character),
            "coins" => Ok(Self::Coins),
            _ => Err(value),
        }
    }
}
