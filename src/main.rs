extern crate inputlib;

use std::io::{self, stdout};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use inputlib::input::*;

fn main() {
    let _stdout = stdout().into_raw_mode().unwrap();

    for input in io::stdin().keys() {
        match input {
            Ok(input) => match input {
                Key::Char(key) => {
                    if c.is_control() {
                        println!("{:?}\r", key as u8);
                    } else {
                        println!("{:?} ({})\r", key as u8, key);
                    }
                }
                Key::Ctrl('q') => break,
                _ => println!("{:?}\r", input),
            },
            Err(err) => program_die(err),
        }
    }
}
