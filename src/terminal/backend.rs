use std::time::Duration;
use std::io::{ stdout, Write};
use crossterm::{
    execute, queue,
    style::{self, Stylize}, cursor, terminal, Result,
    event::{read, poll, Event, KeyCode},
};

pub fn input_event_read()-> Event{
    loop {
        // `poll()` waits for an `Event` for a given time period
        if poll(Duration::from_millis(200)).unwrap() {
            return read().unwrap();
        } 
    }
}

pub fn clear_terminal() {
    execute!(stdout(),
    cursor::MoveTo(0,0),
    terminal::Clear(terminal::ClearType::All)).unwrap();
}