use crate::Row;
use crate::Position;
use std::fs;
use std::io::{Error, Write};
use crate::SearchDirection;
use crate::FileType;

#[derive(Default)]
pub struct Document {
    pub file_name: Option<String>,
    rows: Vec<Row>,
    dirty: bool,
    file_type: FileType,
}

impl Document {
    pub fn open(filename: &str) -> Result<Self, std::io::Error> {
        let contents = fs::read_to_string(filename)?;
        let mut rows = Vec::new();
        let file_type = FileType::from(filename);
        let mut start_with_comment = false;
        for value in contents.lines() {
            rows.push(Row::from(value));
        }
        Ok(Self {
            rows,
            file_name: Some(filename.to_string()),
            dirty: false,
            file_type,
        })
    }
    pub fn open_new_file(filename: &str) -> Result<Self, std::io::Error> {
        let mut rows = Vec::new();
        Ok(Self {
            rows,
            file_name: Some(filename.to_string()),
            dirty: false,
            file_type: FileType::default(),
        })
    }
    
    pub fn row(&self, index: usize) -> Option<&Row> {
        self.rows.get(index)
    }
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }
    pub fn len(&self) -> usize {
        self.rows.len()
    }
    fn insert_line(&mut self, at: &Position) {
        if at.y == self.len() {
            self.rows.push(Row::default());
            return;
        }
        #[allow(clippy::indexing_slicing)]
        let current_row = &mut self.rows[at.y];
        let new_row = current_row.split(at.x);
        #[allow(clippy::integer_arithmetic)]
        self.rows.insert(at.y + 1, new_row);
    }
    pub fn insert(&mut self, at: &Position, c: char) -> i32{
        if at.y > self.len() {
            return 0;
        }
        self.dirty = true;
        if c == '\n' {
            self.insert_line(at);
            self.unhighlight_rows(at.y);
            return 1;
        } else if at.y == self.rows.len() {
            let mut row = Row::default();
            row.insert(0, c);
            self.rows.push(row);
            self.unhighlight_rows(at.y);
            return 0;
        } else {
            let row = self.rows.get_mut(at.y).unwrap();
            row.insert(at.x, c);
            self.unhighlight_rows(at.y);
            return 0;
        }
    }
    fn unhighlight_rows(&mut self, start: usize) {            
        let start = start.saturating_sub(1);            
        for row in self.rows.iter_mut().skip(start) {            
            row.is_highlighted = false;            
        }            
    }
    pub fn delete(&mut self, at: &Position) -> i32 {
        let len = self.len();
        
        if at.y >= len {
            return 0;
        }
        if at.x == self.rows.get_mut(at.y).unwrap().len() && at.y < len - 1 {
            let next_row = self.rows.remove(at.y + 1);
            let row = self.rows.get_mut(at.y).unwrap();
            row.append(&next_row);
            self.unhighlight_rows(at.y);
            return 0;
        } else if at.y <= len && self.rows.get_mut(at.y).unwrap().len() == 0 {
            let row = self.rows.get_mut(at.y).unwrap();
            row.delete(at.x);
            self.unhighlight_rows(at.y);
            return 1;
        } else {
            let row = self.rows.get_mut(at.y).unwrap();
            if at.x != 0 || at.y == 0 {
                row.delete(at.x -1);
            } else {
                row.delete(at.x);
            }
            self.unhighlight_rows(at.y);
            return 0;
        }
    }
    pub fn delete_line(&mut self, at: &Position) -> i32 {
        self.rows.remove(at.y);
        if at.y == 0 {
            return 1;
        } else {
            return 0;
        }
    }
    pub fn save(&mut self) -> Result<(), Error> {
        if let Some(file_name) = &self.file_name {
            let file_path: Vec<&str> = file_name.split("/").collect();
            if file_path.len() > 1 {
                let mut dir = String::new();
                for i in 0..file_path.len() - 1 {
                    dir.push_str(file_path[i]);
                    dir.push_str("/");
                }
                std::fs::create_dir_all(dir).unwrap();
            }
            let mut file = fs::File::create(file_name)?;
            self.file_type = FileType::from(file_name);        
            for row in &mut self.rows {
                file.write_all(row.as_bytes())?;
                file.write_all(b"\n")?;
            }
            self.file_type = FileType::from(file_name);
            self.dirty = false;
        }
        Ok(())
    }
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }
    #[allow(clippy::indexing_slicing)]
    pub fn find(&self, query: &str, at: &Position, direction: SearchDirection) -> Option<Position> {
        if at.y >= self.rows.len() {
            return None;
        }
        let mut position = Position { x: at.x, y: at.y };

        let start = if direction == SearchDirection::Forward {
            at.y
        } else {
            0
        };
        let end = if direction == SearchDirection::Forward {
            self.rows.len()
        } else {
            at.y.saturating_add(1)
        };
        for _ in start..end {
            if let Some(row) = self.rows.get(position.y) {
                if let Some(x) = row.find(&query, position.x, direction) {
                    position.x = x;
                    return Some(position);
                }
                if direction == SearchDirection::Forward {
                    position.y = position.y.saturating_add(1);
                    position.x = 0;
                } else {
                    position.y = position.y.saturating_sub(1);
                    position.x = self.rows[position.y].len();
                }
            } else {
                return None;
            }
        }
        None
    }
    pub fn get_line(&self, at: &Position) -> Option<String> {
        let mut position = Position { x: at.x, y: at.y };
        if let Some(row) = self.rows.get(position.y) {
            let result = Some(row.get_row());
            return result;
        } else {
            return None;
        }
    }
    pub fn highlight(&mut self, word: &Option<String>, until: Option<usize>) {
        let mut start_with_comment = false;
        let until = if let Some(until) = until {
            if until.saturating_add(1) < self.rows.len() {
                until.saturating_add(1)
            } else {
                self.rows.len()
            }
        } else {
            self.rows.len()
        };
        #[allow(clippy::indexing_slicing)]
        for row in &mut self.rows[..until] {
            start_with_comment = row.highlight(
                &self.file_type.highlighting_options(),
                word,
                start_with_comment,
            );
        }
    }
    pub fn file_type(&self) -> String {
        self.file_type.name()
    }
}