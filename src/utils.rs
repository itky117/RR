#![allow(dead_code)]

use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub struct Span {
    pub start_byte: usize,
    pub end_byte: usize,
    pub start_line: u32,
    pub start_col: u32,
    pub end_line: u32,
    pub end_col: u32,
}

impl fmt::Debug for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}-{}:{}", self.start_line, self.start_col, self.end_line, self.end_col)
    }
}

impl Span {
    pub fn new(start_byte: usize, end_byte: usize, start_line: u32, start_col: u32, end_line: u32, end_col: u32) -> Self {
        Self { start_byte, end_byte, start_line, start_col, end_line, end_col }
    }
    
    pub fn merge(&self, other: Span) -> Span {
        if self.start_byte == 0 && self.end_byte == 0 && self.start_line == 0 { return other; }
        if other.start_byte == 0 && other.end_byte == 0 && other.start_line == 0 { return *self; }
        
        Span {
            start_byte: self.start_byte.min(other.start_byte),
            end_byte: self.end_byte.max(other.end_byte),
            start_line: self.start_line.min(other.start_line),
            start_col: if self.start_line < other.start_line { self.start_col } else { self.start_col.min(other.start_col) }, // Approximate
            end_line: self.end_line.max(other.end_line),
            end_col: if self.end_line > other.end_line { self.end_col } else { self.end_col.max(other.end_col) },
        }
    }
}
