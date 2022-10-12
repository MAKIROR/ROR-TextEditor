extern crate inputlib;

use std::io::{self, stdout, Read};
use termion::raw::IntoRawMode;
use inputlib::input::processing::*;

fn main() {
    let _stdout = stdout().into_raw_mode().unwrap();

    for input in io::stdin().bytes() {
        let input = input.unwrap();
        let value = input as char;
        if value.is_control() {
            println!("{:?} \r", input);
        } else {
            println!("{:?} ({})\r", input,value);
        }

        if input == to_ctrl_byte('q') {
            break;
        }
    }
}
