use std::io::{self, stdout, Read};
use termion::raw::IntoRawMode;

fn main() {
    let _stdout = stdout().into_raw_mode().unwrap();

    for input in io::stdin().bytes() {
        let value = input.unwrap() as char;
        
        if value.is_control() {
            
        } else {
            
        }
    }
}
