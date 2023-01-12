use rand::{thread_rng, Rng, seq::SliceRandom};
use crate::piece::{PieceExport, Color, Piece, Line};

pub fn generate_game(nb_piece: u32) -> (Vec<PieceExport>, Color) {
    let mut rng = thread_rng();
    let color_index = rng.gen_range(0..3);
    let winning_color = match color_index {
        0 => Color::BLUE,
        1 => Color::YELLOW,
        2 => Color::RED,
        _ => panic!("Impossible color index"),
    };
    let mut pieces = Vec::new();
    let matching_borders: [(usize, usize); 6] = [(0, 3), (1, 4), (2, 5), (3, 0), (4, 1), (5, 2)];
    let mut last_border = None;
    for i in 0..nb_piece {
        let mut borders_left = vec![0, 1, 2, 3, 4, 5];
        let mut colors_left = vec![Color::BLUE, Color::YELLOW, Color::RED];
        colors_left.remove(color_index);
        // Draw the winning line
        let winning_line = if let Some(l_b) = last_border {
            let matching_border: (usize, usize) = matching_borders[l_b];
            let start_border = matching_border.1;
            let mut end_border = rng.gen_range(0..matching_borders.len());
            while end_border == start_border {
                end_border = rng.gen_range(0..matching_borders.len());
            }
            last_border = Some(end_border);
            borders_left.remove(start_border);
            borders_left.remove(end_border);
            Line::new(winning_color, [start_border as u32, end_border as u32])
        } else {
            let start_border = rng.gen_range(0..matching_borders.len());
            let mut end_border = rng.gen_range(0..matching_borders.len());
            while end_border == start_border {
                end_border = rng.gen_range(0..matching_borders.len());
            }
            last_border = Some(end_border);
            borders_left.remove(start_border);
            borders_left.remove(end_border);
            Line::new(winning_color, [start_border as u32, end_border as u32])
        };
        // Draw the two other lines
        let start_border_1 = *borders_left.choose(&mut rng).unwrap();
        let end_border_1 = matching_borders[start_border_1].1;
        let color = *colors_left.choose(&mut rng).unwrap();
        colors_left.remove(colors_left.iter().position(|c| *c == color).unwrap());
        borders_left.remove(borders_left.iter().position(|c| *c == start_border_1).unwrap());
        borders_left.remove(borders_left.iter().position(|c| *c == end_border_1).unwrap());
        let line_1 = Line::new(color, [start_border_1 as u32, end_border_1 as u32]);

        let start_border_2 = *borders_left.choose(&mut rng).unwrap();
        let end_border_2 = matching_borders[start_border_2].1;
        let color = *colors_left.choose(&mut rng).unwrap();
        colors_left.remove(colors_left.iter().position(|c| *c == color).unwrap());
        borders_left.remove(borders_left.iter().position(|c| *c == start_border_2).unwrap());
        borders_left.remove(borders_left.iter().position(|c| *c == end_border_2).unwrap());
        let line_2 = Line::new(color, [start_border_2 as u32, end_border_2 as u32]);
        pieces.push(Piece::new(i, winning_color, [
            winning_line,
            line_1,
            line_2
        ]).into());
    }
    (pieces, winning_color)
}

//TODO: Add test
mod tests {
    use super::generate_game;

    #[test]
    fn basic() {
        println!("{:#?}", generate_game(10));
    }
}