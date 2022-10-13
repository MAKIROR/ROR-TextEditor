use std::io::{self, stdout};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub struct Editor {}
impl Editor {
    pub fn run(&self) {
        let _stdout = stdout().into_raw_mode().unwrap();
        loop {
            if let Err(error) = self.match_key() {
                Self::program_die(error);
            }
        }
    }
    pub fn default() -> Self {
        Self {}
    }
    fn match_key(&self) -> Result<(), std::io::Error> {
        let key = Self::read_key()?;
        match key {
            Key::Ctrl('q') => panic!("Program end"),
            _ => (),
        }
        Ok(())
    }
    fn read_key() -> Result<(), std::io::Error> {
        loop {
            if let Some(input) = io::stdin().lock().keys().next() {
                return input;
            }
        }
    }
    pub fn program_die(error : std::io::Error) {
        panic!("{}",error);
    }
}

