use std::collections::LinkedList;
use std::time::Duration;
use std::io::{ stdout, Write};
use crossterm::{
    execute,
    style::{self, Stylize, Color}, cursor, terminal,
    event::{read, poll, Event,},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen}
};
use super::cell::CellDraw;


pub fn enter(){
    execute!(stdout(), EnterAlternateScreen).unwrap();
    terminal::enable_raw_mode().unwrap();
    clear_terminal();
}

pub fn exit(){
    clear_terminal();
    execute!(stdout(), LeaveAlternateScreen).unwrap();
    terminal::disable_raw_mode().unwrap();
}

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

pub fn draw(mut list: LinkedList<CellDraw>) {

    execute!(stdout(),
        crossterm::cursor::Hide ).unwrap();
    

    loop{
        let cell = list.pop_front();
        match cell {
            Some(value) => {
                let styled = value.cell.content
                    .with(Color::from(value.cell.fg))
                    .on(Color::from(value.cell.bg));
                execute!(stdout(),
                        cursor::MoveTo(value.point.x, value.point.y),
                        style::PrintStyledContent(styled)).unwrap();
            },
            _ => break
        }
    }

    execute!(stdout(),
        crossterm::cursor::Show ).unwrap();

    stdout().flush().unwrap();

}