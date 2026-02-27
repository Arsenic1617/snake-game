use std::collections::VecDeque;

use crossterm::terminal::size;

use crate::models::*;
use crate::settings::center_position;

pub const CELL_WIDTH: u16 = 2;

pub struct World {
    pub grid: Vec<Vec<Tile>>,
    pub snake: VecDeque<Position>,
    pub input_queue: VecDeque<Direction>,
    pub direction: Option<Direction>,
    pub last_tail: Option<Position>,
    pub width: u16,
    pub height: u16,
    pub score: u32,
    pub game_over: bool,
    pub resized: bool,
}

impl World {
    pub fn new() -> Self {
        let (terminal_width, terminal_height) = size().unwrap();
        let width = terminal_width / CELL_WIDTH;
        let start_position = center_position();

        let mut world = Self {
            grid: vec![vec![Tile::Empty; terminal_height as usize]; width as usize],
            snake: VecDeque::from([start_position]),
            input_queue: VecDeque::with_capacity(2),
            direction: None,
            last_tail: None,
            width,
            height: terminal_height,
            score: 0,
            game_over: false,
            resized: false,
        };

        world.grid[start_position.x as usize][start_position.y as usize] = Tile::Snake;

        world
    }

    pub fn update(&mut self) {
        if self.game_over || (self.direction.is_none() && self.input_queue.is_empty()) {
            return;
        }

        if let Some(next_direction) = self.input_queue.pop_front() {
            self.direction = Some(next_direction);
        }

        let head = *self.snake.front().unwrap();
        let mut next_head = head;

        match self.direction.unwrap() {
            Direction::Up => {
                if next_head.y <= 1 {
                    next_head.y = self.height - 2;
                } else {
                    next_head.y -= 1;
                }
            }
            Direction::Right => {
                if next_head.x >= self.width - 2 {
                    next_head.x = 1;
                } else {
                    next_head.x += 1;
                }
            }
            Direction::Down => {
                if next_head.y >= self.height - 2 {
                    next_head.y = 1;
                } else {
                    next_head.y += 1;
                }
            }
            Direction::Left => {
                if next_head.x <= 1 {
                    next_head.x = self.width - 2;
                } else {
                    next_head.x -= 1;
                }
            }
        }

        match self.grid[next_head.x as usize][next_head.y as usize] {
            Tile::Snake => {
                self.game_over = true;
            }
            Tile::Food => {
                self.score += 1;
                self.snake.push_front(next_head);
                self.grid[next_head.x as usize][next_head.y as usize] = Tile::Snake;
                self.last_tail = None;
            }
            Tile::Empty => {
                self.snake.push_front(next_head);
                self.grid[next_head.x as usize][next_head.y as usize] = Tile::Snake;

                let tail = self.snake.pop_back().unwrap();
                self.grid[tail.x as usize][tail.y as usize] = Tile::Empty;
                self.last_tail = Some(tail);
            }
        }
    }
}
