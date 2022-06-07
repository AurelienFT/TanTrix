use std::fmt;
use std::sync::{Arc, RwLock};

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Color {
    YELLOW,
    RED,
    BLUE,
}

#[derive(Copy, Clone)]
pub enum LinePoints {
    START,
    END,
}

#[derive(Clone, Copy)]
pub struct Line {
    pub color: Color,
    pub sides: [u32; 2],
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Color: {:#?}", self.color).unwrap();
        writeln!(f, "Start: {}, End {}", self.sides[0], self.sides[1])
    }
}

impl fmt::Debug for Line {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Color: {:#?}", self.color).unwrap();
        writeln!(f, "Start: {}, End {}", self.sides[0], self.sides[1])
    }
}

impl Line {
    pub fn new(color: Color, sides: [u32; 2]) -> Line {
        Line { color, sides }
    }
}

#[derive(Clone)]
pub struct Piece {
    pub id: u32,
    pub color: Color,
    pub lines: [Line; 3],
    // Filled when resolved
    pub borders: [Option<Arc<RwLock<Piece>>>; 6],
    pub passed: bool,
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "ID: {}", self.id).unwrap();
        writeln!(f, "Color: {:#?}", self.color).unwrap();
        writeln!(f, "Lines: {:#?}", self.lines).unwrap();
        writeln!(f, "Border ids:").unwrap();
        for (pos, border) in self.borders.iter().enumerate() {
            if let Some(border_inner) = border {
                writeln!(f, "{}:{}", pos, border_inner.read().unwrap().id).unwrap();
            } else {
                writeln!(f, "{}:None", pos).unwrap();
            }
        }
        writeln!(f, "--------------------------------")
    }
}

impl fmt::Debug for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "ID: {}", self.id).unwrap();
        writeln!(f, "Color: {:#?}", self.color).unwrap();
        writeln!(f, "Lines: {:#?}", self.lines).unwrap();
        writeln!(f, "Border ids:").unwrap();
        for (pos, border) in self.borders.iter().enumerate() {
            if let Some(border_inner) = border {
                writeln!(f, "{}:{}", pos, border_inner.read().unwrap().id).unwrap();
            } else {
                writeln!(f, "{}:None", pos).unwrap();
            }
        }
        writeln!(f, "--------------------------------")
    }
}

impl Piece {
    pub fn new(id: u32, color: Color, lines: [Line; 3]) -> Piece {
        Piece {
            id,
            color,
            lines,
            borders: Default::default(),
            passed: false,
        }
    }

    pub fn add_border(&mut self, piece: Arc<RwLock<Piece>>, position: u32) {
        self.borders[position as usize] = Some(piece);
    }

    pub fn rotate_right(&mut self) {
        for line in &mut self.lines {
            if line.sides[0] == 5 {
                line.sides[0] = 0;
            } else {
                line.sides[0] += 1;
            }
            if line.sides[1] == 5 {
                line.sides[1] = 0;
            } else {
                line.sides[1] += 1;
            }
        }
    }

    pub fn rotate_left(&mut self) {
        for line in &mut self.lines {
            if line.sides[0] == 0 {
                line.sides[0] = 5;
            } else {
                line.sides[0] -= 1;
            }
            if line.sides[1] == 0 {
                line.sides[1] = 5;
            } else {
                line.sides[1] -= 1;
            }
        }
    }
}
