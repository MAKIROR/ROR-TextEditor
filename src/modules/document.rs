
use crate::Row;
use crate::Position;
use std::fs;
use std::io::{Error, Write};

#[derive(Default)]
pub struct Document {
    pub file_name: Option<String>,
    rows: Vec<Row>,
    dirty: bool,
}

impl Document {
    pub fn open(filename: &str) -> Result<Self, std::io::Error> {
        let contents = fs::read_to_string(filename)?;
        let mut rows = Vec::new();
        for value in contents.lines() {
            rows.push(Row::from(value));
        }
        Ok(Self {
            rows,
            file_name: Some(filename.to_string()),
            dirty: false,
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
        let new_row = self.rows.get_mut(at.y).unwrap().split(at.x);
        self.rows.insert(at.y + 1, new_row);
        
    }
    pub fn insert(&mut self, at: &Position, c: char) -> i32{
        if at.y > self.len() {
            return 0;
        }
        self.dirty = true;
        if c == '\n' {
            self.insert_line(at);
            return 1;
        }
        if at.y == self.len() {
            let mut row = Row::default();
            row.insert(0, c);
            self.rows.push(row);
            return 0;
        } else {
            let row = self.rows.get_mut(at.y).unwrap();
            row.insert(at.x, c);
            return 0;
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
            return 0;
        } else if at.y <= len && self.rows.get_mut(at.y).unwrap().len() == 0 {
            let row = self.rows.get_mut(at.y).unwrap();
            row.delete(at.x);
            return 1;
        } else {
            let row = self.rows.get_mut(at.y).unwrap();
            if at.x != 0 || at.y == 0 {
                row.delete(at.x -1);
            } else {
                row.delete(at.x);
            }
            return 0;
        }
    }
    pub fn save(&mut self) -> Result<(), Error> {
        if let Some(file_name) = &self.file_name {
            let mut file = fs::File::create(file_name)?;
            for row in &self.rows {
                file.write_all(row.as_bytes())?;
                file.write_all(b"\n")?;
            }
            self.dirty = false;
        }
        Ok(())
    }
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }
    pub fn find(&self, query: &str) -> Option<Vec<Position>> {
        let mut result: Vec<Position> = Vec::new();
        for (y, row) in self.rows.iter().enumerate() {
            if let Some(x) = row.find(query) {
                let value = Position { x, y };
                result.push(value);
            }
        }
        if result.len() > 0 {
            return Some(result);
        } else {
            None
        }
        
    }
}