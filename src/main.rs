use std::sync::{RwLock, Arc};

use crate::{game::check_solved_game, piece::{Piece, Color, Line}};

mod game;
mod piece;

fn main() {
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
