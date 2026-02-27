use std::time::Duration;

use crossterm::event::{Event, KeyCode, KeyEventKind, poll, read};

use crate::models::Direction;
use crate::world::World;

pub fn handle_input(world: &mut World) {
    while poll(Duration::ZERO).unwrap() {
        let Ok(event) = read() else {
            continue;
        };

        match event {
            Event::Resize(_, _) => {
                world.game_over = true;
                world.resized = true;
                break;
            }
            Event::Key(key_event) => {
                if key_event.kind != KeyEventKind::Press {
                    continue;
                }

                match key_event.code {
                    KeyCode::Esc => {
                        world.game_over = true;
                        break;
                    }
                    code => {
                        let Some(direction) = key_to_direction(code) else {
                            continue;
                        };

                        if world.input_queue.len() < 2 && !is_opposite_or_same(direction, world) {
                            world.input_queue.push_back(direction);
                        }
                    }
                }
            }
            _ => (),
        }
    }
}

fn key_to_direction(code: KeyCode) -> Option<Direction> {
    match code {
        KeyCode::Up | KeyCode::Char('w') => Some(Direction::Up),
        KeyCode::Down | KeyCode::Char('s') => Some(Direction::Down),
        KeyCode::Left | KeyCode::Char('a') => Some(Direction::Left),
        KeyCode::Right | KeyCode::Char('d') => Some(Direction::Right),
        _ => None,
    }
}

fn is_opposite_or_same(direction: Direction, world: &World) -> bool {
    let Some(last) = world.input_queue.back().copied().or(world.direction) else {
        return false;
    };

    matches!(
        (direction, last),
        (Direction::Up, Direction::Down)
            | (Direction::Down, Direction::Up)
            | (Direction::Left, Direction::Right)
            | (Direction::Right, Direction::Left)
            | (Direction::Up, Direction::Up)
            | (Direction::Down, Direction::Down)
            | (Direction::Left, Direction::Left)
            | (Direction::Right, Direction::Right)
    )
}
