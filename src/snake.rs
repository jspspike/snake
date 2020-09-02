//! # Quick Start
//!
//! Game without display
//! ```
//! use snake::{Snake, Direction};
//! let mut game = Snake::new(0, 10);
//!
//! game.turn(Direction::Down);
//! ```
//!
//! Game with display
//! ```ignore
//! use snake::{Direction, RenderWindow, Snake, Style};
//! let window = RenderWindow::new((1000, 1000), "Snake", Style::CLOSE, &Default::default());
//! let mut game = Snake::new_display(0, 15, Some(window));
//! ```

use std::cmp::min;

use indexmap::IndexSet;

use crate::coord::{Coord, Direction};

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

#[cfg(feature = "display")]
use sfml::graphics::Color;
#[cfg(feature = "display")]
pub use {sfml::graphics::RenderWindow, sfml::window::Style};

/// Instance of game Snake containing board state, rng, and display
pub struct Snake {
    pub(super) snake: Vec<Coord>,
    empty: IndexSet<Coord>,
    dir: Direction,
    food: Coord,
    pub(super) size: u8,
    rng: StdRng,
    #[cfg(feature = "display")]
    pub(super) display: Option<RenderWindow>,
}

impl Snake {
    /// Creates a game instance with no display
    ///
    /// # Arguments
    ///
    /// * `seed` - Seed for random generation of food
    /// * `size` - The width/height of game board grid
    ///
    /// # Example
    ///
    /// ```
    /// use snake::snake::Snake;
    /// let mut game = Snake::new(0, 10);
    /// ```
    pub fn new(seed: u64, size: u8) -> Snake {
        let snake_first = Coord {
            x: size / 2,
            y: size / 2 - 1,
        };
        let snake_second = Coord {
            x: size / 2 - 1,
            y: size / 2 - 1,
        };

        let snake = vec![snake_first, snake_second];

        let mut empty = IndexSet::new();

        for x in 0..size {
            for y in 0..size {
                empty.insert(Coord { x, y });
            }
        }

        empty.remove(&snake_first);
        empty.remove(&snake_second);

        let mut s = Snake {
            snake,
            empty,
            dir: Direction::Right,
            food: Coord { x: 0, y: 0 },
            size,
            rng: SeedableRng::seed_from_u64(seed),
            #[cfg(feature = "display")]
            display: None,
        };

        s.gen_food();
        s
    }

    /// Creates a game instance
    ///
    /// # Arguments
    ///
    /// * `seed` - Seed for random generation of food
    /// * `size` - The width/height of game board grid
    /// * `display` - Window to display game on, provide `None` for no display
    ///
    /// # Example
    ///
    /// ```
    /// use snake::{Direction, RenderWindow, Snake, Style};
    ///
    /// let window = RenderWindow::new((1000, 1000), "Snake", Style::CLOSE, &Default::default());
    /// let mut game = Snake::new_display(0, 15, Some(window));
    /// ```
    #[cfg(feature = "display")]
    pub fn new_display(seed: u64, size: u8, display: Option<RenderWindow>) -> Snake {
        let mut s = Snake::new(seed, size);
        s.display = display;

        s.init_display();
        s.draw_square(s.food, Color::GREEN);
        s.display();
        s
    }

    /// Returns length of snake
    pub fn length(&self) -> usize {
        self.snake.len()
    }

    /// Returns current direction of snake
    pub fn current_direction(&self) -> Direction {
        self.dir
    }

    /// Returns distance of head from walls in the following order
    /// left, up-left, up, up-right, right, down-right, down, down-left
    pub fn walls(&self) -> Vec<f32> {
        let mut walls = Vec::with_capacity(8);

        let size = self.size - 1;
        let head = self.snake.first().unwrap();

        walls.push(head.x as f32 / self.size as f32);
        walls.push(min(head.x, head.y) as f32 / self.size as f32);
        walls.push(head.y as f32 / self.size as f32);
        walls.push(min(size - head.x, head.y) as f32 / self.size as f32);
        walls.push((size - head.x) as f32 / self.size as f32);
        walls.push(min(size - head.x, size - head.y) as f32 / self.size as f32);
        walls.push((size - head.y) as f32 / self.size as f32);
        walls.push(min(head.x, size - head.y) as f32 / self.size as f32);

        walls
    }

    fn snake_in_dir(&self, dir: Direction, dir2: Direction) -> f32 {
        let mut c = *self.snake.first().unwrap();

        for i in 1..self.size {
            c = match c + dir {
                Err(_) => return 1.0,
                Ok(val) => val,
            };
            c = match c + dir2 {
                Err(_) => return 1.0,
                Ok(val) => val,
            };

            if !c.in_bounds(self.size) {
                return 1.0;
            }
            if !self.empty.contains(&c) {
                return i as f32 / self.size as f32;
            }
        }

        1.0
    }

    /// Returns distance of head from snake in each direction in the following order
    /// left, up-left, up, up-right, right, down-right, down, down-left
    pub fn snake(&self) -> Vec<f32> {
        let mut snake = Vec::with_capacity(8);

        snake.push(self.snake_in_dir(Direction::Left, Direction::Center));
        snake.push(self.snake_in_dir(Direction::Up, Direction::Left));
        snake.push(self.snake_in_dir(Direction::Up, Direction::Center));
        snake.push(self.snake_in_dir(Direction::Up, Direction::Right));
        snake.push(self.snake_in_dir(Direction::Right, Direction::Center));
        snake.push(self.snake_in_dir(Direction::Down, Direction::Right));
        snake.push(self.snake_in_dir(Direction::Down, Direction::Center));
        snake.push(self.snake_in_dir(Direction::Down, Direction::Left));

        snake
    }

    /// Returns distance of head from food in each direction in the following order
    /// left, up-left, up, up-right, right, down-right, down, down-left
    pub fn food(&self) -> Vec<f32> {
        let head = self.snake.first().unwrap();
        let food = self.food;

        let mut dir = vec![1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0];

        if head.y == food.y {
            if head.x < food.x {
                dir[4] = (food.x - head.x) as f32 / self.size as f32;
            } else {
                dir[0] = (head.x - food.x) as f32 / self.size as f32;
            }
        } else if head.x == food.x {
            if head.y < food.y {
                dir[2] = (food.y - head.y) as f32 / self.size as f32;
            } else {
                dir[6] = (head.y - food.y) as f32 / self.size as f32;
            }
        } else if head.x + head.y == food.x + food.y {
            if head.x < food.x {
                dir[3] = (food.x - head.x) as f32 / self.size as f32;
            } else {
                dir[7] = (head.x - food.x) as f32 / self.size as f32;
            }
        } else if (head.x as i16 - head.y as i16).abs() == (food.x as i16 - food.y as i16).abs() {
            if head.x < food.x {
                dir[5] = (food.x - head.x) as f32 / self.size as f32;
            } else {
                dir[1] = (head.x - food.x) as f32 / self.size as f32;
            }
        }

        dir
    }

    /// Returns true or false whether snake is alive or dead
    /// ie. whether game is continuing or over
    fn alive(&self) -> bool {
        let mut snake = self.snake.iter();
        let head = snake.next().unwrap();

        if head.x >= self.size || head.y >= self.size {
            return false;
        }

        self.empty.contains(head)
    }

    /// Returns true if head is on food
    fn found_food(&self) -> bool {
        let head = self.snake.first().unwrap();
        *head == self.food
    }

    /// Spawns food at random open place on board
    fn gen_food(&mut self) -> bool {
        if self.empty.is_empty() {
            return false;
        }

        self.food = *self
            .empty
            .get_index(self.rng.gen_range(0, self.empty.len()))
            .unwrap();

        #[cfg(feature = "display")]
        self.draw_square(self.food, Color::GREEN);

        true
    }

    /// Elapses game one move. Returns true if game is still active and false if game is over (once
    /// snake has died).
    ///
    /// # Arguments
    ///
    /// * `dir` - Direction for the snake to move. If the provided direction is center or opposite
    /// the last provided direction (left/right, up/down), the snake will continue in the last
    /// provided direction
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

        let curr_pos = *self.snake.first().unwrap();

        let new_head = match self.dir {
            Direction::Left => {
                if curr_pos.x == 0 {
                    return false;
                }

                (curr_pos + self.dir).unwrap()
            }
            Direction::Right => {
                if curr_pos.x == self.size - 1 {
                    return false;
                }

                (curr_pos + self.dir).unwrap()
            }
            Direction::Up => {
                if curr_pos.y == 0 {
                    return false;
                }

                (curr_pos + self.dir).unwrap()
            }
            Direction::Down => {
                if curr_pos.y == self.size - 1 {
                    return false;
                }

                (curr_pos + self.dir).unwrap()
            }
            Direction::Center => panic!("Direction can't be center"),
        };

        self.snake.insert(0, new_head);

        if !self.alive() {
            return false;
        }

        self.empty.remove(&new_head);

        if !self.found_food() {
            let tail = self.snake.pop().unwrap();
            self.empty.insert(tail);
            #[cfg(feature = "display")]
            self.draw_square(tail, Color::BLACK);
        } else if !self.gen_food() {
            return false;
        }

        #[cfg(feature = "display")]
        {
            let head = *self.snake.first().unwrap();
            self.draw_square(head, Color::WHITE);

            self.display();
        }

        true
    }
}

// TODO Probably need to do more to close out display
#[cfg(feature = "display")]
impl Drop for Snake {
    fn drop(&mut self) {
        if let Some(d) = self.display.as_mut() {
            d.close();
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_alive() {
        let mut test = Snake::new(0, 10);
        test.snake.insert(0, Coord { x: 6, y: 4 });
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
        test.food = Coord { x: 0, y: 2 };
        assert!(!test.found_food());
        test.snake = vec![Coord { x: 0, y: 2 }];
        assert!(test.found_food());
    }

    #[test]
    fn test_snake() {
        let mut test = Snake::new(0, 10);
        test.food = Coord { x: 0, y: 2 };
        assert!(test.turn(Direction::Up));
        assert!(test.turn(Direction::Center));

        assert!(test.turn(Direction::Left));
        for _ in 0..4 {
            assert!(test.turn(Direction::Center));
        }
        assert_eq!(test.length(), 3);
        test.food = Coord { x: 9, y: 5 };

        assert!(test.turn(Direction::Down));
        for _ in 0..2 {
            assert!(test.turn(Direction::Center));
        }

        assert!(test.turn(Direction::Right));
        for _ in 0..8 {
            assert!(test.turn(Direction::Center));
        }
        assert_eq!(test.length(), 4);
    }

    #[test]
    fn test_walls() {
        let test = Snake::new(0, 10);
        assert_eq!(test.walls(), vec![0.5, 0.4, 0.4, 0.4, 0.4, 0.4, 0.5, 0.5]);
    }

    #[test]
    fn test_snake_dis() {
        let test = Snake::new(0, 10);
        assert_eq!(test.snake(), vec![0.1, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0]);
    }

    #[test]
    fn test_food() {
        let mut test = Snake::new(0, 10);
        assert_eq!(test.food(), vec![1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0]);
        test.food = Coord { x: 8, y: 4 };
        assert_eq!(test.food(), vec![1.0, 1.0, 1.0, 1.0, 0.3, 1.0, 1.0, 1.0]);
    }
}
