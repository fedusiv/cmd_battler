use std::collections::LinkedList;
use std::thread;
use std::sync::mpsc;
use std::time::Instant;

use crossterm::event::KeyEvent;
use crossterm::{
    event::{Event, KeyCode},
};

use crate::utils::Vector2 as Vector2;

use self::cell::CellDraw;


pub mod cell;
mod rect;
mod battle_area;
mod backend;
mod parameters;
mod symbols;


struct Zones{
    battle_area: battle_area::BattleArea
}

pub struct Terminal {

    exit_app: bool,

    start_time : Instant,
    window_draw_timeout: u128,
    window_last_draw: u128,

    terminal_size: Vector2, // actual size of open terminal
    window_size: Vector2,   // size of operatable window, which app uses

    buffer: Vec<cell::Cell>,

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

            window_draw_timeout: parameters::WINDOW_DRAW_TIME,
            window_last_draw: 0,
            start_time : Instant::now(),

            terminal_size: Default::default(),
            window_size: Default::default(),
            buffer: vec![cell::Cell::default(); parameters::WINDOW_SIZE_US],
            zones
        }
    }

    pub fn run_terminal(&mut self){

        backend::init_panic_handler();

        self.init();

        let (tx,rx) = mpsc::channel();
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
        if self.terminal_size.lt(Vector2{x:30, y:30}){
            panic!("Please increase terminal size, your terminal currently has small size");
        }

        // if terminal size is okay, let's state window size we required.
        self.window_size = parameters::WINDOW_SIZE;

        backend::enter();   // making all preparartions for terminal
    }

    fn on_close(&self){
        backend::exit();    // switching back all terminal stuff
    }

    fn draw_window(&mut self){
        let cur_time =  self.start_time.elapsed().as_millis();
        if cur_time - self.window_last_draw > self.window_draw_timeout{
            // create list with elements, which should be drawn
            let mut draw_list:LinkedList<cell::CellDraw> = LinkedList::new();
            // iterate through current buffer of all cell data
            // if it is not equal, need to draw and save
            for y in 0..self.window_size.y{ // for representative used two loops as x and y coordinates
                for x in 0..self.window_size.x{
                    let current_id = (y * self.window_size.x + x) as usize;
                    let mut point = Vector2{x, y};  // curent point
                    // Battle area
                    if self.zones.battle_area.rect.is_in_area(&point){
                        // in this rect
                        if self.buffer[current_id] != self.zones.battle_area.rect.content_unsafe(&point){
                            // change value of buffer
                            self.buffer[current_id] =  self.zones.battle_area.rect.content_unsafe(&point).clone();
                            // if is not equal need to put it into list for drawing
                            self.zones.battle_area.rect.convert_to_global_coor(&mut point); // converted to glonal area
                            let cell = CellDraw{
                                cell: self.buffer[current_id],
                                point
                            };
                            draw_list.push_back(cell);
                        }
                    }
                    // if point is not in anyzone, definitely we do not need to redraw it
                }
            }
            backend::draw(draw_list);
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
