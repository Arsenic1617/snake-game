use crossterm::style::Color;
use crossterm::terminal::size;

use crate::models::Position;
use crate::world::CELL_WIDTH;

pub const SCREEN_BG_COLOR: (u8, u8, u8) = (20, 20, 25);
pub const BORDER_COLOR: (u8, u8, u8) = (100, 100, 100);
pub const SNAKE_COLOR: (u8, u8, u8) = (0, 160, 80);
pub const SNAKE_HEAD_COLOR: (u8, u8, u8) = (0, 210, 100);
pub const SCORE_BG_COLOR: (u8, u8, u8) = (200, 0, 0);
pub const SCORE_FG_COLOR: (u8, u8, u8) = (255, 255, 255);

pub fn rgb(color: (u8, u8, u8)) -> Color {
    Color::Rgb {
        r: color.0,
        g: color.1,
        b: color.2,
    }
}

pub fn center_position() -> Position {
    let (terminal_width, terminal_height) = size().unwrap();

    Position {
        x: terminal_width / CELL_WIDTH / 2,
        y: terminal_height / 2,
    }
}
