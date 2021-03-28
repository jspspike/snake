use sfml::window::Key;
use snake::{Direction, RenderWindow, Snake, Style};
use std::time::{Duration, Instant};

fn main() {
    let window = RenderWindow::new((1000, 1000), "Snake", Style::CLOSE, &Default::default());

    let mut game = Snake::new_display(0, 15, Some(window));
    let mut direction = Direction::Center;
    let mut last_turn = Instant::now();

    loop {
        // Handle user input
        direction = if Key::UP.is_pressed() {
            Direction::Up
        } else if Key::DOWN.is_pressed() {
            Direction::Down
        } else if Key::LEFT.is_pressed() {
            Direction::Left
        } else if Key::RIGHT.is_pressed() {
            Direction::Right
        } else {
            direction
        };
        let val = direction;
        if direction == Direction::Center {
            continue;
        }
        if last_turn.elapsed() > Duration::from_millis(250) {
            if !game.turn(val) {
                return;
            }
            last_turn = Instant::now();
        }
    }
}
