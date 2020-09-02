use sfml::window::Key;
use snake::{Direction, RenderWindow, Snake, Style};
use std::sync::{Arc, Mutex};
use std::{thread, time};

fn main() {
    let window = RenderWindow::new((1000, 1000), "Snake", Style::CLOSE, &Default::default());

    let mut game = Snake::new_display(0, 15, Some(window));
    let direction = Arc::new(Mutex::new(Direction::Center));
    let dir = direction.clone();

    // Thread to handle user input
    thread::spawn(move || loop {
        let mut dir = dir.lock().unwrap();
        *dir = if Key::Up.is_pressed() {
            Direction::Up
        } else if Key::Down.is_pressed() {
            Direction::Down
        } else if Key::Left.is_pressed() {
            Direction::Left
        } else if Key::Right.is_pressed() {
            Direction::Right
        } else {
            *dir
        };
        drop(dir);
        let delay = time::Duration::from_millis(10);
        thread::sleep(delay);
    });

    loop {
        let direction = direction.lock().unwrap();
        let val = *direction;
        if *direction == Direction::Center {
            continue;
        }
        drop(direction);
        if !game.turn(val) {
            return;
        }
        let delay = time::Duration::from_millis(250);
        thread::sleep(delay);
    }
}
