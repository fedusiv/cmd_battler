use std::panic;
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
use super::parameters;


pub fn enter(){
    execute!(stdout(), EnterAlternateScreen).unwrap();
    terminal::enable_raw_mode().unwrap();
    clear_terminal();
    execute!(stdout(),cursor::DisableBlinking).unwrap();
}

pub fn exit(){
    clear_terminal();
    execute!(stdout(), LeaveAlternateScreen).unwrap();
    terminal::disable_raw_mode().unwrap();
    execute!(stdout(),cursor::EnableBlinking).unwrap();
}

pub fn input_event_read()-> Event{
    loop {
        // `poll()` waits for an `Event` for a given time period
        if poll(Duration::from_millis(parameters::INPUT_POLLING_TIMEOUT)).unwrap() {
            return read().unwrap();
        } 
    }
}

pub fn clear_terminal() {
    execute!(stdout(),
    cursor::MoveTo(0,0),
    terminal::Clear(terminal::ClearType::Purge)).unwrap();
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

pub fn init_panic_handler(){
    panic::set_hook(Box::new(|panic_info| {
        exit();
        if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
            println!("panic occurred: {s:?}");
        } else {
            println!("panic occurred");
        }
    }));
}



