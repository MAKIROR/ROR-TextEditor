
use crate::Terminal;
use termion::event::Key;
use crate::Document;
use crate::Row;
use termion::color;
use std::time::{Duration,Instant};
use std::env;
use std::collections::BTreeSet;

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
    cursor_position: Position,
    document: Document,
    offset: Position,
    status_message: StatusMessage,
    quit_times: u8,
}

#[derive(Default)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

const STATUS_BG_COLOR: color::Rgb = color::Rgb(239, 239, 239);
const STATUS_FG_COLOR: color::Rgb = color::Rgb(63, 63, 63);
const VERSION: &str = env!("CARGO_PKG_VERSION");
const QUIT_TIMES: u8 = 3;

impl Editor {
    pub fn default() -> Self {
        let args: Vec<String> = env::args().collect();
        let mut initial_status = String::from("HELP: Ctrl-Q = quit");
        let document = if args.len() > 1 {
            let file_name = &args[1];
            let doc = Document::open(&file_name);
            if doc.is_ok() {
                doc.unwrap()
            } else {
                initial_status = format!("ERR: Could not open file: {}", file_name);
                Document::default()
            }
        } else {
            Document::default()
        };

        Self {
            should_quit: false,
            terminal: Terminal::default().expect("Failed to initialize terminal"),
            document,
            cursor_position: Position::default(),
            offset: Position::default(),
            status_message: StatusMessage::from(initial_status),
            quit_times: QUIT_TIMES,
        }
    }

    pub fn run(&mut self) {
        loop {
            if let Err(error) = self.refresh_screen() {
                program_die(error);
            }
            if self.should_quit {
                break;
            }
            if let Err(error) = self.match_key() {
                program_die(error);
            }
        }
    }

    fn match_key(&mut self) -> Result<(), std::io::Error> {
        let Position { mut y, mut x } = self.cursor_position;
        let mut width = if let Some(row) = self.document.row(y) {
            row.len()
        } else {
            0
        };
        let key = Terminal::read_key()?;
        match key {
            Key::Ctrl('q') => {
                if self.quit_times > 0 && self.document.is_dirty() {
                    self.status_message = StatusMessage::from(format!(
                        "WARNING! File has unsaved changes. Press Ctrl-Q {} more times to quit.",
                        self.quit_times
                    ));
                    self.quit_times -= 1;
                    return Ok(());
                }
                self.should_quit = true
            }
            Key::Ctrl('s') => self.save(),
            Key::Ctrl('d') => {
                if let Some(command) = self.prompt("").unwrap_or(None) {
                    self.command_board(&command);
                }
            }
            Key::Char(c) => {
                let insert_result = self.document.insert(&self.cursor_position, c);
                if insert_result == 1 {
                    self.move_cursor(Key::Down);
                } else {
                    self.move_cursor(Key::Right);
                }

            }
            Key::Backspace => {
                if self.cursor_position.x > 0 || self.cursor_position.y > 0 {
                    let delete_result = self.document.delete(&self.cursor_position);
                    if delete_result == 1 {
                        self.move_cursor(Key::Up);
                        x = width;
                    } else {
                        self.move_cursor(Key::Left);
                    }
                }
            }
            Key::Up 
            | Key::Down 
            | Key::Left 
            | Key::Right
            | Key::PageUp
            | Key::PageDown
            | Key::End
            | Key::Home
            => self.move_cursor(key),
            _ => (),
        }
        self.scroll();
        Ok(())
    }
    fn command_board(&mut self,value: &str) -> Result<(), std::io::Error> {
        let location = String::from(value);
        let command :Vec<&str> = location.split(" ").collect();  
            loop {
                match &command[0][..] as &str {
                    "find" => {
                        if command.len() != 2 {
                            self.status_message = StatusMessage::from(format!("Unqualified find command:{:?}.",command));
                        } else {
                            if let Some(result) = self.document.find(&command[1][..]) {
                                self.status_message = StatusMessage::from(format!("Successful found :{}", command[1]));
                                self.cursor_position = result.get(0);
                                let number = 0;
                                let len = result.len()
                                loop {
                                    let key = Terminal::read_key()?;
                                    match key {
                                        Key::Right => {
                                            if number < result.len() {
                                                self.cursor_position = result.get(number + 1);
                                                let number = number +1;
                                            } else {
                                                self.cursor_position = result.get(0);
                                                let number = 0;
                                            }
                                        }
                                        Key::Left => {
                                            if number == 0 {
                                                self.cursor_position = result.get(len - 1);
                                                let number = len - 1;
                                            } else {
                                                self.cursor_position = result.get(number - 1);
                                                let number = number - 1;
                                            }
                                        }
                                        Key::Ctrl('q') 
                                        | Key::Esc => break,
                                    }
                                }
                            } else {
                                self.status_message = StatusMessage::from(format!("Not found :{}.", command[1]));
                            }
                        }
                        break;
                    }
                    "quit"  => {
                        self.status_message = StatusMessage::from(format!("Exit the command board"));
                        break;
                    }
                    _ => {
                        self.status_message = StatusMessage::from(format!("Not found :{:?}.", command));
                        break;
                    },
                }
            }
        Ok(())
    }
    fn save(&mut self) {
        if self.document.file_name.is_none() {
            let new_name = self.prompt("Save as: ").unwrap_or(None);
            if new_name.is_none() {
                self.status_message = StatusMessage::from("Save aborted.".to_string());
                return;
            }
            self.document.file_name = new_name;
        }

        if self.document.save().is_ok() {
            self.status_message = StatusMessage::from("File saved successfully.".to_string());
        } else {
            self.status_message = StatusMessage::from("Error writing file!".to_string());
        }
    }
    fn move_cursor(&mut self, key: Key) {
        let Position { mut y, mut x } = self.cursor_position;
        let size = self.terminal.size();
        let height = self.document.len();
        let mut width = if let Some(row) = self.document.row(y) {
            row.len()
        } else {
            0
        };
        match key {
            Key::Up => y = y.saturating_sub(1),
            Key::Down => {
                if y < height {
                    y = y.saturating_add(1)
                }
            }
            Key::Left => x = x.saturating_sub(1),
            Key::Right => {
                if x < width {
                    x = x.saturating_add(1)
                }
            }
            Key::PageUp => y=0,
            Key::PageDown => y=height,
            Key::Home => x=0,
            Key::End => x = width,
            _ => (),
        }

        width = if let Some(row) = self.document.row(y) {
            row.len()
        } else {
            0
        };
        if x > width {
            x = width;
        }

        self.cursor_position = Position { x, y }
    }
    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::cursor_hide();          
        Terminal::cursor_position(&Position::default());

        if self.should_quit {
            Terminal::clear_screen();  
            println!("Exit the program. Goodbye :D\r");
        } else {
            self.draw_rows();
            self.draw_status_bar();
            self.draw_message_bar();
            Terminal::cursor_position(&Position {
                x: self.cursor_position.x.saturating_sub(self.offset.x),
                y: self.cursor_position.y.saturating_sub(self.offset.y),
            });
        }
        Terminal::cursor_show();
        Terminal::flush()
    }
    fn draw_rows(&self) {
        let height = self.terminal.size().height;
        for terminal_row in 0..height {
            Terminal::clear_current_line();
            if let Some(row) = self.document.row(terminal_row as usize + self.offset.y) {
                self.draw_row(row);
            } else if terminal_row == height / 3 && self.document.is_empty() {
                self.draw_welcome_message();
            } else {
                println!("~\r");
            }
        }
    }
    pub fn draw_row(&self, row: &Row) {
        let width = self.terminal.size().width as usize;
        let start = self.offset.x;
        let end = self.offset.x + width;
        let row = row.render(start, end);
        println!("{}\r", row)
    }
    fn draw_welcome_message(&self) {
        let mut welcome_message = format!("ROR Text Editor -- version {}", VERSION);
        let width = self.terminal.size().width as usize;
        let len = welcome_message.len();
        let padding = width.saturating_sub(len) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1));
        welcome_message = format!("~{}{}", spaces, welcome_message);
        welcome_message.truncate(width);
        println!("{}\r", welcome_message);
    }
    fn draw_status_bar(&self) {
        let mut status;
        let width = self.terminal.size().width as usize;
        let modified_indicator = if self.document.is_dirty() {
            " (modified)"
        } else {
            ""
        };
        let mut file_name = "[No Name]".to_string();
        if let Some(name) = &self.document.file_name {
            file_name = name.clone();
            file_name.truncate(20);
        }
        status = format!(
            "{} - {} lines{}",
            file_name,
            self.document.len(),
            modified_indicator
        );

        let line_indicator = format!(
            "{}/{}",
            self.cursor_position.y.saturating_add(1),
            self.document.len()
        );
        let len = status.len() + line_indicator.len();
        if width > len {
            status.push_str(&" ".repeat(width - len));
        }
        status = format!("{}{}", status, line_indicator);
        status.truncate(width);
        Terminal::set_bg_color(STATUS_BG_COLOR);
        Terminal::set_fg_color(STATUS_FG_COLOR);
        println!("{}\r", status);
        Terminal::reset_fg_color();
        Terminal::reset_bg_color();
    }
    fn draw_message_bar(&self) {
        Terminal::clear_current_line();
        let message = &self.status_message;
        if Instant::now() - message.time < Duration::new(5, 0) {
            let mut text = message.text.clone();
            text.truncate(self.terminal.size().width as usize);
            print!("{}", text);
        }
    }
    fn scroll(&mut self) {
        let Position { x, y } = self.cursor_position;
        let width = self.terminal.size().width as usize;
        let height = self.terminal.size().height as usize;
        let mut offset = &mut self.offset;
        if y < offset.y {
            offset.y = y;
        } else if y >= offset.y.saturating_add(height) {
            offset.y = y.saturating_sub(height).saturating_add(1);
        }
        if x < offset.x {
            offset.x = x;
        } else if x >= offset.x.saturating_add(width) {
            offset.x = x.saturating_sub(width).saturating_add(1);
        }
    }
    fn prompt(&mut self, prompt: &str) -> Result<Option<String>, std::io::Error> {
        
        let mut result = String::new();
        loop {
            self.status_message = StatusMessage::from(format!("{}{}", prompt, result));
            self.refresh_screen()?;
            match Terminal::read_key()? {
                Key::Backspace => {
                    if !result.is_empty() {
                        result.truncate(result.len() - 1);
                    }
                }
                Key::Char('\n') => break,
                Key::Char(c) => {
                    if !c.is_control() {
                        result.push(c);
                    }
                }
                Key::Esc => {
                    result.truncate(0);
                    break;
                }
                _ => (),
            }
        }
        self.status_message = StatusMessage::from(String::new());
        if result.is_empty() {
            return Ok(None);
        }
        Ok(Some(result))
    }
}

struct StatusMessage {
    text: String,
    time: Instant,
}
impl StatusMessage {
    fn from(message: String) -> Self {
        Self {
            time: Instant::now(),
            text: message,
        }
    }
}

fn program_die(error : std::io::Error) {
    Terminal::clear_screen();            
    panic!("{}",error);
}

