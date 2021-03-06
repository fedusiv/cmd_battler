use super::cell::CellDraw;
use super::parameters;
use crossterm::{
    cursor,
    event::{poll, read, Event},
    execute,
    style::{self, Color, Stylize},
    terminal,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
use std::collections::LinkedList;
use std::io::{stdout, Write};
use std::panic;
use std::time::Duration;

pub fn enter() {
    execute!(stdout(), EnterAlternateScreen).unwrap();
    terminal::enable_raw_mode().unwrap();
    clear_terminal();
    execute!(stdout(), cursor::DisableBlinking, crossterm::cursor::Hide).unwrap();
}

pub fn exit() {
    clear_terminal();
    execute!(stdout(), LeaveAlternateScreen).unwrap();
    terminal::disable_raw_mode().unwrap();
    execute!(stdout(), cursor::EnableBlinking, crossterm::cursor::Show).unwrap();
}

pub fn input_event_read() -> Event {
    loop {
        // `poll()` waits for an `Event` for a given time period
        if poll(Duration::from_millis(parameters::INPUT_POLLING_TIMEOUT)).unwrap() {
            return read().unwrap();
        }
    }
}

pub fn clear_terminal() {
    execute!(
        stdout(),
        cursor::MoveTo(0, 0),
        terminal::Clear(terminal::ClearType::Purge)
    )
    .unwrap();
}

pub fn draw(mut list: LinkedList<CellDraw>) {
    loop {
        let cell = list.pop_front();
        match cell {
            Some(value) => {
                let styled = value
                    .cell
                    .content
                    .with(Color::from(value.cell.fg))
                    .on(Color::from(value.cell.bg));
                execute!(
                    stdout(),
                    cursor::MoveTo(value.point.x as u16, value.point.y as u16),
                    style::PrintStyledContent(styled)
                )
                .unwrap();
            }
            _ => break,
        }
    }

    stdout().flush().unwrap();
}

pub fn init_panic_handler() {
    panic::set_hook(Box::new(|panic_info| {
        exit();
        if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
            println!("panic occurred: {s:?}");
        } else {
            println!("panic occurred");
        }
    }));
}
