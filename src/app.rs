use crossterm::{
    event::{self, Event, KeyEvent},
    queue,
    style::Print,
};
use std::io::{stdout, Write};
//use std::process::exit;
use crate::exit;
use crate::io;
use std::collections::VecDeque;
mod input;

pub struct App {
    event_queue: VecDeque<Event>,
    exit: bool,
}

impl App {
    pub fn new() -> App {
        App {
            event_queue: VecDeque::new(),
            exit: false,
        }
    }
    //returns false when the aplication exits
    pub fn update(&mut self) -> bool {
        self.handle_input();
        self.exit
    }

    fn handle_input(&mut self) {
        input::input(&mut self.event_queue);

        loop {
            let event = self.event_queue.pop_front();
            let event = match event {
                None => break,
                Some(e) => e,
            };
            match event {
                Event::Key(k) => self.handle_key(k),
                _ => (),
            }
        }
    }
    fn handle_key(&mut self, key: event::KeyEvent) {
        if key.modifiers == event::KeyModifiers::CONTROL && key.code == event::KeyCode::Char('c') {
            self.exit = true
        } // exits when ^C is pressed
    }
}
