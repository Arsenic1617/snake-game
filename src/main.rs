mod food_manager;
mod input_handler;
mod models;
mod renderer;
mod settings;
mod world;

use std::{io::stdout, thread::sleep, time::Duration};

use crossterm::{
    event::{poll, read},
    terminal::{disable_raw_mode, enable_raw_mode},
};

use food_manager::spawn_food;
use input_handler::handle_input;
use renderer::{render_game_over, render_score, render_world, setup_screen};
use world::World;

fn main() {
    enable_raw_mode().unwrap();

    let mut stdout = stdout();
    let mut world = World::new();

    setup_screen(&mut stdout);
    spawn_food(&mut stdout, &mut world);

    while poll(Duration::ZERO).unwrap() {
        let _ = read();
    }

    while !world.game_over {
        handle_input(&mut world);

        world.update();

        let has_food = world.grid.iter().any(|r| r.contains(&models::Tile::Food));
        if !has_food {
            spawn_food(&mut stdout, &mut world);
        }

        render_world(&mut stdout, &world);
        render_score(&mut stdout, &world);

        if !world.resized {
            sleep(Duration::from_millis(80));

            if world.game_over {
                render_game_over(&mut stdout, &mut world);
            }
        }
    }

    disable_raw_mode().unwrap();
}
