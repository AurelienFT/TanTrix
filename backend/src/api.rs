use std::{time::SystemTime, sync::{Arc, RwLock}};
use chrono::{DateTime, Utc, Datelike}; // 0.4.15

use rusqlite::Connection;
use tiny_http::Request;
use crate::{piece::{PieceExport, Piece, Color}, game::check_solved_game, generation::generate_game};

pub fn handle_submit(request: &mut Request) -> String {
    let mut content = String::new();
    //TODO: Check if the game is the right one and not an invented
    request.as_reader().read_to_string(&mut content).unwrap();
    let pieces_export = serde_json::from_str::<Vec<PieceExport>>(&content).unwrap();
    let pieces: Vec<Arc<RwLock<Piece>>> = pieces_export.iter().map(|p| Arc::new(RwLock::new(Piece::from(p.clone())))).collect::<Vec<_>>();
    //TODO: Add borders
    //TODO: Get the right color
    check_solved_game(&pieces, Color::YELLOW).to_string()
}


// (Game, color) make a structure
pub fn handle_get_daily_game(request: &mut Request) -> (String, String) {
    let conn = Connection::open("./src/games.sqlite").unwrap();
    let today: DateTime<Utc> = SystemTime::now().into();
    let query = format!("SELECT * FROM daily_games WHERE date == ?");
    let mut statement = conn.prepare(&query).unwrap();
    let mut results = statement.query(&[&format!("{}:{}:{}", today.year(), today.month(), today.day())]).unwrap();
    if let Some(row) = results.next().unwrap() {
        (row.get::<_, String>(2).unwrap(), row.get::<_, String>(3).unwrap())
    } else {
        let query = format!("INSERT INTO daily_games (date, pieces, color) VALUES (?, ?, ?)");
        let mut statement = conn.prepare(&query).unwrap();
        let (pieces, color) = generate_game(10);
        let pieces_serialization = serde_json::to_string(&pieces).unwrap();
        statement.execute(&[&format!("{}:{}:{}", today.year(), today.month(), today.day()), &pieces_serialization, &color.to_string()]).unwrap();        
        (pieces_serialization, color.to_string())
    }
}

pub fn handle_unknown(_request: &mut Request) -> String {
    String::from("bad request")
}