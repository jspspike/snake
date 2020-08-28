use std::ops::Add;

/// Choice for direction on board
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    Center,
}

/// Coordinate to show position on board
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub(super) struct Coord {
    pub(super) x: u8,
    pub(super) y: u8,
}

impl Coord {
    pub fn in_bounds(self, size: u8) -> bool {
        self.x < size && self.y < size
    }
}

impl Add<Direction> for Coord {
    type Output = Result<Self, &'static str>;

    fn add(self, dir: Direction) -> Result<Self, &'static str> {
        match dir {
            Direction::Up => {
                if self.y == 0 {
                    Err("Already at top row")
                } else {
                    Ok(Coord {
                        x: self.x,
                        y: self.y - 1,
                    })
                }
            }
            Direction::Down => Ok(Coord {
                x: self.x,
                y: self.y + 1,
            }),
            Direction::Left => {
                if self.x == 0 {
                    Err("Already at left column")
                } else {
                    Ok(Coord {
                        x: self.x - 1,
                        y: self.y,
                    })
                }
            }
            Direction::Right => Ok(Coord {
                x: self.x + 1,
                y: self.y,
            }),
            Direction::Center => Ok(self),
        }
    }
}
