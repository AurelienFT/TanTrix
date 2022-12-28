use std::{time::SystemTime, sync::{Arc, RwLock}};
use chrono::{DateTime, Utc, Datelike}; // 0.4.15

use rusqlite::Connection;
use tiny_http::Request;
use crate::{piece::{PieceExport, Piece, Color}, game::check_solved_game};

pub fn handle_submit(request: &mut Request) -> String {
    let mut content = String::new();
    //TODO: Check if the game is the right one and not an invented
    request.as_reader().read_to_string(&mut content).unwrap();
    let tiles_export = serde_json::from_str::<Vec<PieceExport>>(&content).unwrap();
    let tiles: Vec<Arc<RwLock<Piece>>> = tiles_export.iter().map(|p| Arc::new(RwLock::new(Piece::from(p.clone())))).collect::<Vec<_>>();
    //TODO: Add borders
    //TODO: Get the right color
    check_solved_game(&tiles, Color::YELLOW).to_string()
}

pub fn handle_get_daily_game(request: &mut Request) -> String {
    let conn = Connection::open("./src/games.sqlite").unwrap();
    let today: DateTime<Utc> = SystemTime::now().into();
    let query = format!("SELECT * FROM daily_games WHERE date == ?");
    let mut statement = conn.prepare(&query).unwrap();
    let mut results = statement.query(&[&format!("{}:{}:{}", today.year(), today.month(), today.day())]).unwrap();
    if let Some(row) = results.next().unwrap() {
        row.get::<_, String>(2).unwrap()
    } else {
        let query = format!("INSERT INTO daily_games (date, tiles) VALUES (?, ?)");
        let mut statement = conn.prepare(&query).unwrap();
        statement.execute(&[&format!("{}:{}:{}", today.year(), today.month(), today.day()), "[]"]).unwrap();
        //TODO: Add generation tiles
        String::from("[]")
    }
}

pub fn handle_unknown(_request: &mut Request) -> String {
    String::from("bad request")
}