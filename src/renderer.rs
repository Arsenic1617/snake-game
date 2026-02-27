use std::io::{Stdout, Write};

use crossterm::{
    cursor::{Hide, MoveTo},
    event::{Event, KeyEventKind, read},
    queue,
    style::{Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{Clear, ClearType, size},
};

use crate::settings::*;
use crate::world::{CELL_WIDTH, World};

fn fill_background(stdout: &mut Stdout) {
    let (terminal_width, terminal_height) = size().unwrap();

    queue!(stdout, SetBackgroundColor(rgb(SCREEN_BG_COLOR))).unwrap();

    for y in 0..terminal_height {
        queue!(
            stdout,
            MoveTo(0, y),
            Print(" ".repeat(terminal_width as usize))
        )
        .unwrap();
    }

    queue!(stdout, ResetColor).unwrap();
}

pub fn setup_screen(stdout: &mut Stdout) {
    let (terminal_width, terminal_height) = size().unwrap();
    let width = terminal_width / CELL_WIDTH;

    queue!(stdout, Hide, Clear(ClearType::All), Clear(ClearType::Purge)).unwrap();

    fill_background(stdout);

    queue!(stdout, SetBackgroundColor(rgb(BORDER_COLOR))).unwrap();

    for x in 0..width {
        queue!(
            stdout,
            MoveTo(x * CELL_WIDTH, 0),
            Print("  "),
            MoveTo(x * CELL_WIDTH, terminal_height - 1),
            Print("  "),
        )
        .unwrap();
    }

    for y in 0..terminal_height {
        queue!(
            stdout,
            MoveTo(0, y),
            Print("  "),
            MoveTo(terminal_width - CELL_WIDTH, y),
            Print("  "),
        )
        .unwrap();
    }

    queue!(stdout, ResetColor).unwrap();

    stdout.flush().unwrap();
}

pub fn render_world(stdout: &mut Stdout, world: &World) {
    if let Some(tail) = world.last_tail {
        queue!(
            stdout,
            MoveTo(tail.x * CELL_WIDTH, tail.y),
            SetBackgroundColor(rgb(SCREEN_BG_COLOR)),
            Print("  "),
            ResetColor,
        )
        .unwrap();
    }

    for (i, position) in world.snake.iter().enumerate() {
        let color = if i == 0 {
            rgb(SNAKE_HEAD_COLOR)
        } else {
            rgb(SNAKE_COLOR)
        };

        queue!(
            stdout,
            MoveTo(position.x * CELL_WIDTH, position.y),
            SetBackgroundColor(color),
            Print("  "),
            ResetColor,
        )
        .unwrap();
    }

    queue!(stdout, ResetColor).unwrap();

    stdout.flush().unwrap();
}

pub fn render_score(stdout: &mut Stdout, world: &World) {
    let (terminal_width, _) = size().unwrap();
    let text = format!(" SCORE: {} ", world.score);

    queue!(
        stdout,
        MoveTo((terminal_width - text.len() as u16) / 2, 0),
        SetBackgroundColor(rgb(SCORE_BG_COLOR)),
        SetForegroundColor(rgb(SCORE_FG_COLOR)),
        Print(text),
        ResetColor,
    )
    .unwrap();

    stdout.flush().unwrap();
}

pub fn render_game_over(stdout: &mut Stdout, world: &mut World) {
    let (terminal_width, height) = size().unwrap();
    let text = format!("★  GAME OVER  —  SCORE: {}  ★", world.score);

    queue!(stdout, Clear(ClearType::All), Clear(ClearType::Purge)).unwrap();

    fill_background(stdout);

    queue!(
        stdout,
        MoveTo((terminal_width - text.len() as u16) / 2, height / 2 - 1),
        SetBackgroundColor(rgb(SCREEN_BG_COLOR)),
        SetForegroundColor(rgb((220, 50, 50))),
        Print(text),
        ResetColor,
    )
    .unwrap();

    stdout.flush().unwrap();

    loop {
        match read() {
            Ok(Event::Key(key_event)) if key_event.kind == KeyEventKind::Press => break,
            Ok(Event::Resize(_, _)) => {
                world.resized = true;
                return;
            }
            _ => {}
        }
    }
}
