use std::thread;
use std::sync::mpsc;
use std::time::Instant;
use std::io::{self, stdout, Write};

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use crate::utils::Vector2 as Vector2;

mod area;

pub struct Terminal {

    exit_app: bool,

    start_time : Instant,
    window_draw_timeout: u128,
    window_last_draw: u128,

    window_size: Vector2
}

impl Terminal{

    pub fn new() -> Terminal{

        Terminal{
            exit_app: false,

            window_draw_timeout: 30,
            window_last_draw: 0,
            start_time : Instant::now(),

            window_size: Vector2 { x: 0, y: 0 }
        }
    }

    pub fn run_terminal(&mut self){

        self.init();

        let _stdout = stdout().into_raw_mode().unwrap();
        let (tx,rx) = mpsc::channel();
        let (itx, irx) = mpsc::channel(); // to controll input handler thread


        thread::spawn(move || 
            {
                loop{
                    // reading control state first
                    let state = irx.try_recv();
                    if let Ok(st) = state {if st { break}}

                    let key_res = key_read();
                    match key_res {
                        Ok(key) => {tx.send(key).unwrap(); }
                        _ => ()
                    }
                }
            });

        loop{
            let received = rx.try_recv();
            match received {
                Ok(key) => self.key_process(key),
                _ => ()
            }
            self.draw_window();
            if self.exit_app{
                break;
            }
        }
        itx.send(true).expect("Can not terminate input handler"); // exit input handler
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

    }

    fn draw_window(&mut self){
        let cur_time =  self.start_time.elapsed().as_millis();
        if cur_time - self.window_last_draw > self.window_draw_timeout{
            print!("{}", termion::clear::All);
            io::stdout().flush().expect("Can not flush stdout");
            self.window_last_draw = cur_time;
        }
    }

    fn key_process(&mut self, key :Key){
        if let Key::Char('q') = key {
            self.exit_app = true
        }
    }
}


fn key_read()-> Result<Key, std::io::Error>{
    loop{
        if let Some(key) = std::io::stdin().lock().keys().next(){
            return key;
        }
    }
}