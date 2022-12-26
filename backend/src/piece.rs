use std::fmt;
use std::sync::{Arc, RwLock};

use serde::{Serialize, Deserialize};

#[derive(PartialEq, Eq, Debug, Clone, Copy, Serialize, Deserialize)]
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

#[derive(Clone, Serialize, Deserialize)]
pub struct PieceExport {
    pub id: u32,
    pub color: Color,
    pub lines: [Line; 3],
    // Filled when resolved
    pub borders: [Option<u32>; 6],
    pub passed: bool,
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
            passed: value.passed,
        }
    }
}


// mod arc_rwlock_serde {
//     use serde::{Deserialize, Serialize};
//     use serde::de::{Deserializer, Visitor, SeqAccess};
//     use serde::ser::{Serializer, SerializeSeq};
//     use core::fmt;
//     use std::sync::{Arc, RwLock};

//     use super::Piece;

//     pub fn serialize<S>(val: &[Option<Arc<RwLock<Piece>>>; 6], s: S) -> Result<S::Ok, S::Error>
//         where S: Serializer,
//     {
//         let mut rows = s.serialize_seq(Some (6))?;
//         for elem in val {
//             if let Some(e) = elem {
//                 rows.serialize_element(&e.read().unwrap().id)?;
//             } else {
//                 rows.serialize_element(&None::<Piece>)?;
//             }
//         }
//         rows.end()
//     }

//     struct BordersDeserializer;

//     impl<'de> Visitor<'de> for BordersDeserializer {
//         type Value = [Option<Arc<RwLock<Piece>>>; 6];

//         fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
//             formatter.write_str("Borders sequence.")
//         }

//         fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
//         where
//             A: SeqAccess<'de>,
//         {
//             let mut pieces: [Option<Arc<RwLock<Piece>>>; 6] = Default::default();
//             let mut i = 0;
//             while i < 6 {
//                 if let Some(piece) = seq.next_element()? {
//                     pieces[i] = Some(Arc::new(RwLock::new(piece)));
//                 } else {
//                     pieces[i] = None;
//                 }
//                 i += 1;
//             }
//             Ok(pieces)
//         }
//     }

//     pub fn deserialize<'de, D>(d: D) -> Result<[Option<Arc<RwLock<Piece>>>; 6], D::Error>
//         where D: Deserializer<'de>,
//     {
//         d.deserialize_seq(BordersDeserializer)
//     }
// }

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

    use crate::{piece::{Piece, Color, Line, PieceExport}, game::check_solved_game};

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

        let pieces_export: Vec<PieceExport> = pieces.iter().map(|p| PieceExport::from(p.read().unwrap().clone())).collect();

        let json = serde_json::to_string_pretty(&pieces_export).unwrap();
        println!("{}", json);
    }
}