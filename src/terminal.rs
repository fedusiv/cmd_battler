use std::thread;
use std::sync::mpsc;
use std::time::Instant;
use std::io::{ stdout, Write};

use crossterm::event::KeyEvent;
use crossterm::{
    execute, queue,
    style::{self, Stylize}, cursor, terminal, Result,
    event::{read, poll, Event, KeyCode},
};

use crate::utils::Vector2 as Vector2;

mod symbols;
mod area;
mod battle_area;
mod backend;


struct Zones{
    battle_area: battle_area::BattleArea
}

pub struct Terminal {

    exit_app: bool,

    start_time : Instant,
    window_draw_timeout: u128,
    window_last_draw: u128,

    window_size: Vector2,

    zones: Zones
}

impl Terminal{

    pub fn new() -> Terminal{

        let area = battle_area::BattleArea::new(Vector2{x:2,y:2}); // area of battles, will be displayed from point 2, 2

        let zones = Zones{
            battle_area: area
        };

        Terminal{
            exit_app: false,

            window_draw_timeout: 1000,
            window_last_draw: 0,
            start_time : Instant::now(),

            window_size: Vector2 { x: 0, y: 0 },
            zones
        }
    }

    pub fn run_terminal(&mut self){

        self.init();

        let (tx,rx) = mpsc::channel();
        let (itx, irx) = mpsc::channel(); // to controll input handler thread


        thread::spawn(move || 
            {
                loop{
                    // reading control state first
                    let state = irx.try_recv();
                    if let Ok(st) = state {if st { break}}

                    let input_event = backend::input_event_read();
                    // match key_res {
                    //     Ok(key) => {tx.send(key).unwrap(); }
                    //     _ => ()
                    // }
                    {tx.send(input_event).unwrap(); }
                }
            });

        loop{
            let event_received = rx.try_recv();
            match event_received {
                Ok(event) => {
                    match event {
                        Event::Key(event) => self.key_process(event),
                        Event::Mouse(_) => todo!(),
                        Event::Resize(_, _) => todo!(),
                    }
                }
                _ => ()
            }
            self.draw_window();
            if self.exit_app{
                break;
            }
        }


        itx.send(true).expect("Can not terminate input handler"); // exit input handler
        self.on_close();

    }

    fn init(&mut self){
        let size = termion::terminal_size();
        match size {
            Ok(s) => {
                self.window_size = Vector2 { x : s.0, y : s.1};
            }
            Err(e) => panic!("{}", e)
        }

        // check if size is valid for playing
        if self.window_size.lt(Vector2{x:30, y:30}){
            panic!("Please increase terminal size, your terminal currently has small size");
        }

        terminal::enable_raw_mode().unwrap();
    }

    fn on_close(&self){
        terminal::disable_raw_mode().unwrap();
        backend::clear_terminal();
    }

    fn draw_window(&mut self){
        let cur_time =  self.start_time.elapsed().as_millis();
        if cur_time - self.window_last_draw > self.window_draw_timeout{

            backend::clear_terminal();
            // drawing operations
            // iterate through each symbol/pixel. Because this is command line based application, each minimal drawing is equal to one symbol
            // for more representation let's do it in two loop implementation
            for y in 0..self.window_size.y{
                for x in 0..self.window_size.x{
                    // x, y coordinate of elements to be drawn
                    let point = Vector2{x,y};
                    // first check battle area
                    let mut symbol = ' ';
                    if self.zones.battle_area.area.is_in_area(&point){
                        symbol = self.zones.battle_area.area.get_symbol(&point);
                    }
                    queue!(stdout(), cursor::MoveTo(x,y), style::PrintStyledContent( symbol.grey())).unwrap();
                }
            }
            stdout().flush().unwrap();
            
            self.window_last_draw = cur_time;
        }
    }

    fn key_process(&mut self, key :KeyEvent){
        // ctrl, alt and shift are parsed as modifiers, need to understand how to use them
        match key.code{
            KeyCode::Char('q') => self.exit_app = true,
            KeyCode::Backspace => todo!(),
            KeyCode::Enter => todo!(),
            KeyCode::Left => todo!(),
            KeyCode::Right => todo!(),
            KeyCode::Up => todo!(),
            KeyCode::Down => todo!(),
            KeyCode::Home => todo!(),
            KeyCode::End => todo!(),
            KeyCode::PageUp => todo!(),
            KeyCode::PageDown => todo!(),
            KeyCode::Tab => todo!(),
            KeyCode::BackTab => todo!(),
            KeyCode::Delete => todo!(),
            KeyCode::Insert => todo!(),
            KeyCode::F(_) => todo!(),
            KeyCode::Null => todo!(),
            KeyCode::Esc => todo!(),
            _ => ()
        }
    }
}
