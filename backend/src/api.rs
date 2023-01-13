use chrono::{DateTime, Datelike, Utc};
use std::{
    sync::{Arc, RwLock},
    time::SystemTime,
}; // 0.4.15

use crate::{
    game::{check_solved_game, GameExport},
    generation::generate_game,
    piece::{Color, Piece, PieceExport},
};
use rusqlite::Connection;
use serde_json::json;
use tiny_http::Request;

pub fn handle_submit(request: &mut Request) -> String {
    let mut content = String::new();
    //TODO: Check if the game is the right one and not an invented
    request.as_reader().read_to_string(&mut content).unwrap();
    let pieces_export = serde_json::from_str::<Vec<PieceExport>>(&content).unwrap();
    let pieces: Vec<Arc<RwLock<Piece>>> = pieces_export
        .iter()
        .map(|p| Arc::new(RwLock::new(Piece::from(p.clone()))))
        .collect::<Vec<_>>();
    //TODO: Add borders
    //TODO: Get the right color
    check_solved_game(&pieces, Color::YELLOW).to_string()
}

// (Game, color) make a structure
pub fn handle_get_daily_game(request: &mut Request) -> String {
    let conn = Connection::open("./src/games.sqlite").unwrap();
    let today: DateTime<Utc> = SystemTime::now().into();
    let query = format!("SELECT * FROM daily_games WHERE date == ?");
    let mut statement = conn.prepare(&query).unwrap();
    let mut results = statement
        .query(&[&format!(
            "{}:{}:{}",
            today.year(),
            today.month(),
            today.day()
        )])
        .unwrap();
    if let Some(row) = results.next().unwrap() {
        row.get::<_, String>(2).unwrap()
    } else {
        let query = format!("INSERT INTO daily_games (date, game_data) VALUES (?, ?)");
        let mut statement = conn.prepare(&query).unwrap();
        let (pieces, color) = generate_game(10);
        let game_encoded = serde_json::to_string(&GameExport { pieces, color }).unwrap();
        statement
            .execute(&[
                &format!("{}:{}:{}", today.year(), today.month(), today.day()),
                &game_encoded,
            ])
            .unwrap();
        game_encoded
    }
}

pub fn handle_unknown(_request: &mut Request) -> String {
    String::from("bad request")
}
