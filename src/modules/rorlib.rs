use crate::Terminal;
use termion::event::Key;
use crate::Document;
use crate::Row;
use termion::color;
use std::time::{Duration,Instant};
use std::env;
use regex::Regex;
use arboard::Clipboard;

#[derive(PartialEq, Copy, Clone)]
pub enum SearchDirection {
    Forward,
    Backward,
}

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
    cursor_position: Position,
    document: Document,
    offset: Position,
    status_message: StatusMessage,
    quit_times: u8,
}

#[derive(Default,Copy, Clone)]
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
        let mut initial_status = String::from("");
        let document = if args.len() > 1 {
            let file_name = &args[1];
            let doc = Document::open(&file_name);
            if doc.is_ok() {
                doc.unwrap()
            } else {
                initial_status = format!("Error: Could not open file: {}", file_name);
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
        let Position { y, mut x } = self.cursor_position;
        let width = if let Some(row) = self.document.row(y) {
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
                if let Some(command) = self.prompt("",|_, _, _|{}).unwrap_or(None) {
                    self.command_board(&command);
                }

            }
            Key::Ctrl('v') => {
                let mut clipboard = Clipboard::new().unwrap();
                let content = clipboard.get_text().unwrap();
                if content == "" {
                    self.status_message = StatusMessage::from(format!("Clipboard is empty"));
                } else {
                    let bytes = content.as_bytes();
                    for (i, &item) in bytes.iter().enumerate() {
                        let insert_result = self.document.insert(&self.cursor_position, item as char);
                        if insert_result == 1 {
                            self.move_cursor(Key::Down);
                        } else {
                            self.move_cursor(Key::Right);
                        }
                    }
                }
            }
            Key::Ctrl('x') => {
                
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
                        if command.len() != 1 {
                            self.status_message = StatusMessage::from(format!("Unqualified find command:{:?}.",command));
                        } else {
                            let old_position = self.cursor_position.clone();
                            let mut direction = SearchDirection::Forward;
                            let query = self
                                .prompt(
                                    "find something : ",
                                    |rorlib, key, query| {
                                        let mut moved = false;
                                        match key {
                                            Key::Right | Key::Down => {
                                                direction = SearchDirection::Forward;
                                                rorlib.move_cursor(Key::Right);
                                                moved = true
                                            }
                                            Key::Left | Key::Up=> direction=SearchDirection::Backward,
                                            _ => (),
                                        }
                      
                                        if let Some(position) = rorlib.
                                                document.
                                                find(&query, &rorlib.cursor_position, direction) {
                                                rorlib.cursor_position = position;
                                                rorlib.scroll();
                                        } else if moved {
                                            rorlib.move_cursor(Key::Left);
                                        }
                                    },
                                )
                                .unwrap_or(None);
                            if query.is_none() {
                                self.cursor_position = old_position;
                                self.scroll();
                            }
                      
                        }
                        break;
                    }
                    "replace" => {
                        //TODO                        
                    }
                    "rename" => {
                        if command.len() != 2 {
                            self.status_message = StatusMessage::from(format!("Unqualified rename command:{:?}.",command));
                        } else {
                            let r = Regex::new(r"[/\^:]+").unwrap();
                            if r.is_match(command[1]) {
                                self.status_message = StatusMessage::from(format!("Unqualified file name:{}.",command[1]));
                            } else {
                                self.document.file_name = Some(command[1].to_string());
                                self.status_message = StatusMessage::from(format!("Successfully changed the filename to:{}.",command[1]));
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
            let r = Regex::new(r"[/\^:]+").unwrap();
            let new_name = self.prompt("Save as: ", |_, _, _| {}).unwrap_or(None);
            if new_name.is_none() {
                self.status_message = StatusMessage::from("Save aborted.".to_string());
                return;
            } else if r.is_match(&new_name.as_ref().unwrap()){
                self.status_message = StatusMessage::from(format!("Unqualified file name"));
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
        let terminal_height = self.terminal.size().height as usize;
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
            Key::Left => {
                if x > 0 {
                    x -= 1;
                } else if y > 0 {
                    y -= 1;
                    if let Some(row) = self.document.row(y) {
                        x = row.len();
                    } else {
                        x = 0;
                    }
                }
            }
            Key::Right => {
                if x < width {
                    x += 1;
                } else if y < height {
                    y += 1;
                    x = 0;
                }
            }
            Key::PageUp => {
                y = if y > terminal_height {
                    y - terminal_height
                } else {
                    0
                }
            }
            Key::PageDown => {
                y = if y.saturating_add(terminal_height) < height {
                    y + terminal_height as usize
                } else {
                    height
                }
            }
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
    fn prompt<C>(&mut self, prompt: &str, mut callback: C) -> Result<Option<String>, std::io::Error>
    where
        C: FnMut(&mut Self, Key, &String),
    {
        let mut result = String::new();
        loop {
            self.status_message = StatusMessage::from(format!("{}{}", prompt, result));
            self.refresh_screen()?;
            let key = Terminal::read_key()?;
            match key {
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
            callback(self, key, &result);
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

