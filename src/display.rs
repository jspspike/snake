use super::snake::{Coord, Snake};
use sfml::graphics::{Color, RectangleShape, RenderTarget, Shape, Transformable};
use sfml::system::Vector2f;

impl Snake {
    pub(super) fn init_display(&mut self) {
        self.display.as_mut().map(|d| {
            d.set_vertical_sync_enabled(true);
            d.clear(Color::BLACK);
        });

        for link in self.snake.clone() {
            self.draw_square(link, Color::WHITE);
        }
    }

    pub(super) fn draw_square(&mut self, pos: Coord, color: Color) {
        let grid_size = self.size as f32;

        self.display.as_mut().map(|d| {
            let size = d.size().x as f32 / grid_size;

            let mut square = RectangleShape::with_size(Vector2f { x: size, y: size });
            square.set_position(Vector2f {
                x: pos.x as f32 * size,
                y: pos.y as f32 * size,
            });
            square.set_fill_color(color);

            d.draw(&square);
        });
    }

    pub(super) fn display(&mut self) {
        self.display.as_mut().map(|d| d.display());
    }
}
