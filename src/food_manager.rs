use std::io::{Stdout, Write};

use crossterm::{
    cursor::MoveTo,
    queue,
    style::{Print, ResetColor, SetBackgroundColor},
};
use rand::random_range;

use crate::settings::SCREEN_BG_COLOR;
use crate::world::{CELL_WIDTH, World};
use crate::{models::Tile, settings::rgb};

pub fn spawn_food(stdout: &mut Stdout, world: &mut World) {
    let (x, y) = loop {
        let x = random_range(1..world.width - 1);
        let y = random_range(1..world.height - 1);

        if world.grid[x as usize][y as usize] == Tile::Empty {
            break (x, y);
        }
    };

    world.grid[x as usize][y as usize] = Tile::Food;

    queue!(
        stdout,
        MoveTo(x * CELL_WIDTH, y),
        SetBackgroundColor(rgb(SCREEN_BG_COLOR)),
        Print("🍎"),
        ResetColor,
    )
    .unwrap();

    stdout.flush().unwrap();
}
