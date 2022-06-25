use std::thread;
use std::sync::mpsc;
use std::time::Instant;

use crossterm::event::KeyEvent;
use crossterm::{
    event::{Event, KeyCode},
};

use crate::utils::Vector2 as Vector2;
use self::view::View;


pub mod cell;
mod rect;
mod backend;
mod parameters;
mod symbols;
mod cursor;
mod view;


pub struct Terminal {

    exit_app: bool,

    start_time : Instant,
    window_draw_timeout: u128,
    window_last_draw: u128,

    terminal_size: Vector2, // actual size of open terminal

    view: View
}

impl Terminal{

    pub fn new() -> Terminal{

        Terminal{
            exit_app: false,

            window_draw_timeout: parameters::WINDOW_DRAW_TIME,
            window_last_draw: 0,
            start_time : Instant::now(),

            terminal_size: Default::default(),

            view: View::create()    // creating view
        }
    }

    pub fn run_terminal(&mut self){

        backend::init_panic_handler();

        self.init();

        let (tx,rx) = mpsc::channel();  // to send input events
        let (itx, irx) = mpsc::channel(); // to controll input handler thread

        // Create additional thread to read inputs from user
        thread::spawn(move || 
            {
                loop{
                    // reading control state first
                    let state = irx.try_recv();
                    if let Ok(st) = state {if st { break}}
                    // read input
                    let input_event = backend::input_event_read();
                    tx.send(input_event).unwrap();
                }
            });

        // Starting main loop
        loop{
            // 1. If got input process it
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
            // 2. Draw
            self.draw_window();
            // 3. Exit when needed
            if self.exit_app{
                break;
            }
        }

        // Terminate spawned thread
        itx.send(true).expect("Can not terminate input handler"); // exit input handler
        self.on_close();

    }



    fn init(&mut self){
        let size = termion::terminal_size();
        match size {
            Ok(s) => {
                self.terminal_size = Vector2 { x : s.0, y : s.1};
            }
            Err(e) => panic!("{}", e)
        }

        // check if size is valid for playing
        if self.terminal_size.lt(parameters::WINDOW_SIZE){
           panic!("Please increase terminal size, your terminal currently has small size");
        }

        backend::enter();   // making all preparartions for terminal

        // View init stage
        self.view.init();
    }

    fn on_close(&self){
        backend::exit();    // switching back all terminal stuff
    }

    fn draw_window(&mut self){
        let cur_time =  self.start_time.elapsed().as_millis();
        if cur_time - self.window_last_draw > self.window_draw_timeout{
            // Time is expired and let's make draw of current frame
            self.view.draw();
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
