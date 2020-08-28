pub mod coord;
#[cfg(feature = "display")]
mod display;
pub mod snake;

pub use crate::snake::Snake;
pub use coord::Direction;

#[cfg(feature = "display")]
pub use crate::snake::{RenderWindow, Style};
