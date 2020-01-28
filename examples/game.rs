use snake::snake::{RenderWindow, Snake, Style};

fn main() {
    let window = RenderWindow::new((1000, 1000), "Snake", Style::CLOSE, &Default::default());

    let game = Snake::new_display(0, 10, Some(window));
    loop {}
}
