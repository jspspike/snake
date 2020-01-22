use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    Center,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Coord {
    x: u8,
    y: u8,
}

#[derive(Clone)]
pub struct Snake {
    snake: Vec<Coord>,
    dir: Direction,
    food: Coord,
    size: u8,
    rng: StdRng,
}

impl Snake {
    pub fn new(seed: u64, size: u8) -> Snake {
        let mut rng: StdRng = SeedableRng::seed_from_u64(seed);
        let snake = vec![
            Coord {
                x: size / 2,
                y: size / 2 - 1,
            },
            Coord {
                x: size / 2 - 1,
                y: size / 2 - 1,
            },
        ];

        loop {
            let food = Coord {
                x: rng.gen_range(0, size),
                y: rng.gen_range(0, size),
            };

            if snake.iter().all(|p| food != *p) {
                break;
            }
        }

        Snake {
            snake,
            dir: Direction::Right,
            food: Coord {
                x: rng.gen_range(0, size),
                y: rng.gen_range(0, size),
            },
            size,
            rng,
        }
    }

    pub fn length(&self) -> usize {
        return self.snake.len();
    }

    fn alive(&self) -> bool {
        let mut snake = self.snake.iter();
        let head = snake.next().unwrap();

        if head.x >= self.size || head.y >= self.size {
            return false;
        }

        snake.all(|p| head != p)
    }

    fn found_food(&self) -> bool {
        let head = self.snake.first().unwrap();
        return *head == self.food;
    }

    fn gen_food(&mut self) {
        loop {
            let food = Coord {
                x: self.rng.gen_range(0, self.size),
                y: self.rng.gen_range(0, self.size),
            };
            let snake = &self.snake;

            if snake.iter().all(|p| food != *p) {
                self.food = food;
                break;
            }
        }
    }

    pub fn turn(&mut self, dir: Direction) -> bool {
        if !self.alive() {
            return false;
        }

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
        } else {
            self.gen_food();
        }

        self.alive()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_alive() {
        let mut test = Snake::new(0, 10);
        assert!(test.alive());
        test.snake = vec![Coord { x: 4, y: 10 }];
        assert!(!test.alive());
        test.snake = vec![
            Coord { x: 5, y: 4 },
            Coord { x: 4, y: 4 },
            Coord { x: 4, y: 3 },
            Coord { x: 5, y: 3 },
            Coord { x: 5, y: 4 },
        ];
        assert!(!test.alive());
    }

    #[test]
    fn test_found_food() {
        let mut test = Snake::new(0, 10);
        assert!(!test.found_food());
        test.snake = vec![Coord { x: 4, y: 9 }];
        assert!(test.found_food());
    }

    #[test]
    fn test_snake() {
        let mut test = Snake::new(0, 10);
        assert!(test.turn(Direction::Down));
        for _ in 0..4 {
            assert!(test.turn(Direction::Center));
        }
        assert!(test.turn(Direction::Left));
        assert_eq!(test.length(), 3);

        for _ in 0..4 {
            assert!(test.turn(Direction::Center));
        }
        assert!(test.turn(Direction::Up));
        assert!(test.turn(Direction::Center));
        assert_eq!(test.length(), 4);
    }
}
