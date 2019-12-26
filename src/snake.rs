use rand::Rng;

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    Center,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Coord {
    x: u8,
    y: u8,
}

pub struct Snake {
    snake: Vec<Coord>,
    dir: Direction,
    food: Coord,
    size: u8,
}

impl Snake {
    pub fn new() -> Snake {
        let mut rng = rand::thread_rng();

        Snake {
            snake: vec![Coord { x: 5, y: 4 }, Coord { x: 4, y: 4 }],
            dir: Direction::Right,
            food: Coord {
                x: rng.gen_range(0, 10),
                y: rng.gen_range(0, 10),
            },
            size: 10,
        }
    }

    fn alive(&self) -> bool {
        let mut snake = self.snake.iter();
        let head = snake.next().unwrap().clone();

        if head.x >= self.size || head.y >= self.size {
            return false;
        }

        for pos in snake {
            if head == *pos {
                return false;
            }
        }

        true
    }

    fn found_food(&self) -> bool {
        let head = self.snake.first().unwrap();
        return *head == self.food;
    }

    pub fn turn(&mut self, dir: Direction) -> bool {
        match self.dir {
            Direction::Left => {
                if dir == Direction::Up || dir == Direction::Down {
                    self.dir = dir;
                }
            }
            Direction::Right => {
                if dir == Direction::Up || dir == Direction::Down {
                    self.dir = dir;
                }
            }
            Direction::Up => {
                if dir == Direction::Left || dir == Direction::Right {
                    self.dir = dir;
                }
            }
            Direction::Down => {
                if dir == Direction::Left || dir == Direction::Right {
                    self.dir = dir;
                }
            }
            Direction::Center => panic!("Direction can't be center"),
        }

        self.snake.insert(
            0,
            match self.dir {
                Direction::Left => Coord {
                    x: self.snake.first().unwrap().x - 1,
                    y: self.snake.first().unwrap().y,
                },
                Direction::Right => Coord {
                    x: self.snake.first().unwrap().x + 1,
                    y: self.snake.first().unwrap().y,
                },
                Direction::Up => Coord {
                    x: self.snake.first().unwrap().x,
                    y: self.snake.first().unwrap().y - 1,
                },
                Direction::Down => Coord {
                    x: self.snake.first().unwrap().x,
                    y: self.snake.first().unwrap().y + 1,
                },
                Direction::Center => panic!("Direction can't be center"),
            },
        );
        if !self.found_food() {
            self.snake.pop();
        }

        self.alive()
    }
}
