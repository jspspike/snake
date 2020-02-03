//! # Quick Start
//!
//! Game without display
//! ```
//! use snake::snake::{Snake, Direction};
//! let mut game = Snake::new(0, 10);
//!
//! game.turn(Direction::Down);
//! ```
//!
//! Game with display
//! ```
//! use snake::snake::{Direction, RenderWindow, Snake, Style};
//! let window = RenderWindow::new((1000, 1000), "Snake", Style::CLOSE, &Default::default());
//! let mut game = Snake::new_display(0, 15, Some(window));
//! ```

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

use sfml::graphics::Color;
pub use sfml::graphics::RenderWindow;
pub use sfml::window::Style;

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
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub(super) struct Coord {
    pub(super) x: u8,
    pub(super) y: u8,
}

/// Instance of game Snake containing board state, rng, and display
pub struct Snake {
    pub(super) snake: Vec<Coord>,
    dir: Direction,
    food: Coord,
    pub(super) size: u8,
    rng: StdRng,
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
        Snake::new_display(seed, size, None)
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
    /// use snake::snake::{Direction, RenderWindow, Snake, Style};
    ///
    /// let window = RenderWindow::new((1000, 1000), "Snake", Style::CLOSE, &Default::default());
    /// let mut game = Snake::new_display(0, 15, Some(window));
    /// ```
    pub fn new_display(seed: u64, size: u8, display: Option<RenderWindow>) -> Snake {
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

        let mut s = Snake {
            snake,
            dir: Direction::Right,
            food: Coord { x: 0, y: 0 },
            size,
            rng: SeedableRng::seed_from_u64(seed),
            display,
        };

        s.init_display();
        s.gen_food();
        s.display();
        s
    }

    /// Returns length of snake
    pub fn length(&self) -> usize {
        self.snake.len()
    }

    /// Returns true or false whether snake is alive or dead
    /// ie. whether game is continuing or over
    fn alive(&self) -> bool {
        let mut snake = self.snake.iter();
        let head = snake.next().unwrap();

        if head.x >= self.size || head.y >= self.size {
            return false;
        }

        snake.all(|p| head != p)
    }

    /// Returns true if head is on food
    fn found_food(&self) -> bool {
        let head = self.snake.first().unwrap();
        return *head == self.food;
    }

    /// Spawns food at random open place on board
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
        self.draw_square(self.food, Color::GREEN);
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

        self.snake.insert(
            0,
            match self.dir {
                Direction::Left => {
                    if curr_pos.x == 0 {
                        return false;
                    }

                    Coord {
                        x: curr_pos.x - 1,
                        y: curr_pos.y,
                    }
                }
                Direction::Right => {
                    if curr_pos.x == self.size - 1 {
                        return false;
                    }

                    Coord {
                        x: curr_pos.x + 1,
                        y: curr_pos.y,
                    }
                }
                Direction::Up => {
                    if curr_pos.y == 0 {
                        return false;
                    }

                    Coord {
                        x: curr_pos.x,
                        y: curr_pos.y - 1,
                    }
                }
                Direction::Down => {
                    if curr_pos.y == self.size - 1 {
                        return false;
                    }

                    Coord {
                        x: curr_pos.x,
                        y: curr_pos.y + 1,
                    }
                }
                Direction::Center => panic!("Direction can't be center"),
            },
        );
        if !self.found_food() {
            let tail = self.snake.pop();
            self.draw_square(tail.unwrap(), Color::BLACK);
        } else {
            self.gen_food();
        }

        let head = *self.snake.first().unwrap();
        self.draw_square(head, Color::WHITE);

        self.display();
        self.alive()
    }
}

// TODO Probably need to do more to close out display
impl Drop for Snake {
    fn drop(&mut self) {
        self.display.as_mut().map(|d| {
            d.close();
        });
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
        test.snake = vec![Coord { x: 5, y: 0 }];
        assert!(test.found_food());
    }

    #[test]
    fn test_snake() {
        let mut test = Snake::new(0, 10);
        assert!(test.turn(Direction::Up));
        for _ in 0..3 {
            assert!(test.turn(Direction::Center));
        }
        assert_eq!(test.length(), 3);

        assert!(test.turn(Direction::Left));
        assert!(test.turn(Direction::Down));
        for _ in 0..8 {
            assert!(test.turn(Direction::Center));
        }
        assert_eq!(test.length(), 4);
    }
}
