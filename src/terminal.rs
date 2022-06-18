use std::thread;
use std::sync::mpsc;
use std::time::Instant;
use std::io::{self, stdout, Write};

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;


pub struct Terminal {

    exit_app: bool,

    start_time : Instant,
    window_draw_timeout: u128,
    window_last_draw: u128
}

impl Terminal{

    pub fn new() -> Terminal{

        Terminal{
            exit_app: false,

            window_draw_timeout: 30,
            window_last_draw: 0,
            start_time : Instant::now()
        }
    }

    pub fn run_terminal(&mut self){
        let _stdout = stdout().into_raw_mode().unwrap();
        let (tx,rx) = mpsc::channel();
        let (itx, irx) = mpsc::channel(); // to controll input handler thread


        thread::spawn(move || 
            {
                loop{
                    // reading control state first
                    let state = irx.try_recv();
                    match state {
                        Ok(st) => {if st { break}},
                        _ => ()
                    }

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


    fn draw_window(&mut self){
        let cur_time =  self.start_time.elapsed().as_millis();
        if cur_time - self.window_last_draw > self.window_draw_timeout{
            print!("{}", termion::clear::All);
            io::stdout().flush().expect("Can not flush stdout");
            self.window_last_draw = cur_time;
        }
    }

    fn key_process(&mut self, key :Key){
        match key {
            Key::Char('q') => {
                self.exit_app = true
            },
            _ => ()
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