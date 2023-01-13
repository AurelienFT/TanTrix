use std::fmt;
use std::sync::{Arc, RwLock};

use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Color {
    YELLOW,
    RED,
    BLUE,
}

impl ToString for Color {
    fn to_string(&self) -> String {
        match self {
            Color::YELLOW => String::from("yellow"),
            Color::RED => String::from("red"),
            Color::BLUE => String::from("blue"),
        }
    }
}

#[derive(Copy, Clone)]
pub enum LinePoints {
    START,
    END,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
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

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PieceExport {
    pub id: u32,
    pub color: Color,
    pub lines: [Line; 3],
    // Filled when resolved
    pub borders: [Option<u32>; 6],
}

impl From<Piece> for PieceExport {
    fn from(value: Piece) -> Self {
        PieceExport {
            id: value.id,
            color: value.color,
            lines: value.lines,
            borders: [
                value.borders[0].as_ref().map(|p| p.read().unwrap().id),
                value.borders[1].as_ref().map(|p| p.read().unwrap().id),
                value.borders[2].as_ref().map(|p| p.read().unwrap().id),
                value.borders[3].as_ref().map(|p| p.read().unwrap().id),
                value.borders[4].as_ref().map(|p| p.read().unwrap().id),
                value.borders[5].as_ref().map(|p| p.read().unwrap().id),
            ],
        }
    }
}

impl From<PieceExport> for Piece {
    /// Becareful doesn't fill borders
    fn from(value: PieceExport) -> Self {
        Piece {
            id: value.id,
            color: value.color,
            lines: value.lines,
            borders: [None, None, None, None, None, None],
            passed: false,
        }
    }
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

mod tests {
    use std::sync::{Arc, RwLock};

    use crate::{
        game::check_solved_game,
        piece::{Color, Line, Piece, PieceExport},
    };

    #[test]
    fn test_serialize_json() {
        let pieces = vec![
            Arc::new(RwLock::new(Piece::new(
                1,
                Color::BLUE,
                [
                    Line::new(Color::BLUE, [0, 4]),
                    Line::new(Color::RED, [1, 5]),
                    Line::new(Color::YELLOW, [2, 3]),
                ],
            ))),
            Arc::new(RwLock::new(Piece::new(
                2,
                Color::BLUE,
                [
                    Line::new(Color::BLUE, [1, 4]),
                    Line::new(Color::RED, [2, 3]),
                    Line::new(Color::YELLOW, [0, 5]),
                ],
            ))),
            Arc::new(RwLock::new(Piece::new(
                3,
                Color::BLUE,
                [
                    Line::new(Color::BLUE, [2, 3]),
                    Line::new(Color::RED, [0, 1]),
                    Line::new(Color::YELLOW, [4, 5]),
                ],
            ))),
        ];

        pieces[1].write().unwrap().rotate_left();
        pieces[2].write().unwrap().rotate_right();
        pieces[2].write().unwrap().rotate_right();

        // Link 0-1
        pieces[0].write().unwrap().add_border(pieces[1].clone(), 2);
        pieces[1].write().unwrap().add_border(pieces[0].clone(), 5);

        // Link 1-2
        pieces[1].write().unwrap().add_border(pieces[2].clone(), 4);
        pieces[2].write().unwrap().add_border(pieces[1].clone(), 1);

        // Link 2-0
        pieces[0].write().unwrap().add_border(pieces[2].clone(), 3);
        pieces[2].write().unwrap().add_border(pieces[0].clone(), 0);
        //assert!(check_solved_game(&pieces, Color::YELLOW));

        let pieces_export: Vec<PieceExport> = pieces
            .iter()
            .map(|p| PieceExport::from(p.read().unwrap().clone()))
            .collect();

        let json = serde_json::to_string_pretty(&pieces_export).unwrap();
        println!("{}", json);
    }
}
