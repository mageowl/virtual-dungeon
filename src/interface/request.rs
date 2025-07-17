use std::str::Split;

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
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

#[derive(Debug)]
pub enum Request {
    Move(Direction),
    Attack(Direction),
    Scan(i8, i8),
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
