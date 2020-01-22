use quicksilver::{
    Result,
    geom::{Circle, Line, Rectangle, Transform, Triangle, Vector},
    graphics::{Background::Col, Color},
    lifecycle::{Settings, State, Window, run},
};

struct Display;

impl State for Display {
    fn new() -> Result<Display> {
        Ok(Display)
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        
    }
}
