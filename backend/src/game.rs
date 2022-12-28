use crate::piece::{Color, Line, LinePoints, Piece};
use std::sync::{Arc, RwLock};

fn get_color_line<'a>(piece: &'a Piece, color: &'a Color) -> Option<&'a Line> {
    piece.lines.iter().find(|line| line.color == *color)
}

// Refactor when if let chains are stable
fn check_solved(piece: &Arc<RwLock<Piece>>, color: Color, start_or_end: LinePoints) -> bool {
    let read_piece = piece.read().unwrap().clone();
    let line = get_color_line(&read_piece, &color).unwrap();
    let linked = &read_piece.borders[line.sides[start_or_end as usize] as usize];
    if let Some(linked_piece) = linked {
        let read_linked_piece = &linked_piece.read().unwrap().clone();
        let linked_line = get_color_line(read_linked_piece, &color).unwrap();
        if let Some(linked_face) =
            &read_linked_piece.borders[((line.sides[start_or_end as usize] + 3) % 6) as usize]
        {
            if linked_face.read().unwrap().id == read_piece.id {
                if read_piece.passed {
                    true
                } else {
                    {
                        let mut write_piece = piece.write().unwrap();
                        write_piece.passed = true;
                    }
                    if linked_line.sides[0] == (line.sides[start_or_end as usize] + 3) % 6 {
                        check_solved(linked_piece, color, LinePoints::END)
                    } else if linked_line.sides[1] == (line.sides[start_or_end as usize] + 3) % 6 {
                        check_solved(linked_piece, color, LinePoints::START)
                    } else {
                        false
                    }
                }
            } else {
                false
            }
        } else {
            false
        }
    } else {
        false
    }
}

pub fn check_solved_game(pieces: &[Arc<RwLock<Piece>>], color: Color) -> bool {
    if pieces.is_empty() {
        return false;
    }
    let start_piece = &pieces[0];
    if check_solved(start_piece, color, LinePoints::END) {
        for piece in pieces {
            if !piece.read().unwrap().passed {
                return false;
            }
        }
        true
    } else {
        false
    }

}

#[cfg(test)]
mod tests {
    use crate::game::check_solved_game;
    use crate::piece::{Color, Line, Piece};
    use std::sync::{Arc, RwLock};
    #[test]
    fn solved() {
        let pieces = [
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
        println!("test = {:#?}", pieces);
        assert!(check_solved_game(&pieces, Color::YELLOW));
    }
}
